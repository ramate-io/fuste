#![no_std]

use fuste_channel::{ops::block_on_channel, ChannelError, ChannelSystemId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerialChannelError {
	CouldNotSerialize(u32),
	SerializedBufferTooSmall(u32),
	SerializedBufferMismatch((u32, u32)),
	SchemeMismatch([u8; 32], [u8; 32]),
	CouldNotDeserialize(u32),
	ChannelError(ChannelError),
}

/// Serial types must be able to serialize themselves into a buffer of a known size.
pub trait Serialize {
	fn try_to_bytes<const N: usize>(&self) -> Result<(usize, [u8; N]), SerialChannelError> {
		let mut buffer = [0; N];
		let len = self.try_write_to_buffer(&mut buffer)?;
		Ok((len, buffer))
	}

	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError>;
}

pub trait Deserialize: Sized {
	fn try_from_bytes(bytes: &[u8]) -> Result<Self, SerialChannelError> {
		let (_remaining_buffer, value) = Self::try_from_bytes_with_remaining_buffer(bytes)?;
		Ok(value)
	}

	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError>;
}

/// A marker trait for types that can be serialized and deserialized.
pub trait SerialType: Serialize + Deserialize {}

impl<T> SerialType for T where T: Serialize + Deserialize {}

pub fn serial_channel_request<
	const RSIZE: usize,
	const WSIZE: usize,
	R: SerialType,
	W: SerialType,
>(
	system_id: ChannelSystemId,
	request: &R,
) -> Result<W, SerialChannelError> {
	let (len, bytes) = request.try_to_bytes::<RSIZE>()?;
	// use the short buffer to minimize memory reads by the system
	let short_read_buffer = &bytes[..len];

	let mut read_write_buffer = [0; WSIZE];

	let status = block_on_channel(system_id.clone(), short_read_buffer, &mut read_write_buffer)
		.map_err(SerialChannelError::ChannelError)?;
	let written_len = status.size() as usize;

	Ok(W::try_from_bytes(&read_write_buffer[..written_len])?)
}

pub struct Empty;

impl Serialize for Empty {
	fn try_write_to_buffer(&self, _buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		Ok(0)
	}
}

impl Deserialize for Empty {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		Ok((buffer, Self))
	}
}

pub struct Bytes<const N: usize>(pub [u8; N]);

impl<const N: usize> Serialize for Bytes<N> {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		buffer.copy_from_slice(&self.0);
		Ok(self.0.len())
	}
}

impl<const N: usize> Deserialize for Bytes<N> {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		Ok((buffer, Self(buffer.try_into().unwrap())))
	}
}
