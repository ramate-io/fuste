use crate::signer_index::TransactionSignerIndex;
use core::any::type_name;
use fuste_channel::ChannelSystemId;
use fuste_serial_channel::{
	serial_channel_request, Bytes, Deserialize, SerialChannelError, SerialType, Serialize,
};

#[derive(Debug, Clone)]
pub struct SignerLoad<
	const ADDRESS_BYTES: usize,
	const PUBLIC_KEY_BYTES: usize,
	const SIGNER_COUNT: usize,
	const TYPE_NAME_BYTES: usize,
	const VALUE_BYTES: usize,
> {
	signer_index: TransactionSignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
	type_bytes: [u8; TYPE_NAME_BYTES],
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
	> SignerLoad<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES>
{
	pub fn new(
		signer_index: TransactionSignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
		type_bytes: [u8; TYPE_NAME_BYTES],
	) -> Self {
		Self { signer_index, type_bytes }
	}

	/// Loads the raw signer load to the channel.
	pub fn load<const RSIZE: usize>(
		&self,
		system_id: ChannelSystemId,
	) -> Result<Bytes<VALUE_BYTES>, SerialChannelError> {
		let bytes = serial_channel_request::<RSIZE, 0, Self, Bytes<VALUE_BYTES>>(system_id, self)?;
		Ok(bytes)
	}
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
	> Serialize
	for SignerLoad<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES>
{
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		if buffer.len()
			< SIGNER_COUNT * (ADDRESS_BYTES + PUBLIC_KEY_BYTES) + TYPE_NAME_BYTES + VALUE_BYTES
		{
			return Err(SerialChannelError::SerializedBufferTooSmall(
				(TYPE_NAME_BYTES + VALUE_BYTES) as u32,
			));
		}

		let written_len = self.signer_index.try_write_to_buffer(buffer)?;
		buffer[written_len..written_len + TYPE_NAME_BYTES].copy_from_slice(&self.type_bytes);
		Ok(written_len + TYPE_NAME_BYTES)
	}
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
	> Deserialize
	for SignerLoad<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES>
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

		Ok((remaining_buffer, Self { signer_index, type_bytes }))
	}
}

#[derive(Debug, Clone)]
pub struct TypedSignerLoad<
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
	> TypedSignerLoad<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, T>
{
	pub fn new(
		signer_index: TransactionSignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
		value: T,
	) -> Self {
		Self { signer_index, value }
	}

	pub fn load<const TYPE_NAME_BYTES: usize, const VALUE_BYTES: usize, const RSIZE: usize>(
		&self,
		system_id: ChannelSystemId,
	) -> Result<T, SerialChannelError> {
		let signer_load = SignerLoad::<
			ADDRESS_BYTES,
			PUBLIC_KEY_BYTES,
			SIGNER_COUNT,
			TYPE_NAME_BYTES,
			VALUE_BYTES,
		>::try_from(self)?;
		let bytes = signer_load.load::<RSIZE>(system_id)?;
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
	> TryFrom<&TypedSignerLoad<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, T>>
	for SignerLoad<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES>
{
	type Error = SerialChannelError;

	fn try_from(
		value: &TypedSignerLoad<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, T>,
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

		Ok(Self { signer_index: value.signer_index.clone(), type_bytes })
	}
}
