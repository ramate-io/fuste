#![no_std]
pub mod signer_index;
pub mod signer_load;
pub mod signer_store;

use fuste_channel::ChannelSystemId;
use fuste_serial_channel::SerialChannelError;
use fuste_serial_channel::SerialType;
use signer_index::{SignerIndex, TransactionSignerIndex};
use signer_load::TypedSignerLoad;
use signer_store::TypedSignerStore;

pub fn store_with_sizes<
	const RSIZE: usize,
	const N: usize,
	const P: usize,
	const K: usize,
	const TSIZE: usize,
	const B: usize,
	T: SerialType,
>(
	system_id: ChannelSystemId,
	signer_index: TransactionSignerIndex<N, P, K>,
	data: T,
) -> Result<(), SerialChannelError> {
	let typed_signer_store = TypedSignerStore::<N, P, K, T>::new(signer_index, data);
	typed_signer_store.store::<TSIZE, B, RSIZE>(system_id)
}

pub fn load_with_sizes<
	const RSIZE: usize,
	const WSIZE: usize,
	const N: usize,
	const P: usize,
	const K: usize,
	const TSIZE: usize,
	const B: usize,
	T: SerialType,
>(
	system_id: ChannelSystemId,
	signer_index: TransactionSignerIndex<N, P, K>,
	data: T,
) -> Result<T, SerialChannelError> {
	let typed_signer_load = TypedSignerLoad::<N, P, K, T>::new(signer_index, data);
	let value = typed_signer_load.load::<TSIZE, B, RSIZE>(system_id)?;
	Ok(value)
}

pub struct SignerStoreSystem<const N: usize, const P: usize, const K: usize> {
	channel_system_id: ChannelSystemId,
}

impl<const N: usize, const P: usize, const K: usize> Default for SignerStoreSystem<N, P, K> {
	fn default() -> Self {
		Self { channel_system_id: ChannelSystemId::new(0x516d) }
	}
}

impl<const N: usize, const P: usize, const K: usize> SignerStoreSystem<N, P, K> {
	pub fn store<T: SerialType>(
		&self,
		signer_index: impl SignerIndex<N, P, K>,
		data: T,
	) -> Result<(), SerialChannelError> {
		store_with_sizes::<{ 1024 * 32 }, N, P, K, { 32 * 32 }, B, T>(
			self.channel_system_id,
			signer_index.try_into(),
			data,
		)
	}
}
