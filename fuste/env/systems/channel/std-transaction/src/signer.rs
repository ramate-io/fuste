use crate::{TransactionData, TransactionScheme};
use fuste_channel::ChannelSystemId;
use fuste_serial_channel::{serial_channel_request, Deserialize, SerialChannelError, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddressBytes<const N: usize>([u8; N]);

impl<const N: usize> AddressBytes<N> {
	const DEFAULT_ADDRESS_BYTES: Self = Self([0; N]);

	pub fn new(bytes: [u8; N]) -> Self {
		Self(bytes)
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.0
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

	pub fn as_bytes(&self) -> &[u8] {
		&self.0
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
	pub public_key_bytes: PublicKeyBytes<P>,
}

impl<const A: usize, const P: usize> Default for TransactionSigner<A, P> {
	fn default() -> Self {
		Self {
			address_bytes: AddressBytes::<A>::DEFAULT_ADDRESS_BYTES,
			public_key_bytes: PublicKeyBytes::<P>::DEFAULT_PUBLIC_KEY_BYTES,
		}
	}
}

impl<const A: usize, const P: usize> TransactionSigner<A, P> {
	pub const DEFAULT_SIGNER: Self = Self {
		address_bytes: AddressBytes::<A>::DEFAULT_ADDRESS_BYTES,
		public_key_bytes: PublicKeyBytes::<P>::DEFAULT_PUBLIC_KEY_BYTES,
	};

	pub fn new(address_bytes: AddressBytes<A>, public_key_bytes: PublicKeyBytes<P>) -> Self {
		Self { address_bytes, public_key_bytes }
	}

	pub fn address(&self) -> &AddressBytes<A> {
		&self.address_bytes
	}

	pub fn private_key(&self) -> &PublicKeyBytes<P> {
		&self.public_key_bytes
	}
}

impl<const A: usize, const P: usize> Serialize for TransactionSigner<A, P> {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		let mut written_len = self.address_bytes.try_write_to_buffer(buffer)?;
		written_len += self.public_key_bytes.try_write_to_buffer(&mut buffer[written_len..])?;
		Ok(written_len)
	}
}

impl<const A: usize, const P: usize> Deserialize for TransactionSigner<A, P> {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		let (remaining_buffer, address_bytes) =
			AddressBytes::<A>::try_from_bytes_with_remaining_buffer(buffer)?;
		let (remaining_buffer, public_key_bytes) =
			PublicKeyBytes::<P>::try_from_bytes_with_remaining_buffer(remaining_buffer)?;
		Ok((remaining_buffer, Self { address_bytes, public_key_bytes }))
	}
}

#[derive(Debug, Clone)]
pub struct TransactionSignerAtIndex {
	index: u32,
}

impl TransactionSignerAtIndex {
	pub fn new(index: u32) -> Self {
		Self { index }
	}

	pub fn index(&self) -> u32 {
		self.index
	}
}

impl TransactionScheme for TransactionSignerAtIndex {
	const CHANNEL_SYSTEM_ID: ChannelSystemId = ChannelSystemId::constant(0x516d);
}

impl Deserialize for TransactionSignerAtIndex {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		if buffer.len() < 4 {
			return Err(SerialChannelError::SerializedBufferTooSmall(4));
		}
		let index = u32::from_le_bytes(buffer[..4].try_into().unwrap());
		Ok((&buffer[4..], Self { index }))
	}
}

impl Serialize for TransactionSignerAtIndex {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		if buffer.len() < 4 {
			return Err(SerialChannelError::SerializedBufferTooSmall(4));
		}

		let inner_buffer = &mut buffer[..4];
		inner_buffer.copy_from_slice(&self.index.to_le_bytes());

		Ok(4)
	}
}

impl<const A: usize, const P: usize> TransactionData for TransactionSigner<A, P> {}

pub fn signer_at_index_with_sizes<
	const RSIZE: usize,
	const WSIZE: usize,
	const A: usize,
	const P: usize,
>(
	index: u32,
) -> Result<TransactionSigner<A, P>, SerialChannelError> {
	let signer =
		serial_channel_request::<RSIZE, WSIZE, TransactionSignerAtIndex, TransactionSigner<A, P>>(
			TransactionSignerAtIndex::CHANNEL_SYSTEM_ID,
			&TransactionSignerAtIndex { index },
		)?;
	Ok(signer)
}

pub fn signer_at_index<const A: usize, const P: usize>(
	index: u32,
) -> Result<TransactionSigner<A, P>, SerialChannelError> {
	signer_at_index_with_sizes::<1024, 1024, A, P>(index)
}
