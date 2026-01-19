use fuste_serial_channel::{Deserialize, SerialChannelError, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Id<const I: usize>([u8; I]);

impl<const I: usize> Id<I> {
	pub fn new(bytes: [u8; I]) -> Self {
		Self(bytes)
	}

	pub const fn const_new() -> Self {
		Self([0; I])
	}

	pub fn to_le_bytes(&self) -> [u8; I] {
		self.0
	}

	pub fn copy_from_slice(slice: &[u8]) -> Self {
		let mut bytes = [0; I];
		bytes.copy_from_slice(slice);
		Self(bytes)
	}
}

impl<const I: usize> Serialize for Id<I> {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		buffer[..I].copy_from_slice(&self.0);
		Ok(I)
	}
}

impl<const I: usize> Deserialize for Id<I> {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		let id = Self(buffer[..I].try_into().unwrap());
		Ok((&buffer[I..], id))
	}
}
