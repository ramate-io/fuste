use crate::signer_index::TransactionSignerIndex;
use core::any::type_name;
use fuste_channel::ChannelSystemId;
use fuste_serial_channel::{
	serial_channel_request, Bytes, Deserialize, SerialChannelError, SerialType, Serialize,
};

#[derive(Debug, Clone)]
pub struct SignerLoad<
	const N: usize,
	const P: usize,
	const K: usize,
	const T: usize,
	const B: usize,
> {
	signer_index: TransactionSignerIndex<N, P, K>,
	type_bytes: [u8; T],
}

impl<const N: usize, const P: usize, const K: usize, const T: usize, const B: usize>
	SignerLoad<N, P, K, T, B>
{
	pub fn new(signer_index: TransactionSignerIndex<N, P, K>, type_bytes: [u8; T]) -> Self {
		Self { signer_index, type_bytes }
	}

	/// Loads the raw signer load to the channel.
	pub fn load<const RSIZE: usize>(
		&self,
		system_id: ChannelSystemId,
	) -> Result<Bytes<B>, SerialChannelError> {
		let bytes = serial_channel_request::<RSIZE, 0, Self, Bytes<B>>(system_id, self)?;
		Ok(bytes)
	}
}

impl<const N: usize, const P: usize, const K: usize, const T: usize, const B: usize> Serialize
	for SignerLoad<N, P, K, T, B>
{
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		if buffer.len() < K * (N + P) + T + B {
			return Err(SerialChannelError::SerializedBufferTooSmall((T + B) as u32));
		}

		let written_len = self.signer_index.try_write_to_buffer(buffer)?;
		buffer[written_len..written_len + T].copy_from_slice(&self.type_bytes);
		Ok(written_len + T)
	}
}

impl<const N: usize, const P: usize, const K: usize, const T: usize, const B: usize> Deserialize
	for SignerLoad<N, P, K, T, B>
{
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		let (remaining_buffer, signer_index) =
			TransactionSignerIndex::<N, P, K>::try_from_bytes_with_remaining_buffer(buffer)?;

		let mut type_bytes = [0; T];
		type_bytes.copy_from_slice(&remaining_buffer[..T]);
		let remaining_buffer = &remaining_buffer[T..];

		Ok((remaining_buffer, Self { signer_index, type_bytes }))
	}
}

#[derive(Debug, Clone)]
pub struct TypedSignerLoad<const N: usize, const P: usize, const K: usize, T: SerialType> {
	signer_index: TransactionSignerIndex<N, P, K>,
	value: T,
}

impl<const N: usize, const P: usize, const K: usize, T: SerialType> TypedSignerLoad<N, P, K, T> {
	pub fn new(signer_index: TransactionSignerIndex<N, P, K>, value: T) -> Self {
		Self { signer_index, value }
	}

	pub fn load<const TSIZE: usize, const B: usize, const RSIZE: usize>(
		&self,
		system_id: ChannelSystemId,
	) -> Result<T, SerialChannelError> {
		let signer_load = SignerLoad::<N, P, K, TSIZE, B>::try_from(self)?;
		let bytes = signer_load.load::<RSIZE>(system_id)?;
		let (_remaining_buffer, value) = T::try_from_bytes_with_remaining_buffer(&bytes.0)?;
		Ok(value)
	}
}

impl<
		const N: usize,
		const P: usize,
		const K: usize,
		const TSIZE: usize,
		const B: usize,
		T: SerialType,
	> TryFrom<&TypedSignerLoad<N, P, K, T>> for SignerLoad<N, P, K, TSIZE, B>
{
	type Error = SerialChannelError;

	fn try_from(value: &TypedSignerLoad<N, P, K, T>) -> Result<Self, Self::Error> {
		let name = type_name::<T>();
		let name_bytes = name.as_bytes();

		if name_bytes.len() > TSIZE {
			return Err(SerialChannelError::SerializedBufferTooSmall(name_bytes.len() as u32));
		}

		let mut type_bytes = [0u8; TSIZE];
		type_bytes[..name_bytes.len()].copy_from_slice(name_bytes);

		let mut value_bytes = [0; B];
		value.value.try_write_to_buffer(&mut value_bytes)?;

		Ok(Self { signer_index: value.signer_index.clone(), type_bytes })
	}
}
