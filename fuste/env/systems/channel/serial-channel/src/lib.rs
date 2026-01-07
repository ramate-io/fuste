#![no_std]

use fuste_channel::{ops::block_on_channel, ChannelError, ChannelSystemId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialChannelId(u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerialChannelError {
	CouldNotSerialize(u32),
	SerializedBufferTooSmall(u32),
	SerializedBufferMismatch((u32, u32)),
	CouldNotDeserialize(u32),
	ChannelError(ChannelError),
}

/// Serial types must be able to serialize themselves into a buffer of a known size.
pub trait Serialize<const N: usize> {
	fn try_to_bytes(&self) -> Result<(usize, [u8; N]), SerialChannelError>;

	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<u32, SerialChannelError> {
		let (len, bytes) = self.try_to_bytes()?;

		// Somehow the writer reported a larger buffer than the actual bytes associated with the type.
		if len > bytes.len() {
			return Err(SerialChannelError::SerializedBufferMismatch((
				len as u32,
				bytes.len() as u32,
			)));
		}

		// The buffer returned is larger than the available buffer.
		if len > buffer.len() {
			return Err(SerialChannelError::SerializedBufferTooSmall(bytes.len() as u32));
		}

		// Copy the bytes into the buffer.
		buffer[..len].copy_from_slice(&bytes[..len]);

		// Return the number of bytes written.
		Ok(len as u32)
	}

	fn allocate_buffer() -> [u8; N] {
		[0; N]
	}
}

pub trait Deserialize: Sized {
	fn try_from_bytes(bytes: &[u8]) -> Result<Self, SerialChannelError>;
}

/// A marker trait for types that can be serialized and deserialized.
pub trait SerialType<const N: usize>: Serialize<N> + Deserialize {}

impl<const N: usize, T> SerialType<N> for T where T: Serialize<N> + Deserialize {}

pub fn serial_channel_request<
	const N: usize,
	const M: usize,
	R: SerialType<N>,
	W: SerialType<M>,
>(
	system_id: ChannelSystemId,
	request: &R,
) -> Result<W, SerialChannelError> {
	let (len, bytes) = request.try_to_bytes()?;
	// use the short buffer to minimize memory reads by the system
	let short_read_buffer = &bytes[..len];

	let mut read_write_buffer = [0; M];

	let status = block_on_channel(system_id.clone(), short_read_buffer, &mut read_write_buffer)
		.map_err(SerialChannelError::ChannelError)?;
	let written_len = status.size() as usize;

	Ok(W::try_from_bytes(&read_write_buffer[..written_len])?)
}
