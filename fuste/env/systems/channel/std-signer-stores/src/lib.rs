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
	const ADDRESS_BYTES: usize,
	const PUBLIC_KEY_BYTES: usize,
	const SIGNER_COUNT: usize,
	const TYPE_NAME_BYTES: usize,
	const VALUE_BYTES: usize,
	T: SerialType,
>(
	system_id: ChannelSystemId,
	signer_index: TransactionSignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
	data: T,
) -> Result<(), SerialChannelError> {
	let typed_signer_store =
		TypedSignerStore::<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, T>::new(
			signer_index,
			data,
		);
	typed_signer_store.store::<TYPE_NAME_BYTES, VALUE_BYTES, RSIZE>(system_id)
}

pub fn load_with_sizes<
	const RSIZE: usize,
	const WSIZE: usize,
	const ADDRESS_BYTES: usize,
	const PUBLIC_KEY_BYTES: usize,
	const SIGNER_COUNT: usize,
	const TYPE_NAME_BYTES: usize,
	const VALUE_BYTES: usize,
	T: SerialType,
>(
	system_id: ChannelSystemId,
	signer_index: TransactionSignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
	data: T,
) -> Result<T, SerialChannelError> {
	let typed_signer_load =
		TypedSignerLoad::<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, T>::new(
			signer_index,
			data,
		);
	let value = typed_signer_load.load::<TYPE_NAME_BYTES, VALUE_BYTES, RSIZE>(system_id)?;
	Ok(value)
}

pub struct SignerStoreSystem<
	const ADDRESS_BYTES: usize,
	const PUBLIC_KEY_BYTES: usize,
	const SIGNER_COUNT: usize,
> {
	channel_system_id: ChannelSystemId,
}

impl<const ADDRESS_BYTES: usize, const PUBLIC_KEY_BYTES: usize, const SIGNER_COUNT: usize> Default
	for SignerStoreSystem<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>
{
	fn default() -> Self {
		Self { channel_system_id: ChannelSystemId::new(0x516d) }
	}
}

impl<const ADDRESS_BYTES: usize, const PUBLIC_KEY_BYTES: usize, const SIGNER_COUNT: usize>
	SignerStoreSystem<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>
{
	pub fn store<T: SerialType>(
		&self,
		signer_index: impl SignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
		data: T,
	) -> Result<(), SerialChannelError> {
		let transaction_signer_index: TransactionSignerIndex<
			ADDRESS_BYTES,
			PUBLIC_KEY_BYTES,
			SIGNER_COUNT,
		> = signer_index
			.try_into()
			.map_err(|_| SerialChannelError::SerializedBufferTooSmall(0))?;
		store_with_sizes::<
			{ 1024 * 32 },
			ADDRESS_BYTES,
			PUBLIC_KEY_BYTES,
			SIGNER_COUNT,
			128,
			{ 128 * 32 },
			T,
		>(self.channel_system_id.clone(), transaction_signer_index, data)
	}
}
