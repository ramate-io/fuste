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
	/// Store with default sizes: RSIZE=32768 (32KB), TYPE_NAME_BYTES=128, VALUE_BYTES=16384 (16KB)
	pub fn store<T: SerialType>(
		&self,
		signer_index: impl SignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
		data: T,
	) -> Result<(), SerialChannelError> {
		self.store_with_sizes::<
			T,
			{ 1024 * 32 }, // RSIZE: 32KB
			128,          // TYPE_NAME_BYTES
			{ 1024 * 16 }, // VALUE_BYTES: 16KB
		>(signer_index, data)
	}

	/// Store with custom sizes
	pub fn store_with_sizes<
		T: SerialType,
		const RSIZE: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
	>(
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
			RSIZE,
			ADDRESS_BYTES,
			PUBLIC_KEY_BYTES,
			SIGNER_COUNT,
			TYPE_NAME_BYTES,
			VALUE_BYTES,
			T,
		>(self.channel_system_id.clone(), transaction_signer_index, data)
	}

	/// Load with default sizes: RSIZE=32768 (32KB), TYPE_NAME_BYTES=128, VALUE_BYTES=16384 (16KB)
	pub fn load<T: SerialType>(
		&self,
		signer_index: impl SignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
		data: T,
	) -> Result<T, SerialChannelError> {
		self.load_with_sizes::<
			T,
			{ 1024 * 32 }, // RSIZE: 32KB
			128,          // TYPE_NAME_BYTES
			{ 1024 * 16 }, // VALUE_BYTES: 16KB
		>(signer_index, data)
	}

	/// Load with custom sizes
	pub fn load_with_sizes<
		T: SerialType,
		const RSIZE: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
	>(
		&self,
		signer_index: impl SignerIndex<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT>,
		data: T,
	) -> Result<T, SerialChannelError> {
		let transaction_signer_index: TransactionSignerIndex<
			ADDRESS_BYTES,
			PUBLIC_KEY_BYTES,
			SIGNER_COUNT,
		> = signer_index
			.try_into()
			.map_err(|_| SerialChannelError::SerializedBufferTooSmall(0))?;
		load_with_sizes::<
			RSIZE,
			0, // WSIZE: unused but required by function signature
			ADDRESS_BYTES,
			PUBLIC_KEY_BYTES,
			SIGNER_COUNT,
			TYPE_NAME_BYTES,
			VALUE_BYTES,
			T,
		>(self.channel_system_id.clone(), transaction_signer_index, data)
	}
}
