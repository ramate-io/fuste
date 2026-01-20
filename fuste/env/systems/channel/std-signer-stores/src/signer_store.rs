use crate::signer_index::TransactionSignerIndex;
use core::any::type_name;
use fuste_channel::ChannelSystemId;
use fuste_serial_channel::{
	serial_channel_request, Deserialize, Empty, SerialChannelError, SerialType, Serialize,
};

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
}

impl<const N: usize, const P: usize, const K: usize, const T: usize, const B: usize>
	SignerStore<N, P, K, T, B>
{
	pub fn new(
		signer_index: TransactionSignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
		type_bytes: [u8; TYPE_NAME_BYTES],
		bytes: [u8; B],
	) -> Self {
		Self { signer_index, type_bytes, bytes }
	}

	/// Stores the raw signer store to the channel.
	pub fn store<const RSIZE: usize>(
		&self,
		system_id: ChannelSystemId,
	) -> Result<(), SerialChannelError> {
		serial_channel_request::<RSIZE, 0, Self, Empty>(system_id, self)?;
		Ok(())
	}
}

impl<const N: usize, const P: usize, const K: usize, const T: usize, const B: usize> Serialize
	for SignerStore<N, P, K, T, B>
{
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		if buffer.len() < K * (N + P) + T + B {
			return Err(SerialChannelError::SerializedBufferTooSmall((T + B) as u32));
		}

		let mut written_len = self.signer_index.try_write_to_buffer(buffer)?;
		buffer[written_len..written_len + T].copy_from_slice(&self.type_bytes);
		written_len += T;
		buffer[written_len..written_len + B].copy_from_slice(&self.bytes);
		Ok(written_len + B)
	}
}

impl<const N: usize, const P: usize, const K: usize, const T: usize, const B: usize> Deserialize
	for SignerStore<N, P, K, T, B>
{
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		let (remaining_buffer, signer_index) =
			TransactionSignerIndex::<N, P, K>::try_from_bytes_with_remaining_buffer(buffer)?;

		let mut type_bytes = [0; T];
		type_bytes.copy_from_slice(&remaining_buffer[..T]);
		let remaining_buffer = &remaining_buffer[T..];

		let mut bytes = [0; B];
		bytes.copy_from_slice(&remaining_buffer[..B]);
		let remaining_buffer = &remaining_buffer[B..];

		Ok((remaining_buffer, Self { signer_index, type_bytes, bytes }))
	}
}

#[derive(Debug, Clone)]
pub struct TypedSignerStore<const N: usize, const P: usize, const K: usize, T: SerialType> {
	signer_index: TransactionSignerIndex<N, P, K>,
	value: T,
}

impl<const N: usize, const P: usize, const K: usize, T: SerialType> TypedSignerStore<N, P, K, T> {
	pub fn new(signer_index: TransactionSignerIndex<N, P, K>, value: T) -> Self {
		Self { signer_index, value }
	}

	pub fn store<const TSIZE: usize, const B: usize, const RSIZE: usize>(
		&self,
		system_id: ChannelSystemId,
	) -> Result<(), SerialChannelError> {
		let signer_store = SignerStore::<N, P, K, TSIZE, B>::try_from(self)?;
		signer_store.store::<RSIZE>(system_id)?;
		Ok(())
	}
}

impl<
		const N: usize,
		const P: usize,
		const K: usize,
		const TSIZE: usize,
		const B: usize,
		T: SerialType,
	> TryFrom<&TypedSignerStore<N, P, K, T>> for SignerStore<N, P, K, TSIZE, B>
{
	type Error = SerialChannelError;

	fn try_from(value: &TypedSignerStore<N, P, K, T>) -> Result<Self, Self::Error> {
		let name = type_name::<T>();
		let name_bytes = name.as_bytes();

		if name_bytes.len() > TSIZE {
			return Err(SerialChannelError::SerializedBufferTooSmall(name_bytes.len() as u32));
		}

		let mut type_bytes = [0u8; TSIZE];
		type_bytes[..name_bytes.len()].copy_from_slice(name_bytes);

		let mut value_bytes = [0; B];
		value.value.try_write_to_buffer(&mut value_bytes)?;

		Ok(Self { signer_index: value.signer_index.clone(), type_bytes, bytes: value_bytes })
	}
}
