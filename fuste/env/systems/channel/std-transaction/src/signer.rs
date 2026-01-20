use crate::TransactionData;
use fuste_serial_channel::{Deserialize, SerialChannelError, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddressBytes<const N: usize>([u8; N]);

impl<const N: usize> AddressBytes<N> {
	const DEFAULT_ADDRESS_BYTES: Self = Self([0; N]);

	pub fn new(bytes: [u8; N]) -> Self {
		Self(bytes)
	}
}

impl<const N: usize> Serialize for AddressBytes<N> {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		if buffer.len() < N {
			return Err(SerialChannelError::SerializedBufferTooSmall(N as u32));
		}

		buffer[..N].copy_from_slice(&self.0);
		Ok(N)
	}
}

impl<const N: usize> Deserialize for AddressBytes<N> {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		if buffer.len() < N {
			return Err(SerialChannelError::SerializedBufferTooSmall(N as u32));
		}

		let mut bytes = [0; N];
		bytes.copy_from_slice(&buffer[..N]);

		Ok((&buffer[N..], Self(bytes)))
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublicKeyBytes<const N: usize>([u8; N]);

impl<const N: usize> PublicKeyBytes<N> {
	const DEFAULT_PUBLIC_KEY_BYTES: Self = Self([0; N]);

	pub fn new(bytes: [u8; N]) -> Self {
		Self(bytes)
	}
}

impl<const N: usize> Serialize for PublicKeyBytes<N> {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		if buffer.len() < N {
			return Err(SerialChannelError::SerializedBufferTooSmall(N as u32));
		}

		buffer[..N].copy_from_slice(&self.0);
		Ok(N)
	}
}

impl<const N: usize> Deserialize for PublicKeyBytes<N> {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		if buffer.len() < N {
			return Err(SerialChannelError::SerializedBufferTooSmall(N as u32));
		}

		let mut bytes = [0; N];
		bytes.copy_from_slice(&buffer[..N]);

		Ok((&buffer[N..], Self(bytes)))
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransactionSigner<const A: usize, const P: usize> {
	pub address_bytes: AddressBytes<A>,
	pub private_key_bytes: PublicKeyBytes<P>,
}

impl<const A: usize, const P: usize> TransactionSigner<A, P> {
	pub const DEFAULT_SIGNER: Self = Self {
		address_bytes: AddressBytes::<A>::DEFAULT_ADDRESS_BYTES,
		private_key_bytes: PublicKeyBytes::<P>::DEFAULT_PUBLIC_KEY_BYTES,
	};

	pub fn new(address_bytes: AddressBytes<A>, private_key_bytes: PublicKeyBytes<P>) -> Self {
		Self { address_bytes, private_key_bytes }
	}
}

impl<const A: usize, const P: usize> Serialize for TransactionSigner<A, P> {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		let mut written_len = self.address_bytes.try_write_to_buffer(buffer)?;
		written_len += self.private_key_bytes.try_write_to_buffer(&mut buffer[written_len..])?;
		Ok(written_len)
	}
}

impl<const A: usize, const P: usize> Deserialize for TransactionSigner<A, P> {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		let (remaining_buffer, address_bytes) =
			AddressBytes::<A>::try_from_bytes_with_remaining_buffer(buffer)?;
		let (remaining_buffer, private_key_bytes) =
			PublicKeyBytes::<P>::try_from_bytes_with_remaining_buffer(remaining_buffer)?;
		Ok((remaining_buffer, Self { address_bytes, private_key_bytes }))
	}
}

impl<const A: usize, const P: usize> TransactionData for TransactionSigner<A, P> {}
