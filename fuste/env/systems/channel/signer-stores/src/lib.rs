pub mod base;

use core::marker::PhantomData;
use fuste_channel::ChannelSystemId;
use fuste_serial_channel::{Deserialize, SerialChannelError, SerialType, Serialize};

/// A marker trait for types that can be stored in a signer store.
pub trait StoreSignerIndex<S>: SerialType {
	fn index_signers(&self) -> &[Option<S>];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignerStore<const N: usize, S, I: StoreSignerIndex<S>> {
	__signer_marker: PhantomData<S>,
	pub index: I,
	pub data_bytes: [u8; N],
}

impl<const N: usize, S, I: StoreSignerIndex<S>> SignerStore<N, S, I> {
	pub const CHANNEL_SYSTEM_ID: ChannelSystemId = ChannelSystemId::constant(0xad03);

	pub fn new(index: I, data_bytes: [u8; N]) -> Self {
		Self { index, data_bytes, __signer_marker: PhantomData }
	}
}

impl<const N: usize, S, I: StoreSignerIndex<S>> Serialize for SignerStore<N, S, I> {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		let mut written_len = self.index.try_write_to_buffer(buffer)?;
		buffer[written_len..written_len + N].copy_from_slice(&self.data_bytes);
		written_len += N;
		Ok(written_len)
	}
}

impl<const N: usize, S, I: StoreSignerIndex<S>> Deserialize for SignerStore<N, S, I> {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		let (remaining_buffer, index) = I::try_from_bytes_with_remaining_buffer(buffer)?;

		let mut data_bytes = [0; N];
		data_bytes.copy_from_slice(&remaining_buffer[..N]);

		Ok((remaining_buffer, Self { index, data_bytes, __signer_marker: PhantomData }))
	}
}
