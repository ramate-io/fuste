use crate::{TransactionData, TransactionScheme};
use fuste_channel::ChannelSystemId;
use fuste_serial_channel::{serial_channel_request, Deserialize, SerialChannelError, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionId<const N: usize>([u8; N]);

impl<const N: usize> TransactionId<N> {
	pub fn request() -> TransactionId<N> {
		Self([0; N])
	}
}

impl<const N: usize> Serialize for TransactionId<N> {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		buffer.copy_from_slice(&self.0);
		Ok(self.0.len())
	}
}

impl<const N: usize> Deserialize for TransactionId<N> {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		if buffer.len() < N {
			return Err(SerialChannelError::SerializedBufferTooSmall(N as u32));
		}
		Ok((&buffer[N..], Self(buffer.try_into().unwrap())))
	}
}

impl<const N: usize> TransactionScheme for TransactionId<N> {
	const CHANNEL_SYSTEM_ID: ChannelSystemId = ChannelSystemId::constant(0x1d);
}

impl<const N: usize> TransactionData for TransactionId<N> {}

pub fn transaction_id_with_sizes<const RSIZE: usize, const WSIZE: usize, const N: usize>(
) -> Result<TransactionId<N>, SerialChannelError> {
	let request = TransactionId::<N>::request();

	serial_channel_request::<RSIZE, WSIZE, TransactionId<N>, TransactionId<N>>(
		TransactionId::<N>::CHANNEL_SYSTEM_ID,
		&request,
	)
}

pub fn transaction_id<const N: usize>() -> Result<TransactionId<N>, SerialChannelError> {
	transaction_id_with_sizes::<1024, 1024, N>()
}
