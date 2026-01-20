use crate::signer_index::TransactionSignerIndex;
use core::any::type_name;
use fuste_channel::ChannelSystemId;
use fuste_serial_channel::{
	serial_channel_request, Bytes, Deserialize, Empty, SerialChannelError, SerialType, Serialize,
};

/// Result of executing a SignerStore operation.
pub enum ExecuteResult<const VALUE_BYTES: usize> {
	Store(()),
	Load(Bytes<VALUE_BYTES>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Op(u8);

impl Op {
	pub const STORE: Op = Op(1 << 0);
	pub const LOAD: Op = Op(1 << 1);

	pub const fn contains(self, other: Op) -> bool {
		(self.0 & other.0) != 0
	}
}

impl core::ops::BitOr for Op {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		Op(self.0 | rhs.0)
	}
}

impl core::ops::BitOrAssign for Op {
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0;
	}
}

#[derive(Debug, Clone)]
pub struct SignerStore<
	const ADDRESS_BYTES: usize,
	const PUBLIC_KEY_BYTES: usize,
	const SIGNER_COUNT: usize,
	const TYPE_NAME_BYTES: usize,
	const VALUE_BYTES: usize,
> {
	signer_index: TransactionSignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
	type_bytes: [u8; TYPE_NAME_BYTES],
	bytes: [u8; VALUE_BYTES],
	op: Op,
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
	> SignerStore<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES>
{
	pub fn new(
		signer_index: TransactionSignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
		type_bytes: [u8; TYPE_NAME_BYTES],
		bytes: [u8; VALUE_BYTES],
		op: Op,
	) -> Self {
		Self { signer_index, type_bytes, bytes, op }
	}

	pub fn signer_index(
		&self,
	) -> &TransactionSignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT> {
		&self.signer_index
	}

	pub fn type_bytes(&self) -> &[u8; TYPE_NAME_BYTES] {
		&self.type_bytes
	}

	pub fn bytes(&self) -> &[u8; VALUE_BYTES] {
		&self.bytes
	}

	/// Execute the operation based on the Op field.
	///
	/// - When `Op::STORE` is set: stores to the channel using Empty with WSIZE 0
	/// - When `Op::LOAD` is set: loads from the channel using Bytes with WSIZE VALUE_BYTES
	pub fn execute<const RSIZE: usize>(
		&self,
		system_id: ChannelSystemId,
	) -> Result<ExecuteResult<VALUE_BYTES>, SerialChannelError> {
		if self.op.contains(Op::LOAD) {
			// When Op::LOAD is set, use Bytes with WSIZE VALUE_BYTES
			let bytes = serial_channel_request::<RSIZE, VALUE_BYTES, Self, Bytes<VALUE_BYTES>>(
				system_id, self,
			)?;
			Ok(ExecuteResult::Load(bytes))
		} else {
			// When Op::LOAD is not set (Op::STORE), use Empty with WSIZE 0
			serial_channel_request::<RSIZE, 0, Self, Empty>(system_id, self)?;
			Ok(ExecuteResult::Store(()))
		}
	}
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
	> Serialize
	for SignerStore<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES>
{
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		let required_len = if self.op.contains(Op::STORE) {
			SIGNER_COUNT * (ADDRESS_BYTES + PUBLIC_KEY_BYTES) + TYPE_NAME_BYTES + VALUE_BYTES
		} else {
			SIGNER_COUNT * (ADDRESS_BYTES + PUBLIC_KEY_BYTES) + TYPE_NAME_BYTES
		};

		if buffer.len() < required_len {
			return Err(SerialChannelError::SerializedBufferTooSmall(
				(required_len - SIGNER_COUNT * (ADDRESS_BYTES + PUBLIC_KEY_BYTES)) as u32,
			));
		}

		let mut written_len = self.signer_index.try_write_to_buffer(buffer)?;
		buffer[written_len..written_len + TYPE_NAME_BYTES].copy_from_slice(&self.type_bytes);
		written_len += TYPE_NAME_BYTES;

		if self.op.contains(Op::STORE) {
			buffer[written_len..written_len + VALUE_BYTES].copy_from_slice(&self.bytes);
			written_len += VALUE_BYTES;
		}

		Ok(written_len)
	}
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
	> Deserialize
	for SignerStore<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES>
{
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		let (remaining_buffer, signer_index) = TransactionSignerIndex::<
			ADDRESS_BYTES,
			PUBLIC_KEY_BYTES,
			SIGNER_COUNT,
		>::try_from_bytes_with_remaining_buffer(buffer)?;

		let mut type_bytes = [0; TYPE_NAME_BYTES];
		type_bytes.copy_from_slice(&remaining_buffer[..TYPE_NAME_BYTES]);
		let remaining_buffer = &remaining_buffer[TYPE_NAME_BYTES..];

		// When Op::STORE is not set (i.e., Op::LOAD), skip reading value bytes and use zeros
		// We need to know the op, but since we can't infer it, we'll read bytes if available
		// In practice, the caller should know the op based on the operation being performed
		let (remaining_buffer, bytes, op) = if remaining_buffer.len() >= VALUE_BYTES {
			let mut bytes = [0; VALUE_BYTES];
			bytes.copy_from_slice(&remaining_buffer[..VALUE_BYTES]);
			let remaining_buffer = &remaining_buffer[VALUE_BYTES..];
			(remaining_buffer, bytes, Op::STORE)
		} else {
			// Not enough bytes for value, treat as Load operation with zeros
			let bytes = [0; VALUE_BYTES];
			(remaining_buffer, bytes, Op::LOAD)
		};

		Ok((remaining_buffer, Self { signer_index, type_bytes, bytes, op }))
	}
}

#[derive(Debug, Clone)]
pub struct TypedSignerStore<
	const ADDRESS_BYTES: usize,
	const PUBLIC_KEY_BYTES: usize,
	const SIGNER_COUNT: usize,
	T: SerialType,
> {
	signer_index: TransactionSignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
	value: T,
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		T: SerialType,
	> TypedSignerStore<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, T>
{
	pub fn new(
		signer_index: TransactionSignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
		value: T,
	) -> Self {
		Self { signer_index, value }
	}

	/// Store the value to the channel.
	///
	/// Constructs a SignerStore with Op::STORE and executes the operation.
	/// The value is serialized and stored along with the signer index and type information.
	pub fn store<const TYPE_NAME_BYTES: usize, const VALUE_BYTES: usize, const RSIZE: usize>(
		&self,
		system_id: ChannelSystemId,
	) -> Result<(), SerialChannelError> {
		// Construct SignerStore with Op::STORE
		let signer_store = SignerStore::<
			ADDRESS_BYTES,
			PUBLIC_KEY_BYTES,
			SIGNER_COUNT,
			TYPE_NAME_BYTES,
			VALUE_BYTES,
		>::try_from(self)?;

		// Execute with Op::STORE (uses Empty with WSIZE 0)
		match signer_store.execute::<RSIZE>(system_id)? {
			ExecuteResult::Store(()) => Ok(()),
			ExecuteResult::Load(_) => {
				// This shouldn't happen when Op::STORE is set
				Err(SerialChannelError::SerializedBufferTooSmall(0))
			}
		}
	}

	/// Load the value from the channel.
	///
	/// Constructs a SignerStore with Op::LOAD and executes the operation.
	/// The value is deserialized from the response buffer.
	pub fn load<const TYPE_NAME_BYTES: usize, const VALUE_BYTES: usize, const RSIZE: usize>(
		&self,
		system_id: ChannelSystemId,
	) -> Result<T, SerialChannelError> {
		// Prepare type name bytes
		let name = type_name::<T>();
		let name_bytes = name.as_bytes();

		if name_bytes.len() > TYPE_NAME_BYTES {
			return Err(SerialChannelError::SerializedBufferTooSmall(name_bytes.len() as u32));
		}

		let mut type_bytes = [0u8; TYPE_NAME_BYTES];
		type_bytes[..name_bytes.len()].copy_from_slice(name_bytes);

		// Construct SignerStore with Op::LOAD and zero bytes
		let signer_store =
			SignerStore::<
				ADDRESS_BYTES,
				PUBLIC_KEY_BYTES,
				SIGNER_COUNT,
				TYPE_NAME_BYTES,
				VALUE_BYTES,
			>::new(self.signer_index.clone(), type_bytes, [0; VALUE_BYTES], Op::LOAD);

		// Execute with Op::LOAD (uses Bytes with WSIZE VALUE_BYTES)
		let bytes = match signer_store.execute::<RSIZE>(system_id)? {
			ExecuteResult::Load(bytes) => bytes,
			ExecuteResult::Store(_) => {
				// This shouldn't happen when Op::LOAD is set
				return Err(SerialChannelError::SerializedBufferTooSmall(0));
			}
		};

		// Deserialize from the write buffer
		let (_remaining_buffer, value) = T::try_from_bytes_with_remaining_buffer(&bytes.0)?;
		Ok(value)
	}
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
		T: SerialType,
	> TryFrom<&TypedSignerStore<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, T>>
	for SignerStore<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES>
{
	type Error = SerialChannelError;

	fn try_from(
		value: &TypedSignerStore<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, T>,
	) -> Result<Self, Self::Error> {
		let name = type_name::<T>();
		let name_bytes = name.as_bytes();

		if name_bytes.len() > TYPE_NAME_BYTES {
			return Err(SerialChannelError::SerializedBufferTooSmall(name_bytes.len() as u32));
		}

		let mut type_bytes = [0u8; TYPE_NAME_BYTES];
		type_bytes[..name_bytes.len()].copy_from_slice(name_bytes);

		let mut value_bytes = [0; VALUE_BYTES];
		value.value.try_write_to_buffer(&mut value_bytes)?;

		Ok(Self {
			signer_index: value.signer_index.clone(),
			type_bytes,
			bytes: value_bytes,
			op: Op::STORE,
		})
	}
}
