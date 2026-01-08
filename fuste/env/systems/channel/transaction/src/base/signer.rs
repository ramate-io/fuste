use fuste_serial_channel::{Deserialize, SerialChannelError, Serialize};

/// The base transaction scheme that should be installed with any transaction-based system.
///
/// NOTE: this is not optimized to minimize, allocations.
/// You could have a strictly borrow-based version of this.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseSigner<const N: usize, const P: usize> {
	address_bytes: [u8; N],
	public_key_bytes: [u8; P],
}

impl<const N: usize, const P: usize> Serialize for BaseSigner<N, P> {
	fn try_to_bytes<const M: usize>(&self) -> Result<(usize, [u8; M]), SerialChannelError> {
		Ok((0, [0; M]))
	}
}

impl<const N: usize, const P: usize> Deserialize for BaseSigner<N, P> {
	fn try_from_bytes(bytes: &[u8]) -> Result<Self, SerialChannelError> {
		// First N bytes should be the address bytes.
		if bytes.len() < N {
			return Err(SerialChannelError::SerializedBufferTooSmall(N as u32));
		}
		let mut copy_address_buffer = [0; N];
		copy_address_buffer.copy_from_slice(&bytes[..N]);
		let address_bytes = copy_address_buffer;

		// Next P bytes should be the public key bytes.
		if bytes.len() < N + P {
			return Err(SerialChannelError::SerializedBufferTooSmall(N as u32 + P as u32));
		}
		let mut copy_public_key_buffer = [0; P];
		copy_public_key_buffer.copy_from_slice(&bytes[N..N + P]);
		let public_key_bytes = copy_public_key_buffer;

		Ok(BaseSigner { address_bytes, public_key_bytes })
	}
}
