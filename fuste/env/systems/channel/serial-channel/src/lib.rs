#![no_std]

use fuste_channel::{ops::block_on_channel, ChannelError, ChannelSystemId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerialChannelError {
	CouldNotSerialize(u32),
	SerializedBufferTooSmall(u32),
	SerializedBufferMismatch((u32, u32)),
	TypeMismatch((u32, u32)),
	CouldNotDeserialize(u32),
	ChannelError(ChannelError),
}

/// Serial types must be able to serialize themselves into a buffer of a known size.
pub trait Serialize {
	fn try_to_bytes<const N: usize>(&self) -> Result<(usize, [u8; N]), SerialChannelError>;

	fn try_write_to_buffer<const N: usize>(
		&self,
		buffer: &mut [u8; N],
	) -> Result<u32, SerialChannelError> {
		let (len, bytes) = self.try_to_bytes::<N>()?;

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

	fn allocate_buffer<const N: usize>() -> [u8; N] {
		[0; N]
	}
}

pub trait Deserialize: Sized {
	fn try_from_bytes(bytes: &[u8]) -> Result<Self, SerialChannelError>;
}

/// A marker trait for types that can be serialized and deserialized.
pub trait SerialType: Serialize + Deserialize {}

impl<T> SerialType for T where T: Serialize + Deserialize {}

pub fn serial_channel_request<const N: usize, const M: usize, R: SerialType, W: SerialType>(
	system_id: ChannelSystemId,
	request: &R,
) -> Result<W, SerialChannelError> {
	let (len, bytes) = request.try_to_bytes::<N>()?;
	// use the short buffer to minimize memory reads by the system
	let short_read_buffer = &bytes[..len];

	let mut read_write_buffer = [0; M];

	let status = block_on_channel(system_id.clone(), short_read_buffer, &mut read_write_buffer)
		.map_err(SerialChannelError::ChannelError)?;
	let written_len = status.size() as usize;

	Ok(W::try_from_bytes(&read_write_buffer[..written_len])?)
}
