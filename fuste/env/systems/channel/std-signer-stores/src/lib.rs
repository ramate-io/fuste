#![no_std]
pub mod signer_index;
pub mod signer_store;

pub use signer_store::Op;

use fuste_channel::ChannelSystemId;
use fuste_serial_channel::SerialChannelError;
use fuste_serial_channel::SerialType;
use signer_index::{SignerIndex, TransactionSignerIndex};
use signer_store::TypedSignerStore;

/// Store data to the channel with explicit size parameters.
///
/// This function creates a typed signer store and stores it to the channel.
/// The data is serialized and stored along with the signer index and type information.
///
/// # Parameters
/// - `RSIZE`: Request buffer size for the serial channel
/// - `ADDRESS_BYTES`: Size of address in bytes
/// - `PUBLIC_KEY_BYTES`: Size of public key in bytes
/// - `SIGNER_COUNT`: Number of signers
/// - `TYPE_NAME_BYTES`: Maximum size for type name in bytes
/// - `VALUE_BYTES`: Maximum size for serialized value in bytes
/// - `T`: The type to store (must implement `SerialType`)
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

/// Load data from the channel with explicit size parameters.
///
/// This function creates a typed signer store and loads data from the channel.
/// The data is deserialized and returned along with the signer index and type information.
///
/// # Parameters
/// - `RSIZE`: Request buffer size for the serial channel
/// - `WSIZE`: Write buffer size (unused, kept for API compatibility)
/// - `ADDRESS_BYTES`: Size of address in bytes
/// - `PUBLIC_KEY_BYTES`: Size of public key in bytes
/// - `SIGNER_COUNT`: Number of signers
/// - `TYPE_NAME_BYTES`: Maximum size for type name in bytes
/// - `VALUE_BYTES`: Maximum size for serialized value in bytes
/// - `T`: The type to load (must implement `SerialType`)
///
/// # Note
/// The `data` parameter is used as a type hint and seed value. The actual data is loaded from the channel.
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
	let typed_signer_store =
		TypedSignerStore::<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, T>::new(
			signer_index,
			data,
		);
	typed_signer_store.load::<TYPE_NAME_BYTES, VALUE_BYTES, RSIZE>(system_id)
}

/// A system for storing and loading data associated with signers.
///
/// This system provides a unified API for storing and loading typed data
/// associated with transaction signers. It uses the channel system to
/// communicate with the backend storage.
///
/// # Type Parameters
/// - `ADDRESS_BYTES`: Size of address in bytes
/// - `PUBLIC_KEY_BYTES`: Size of public key in bytes
/// - `SIGNER_COUNT`: Number of signers
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
	/// Store data with default buffer sizes.
	///
	/// Uses default sizes:
	/// - `RSIZE`: 32768 (32KB) - Request buffer size
	/// - `TYPE_NAME_BYTES`: 128 - Maximum type name size
	/// - `VALUE_BYTES`: 16384 (16KB) - Maximum serialized value size
	///
	/// For custom sizes, use `store_with_sizes`.
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

	/// Store data with custom buffer sizes.
	///
	/// Allows fine-grained control over buffer sizes for specialized use cases.
	/// Use `store` for default sizes unless you have specific requirements.
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

	/// Load data with default buffer sizes.
	///
	/// Uses default sizes:
	/// - `RSIZE`: 32768 (32KB) - Request buffer size
	/// - `TYPE_NAME_BYTES`: 128 - Maximum type name size
	/// - `VALUE_BYTES`: 16384 (16KB) - Maximum serialized value size
	///
	/// The `data` parameter is used as a type hint and seed value.
	/// For custom sizes, use `load_with_sizes`.
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

	/// Load data with custom buffer sizes.
	///
	/// Allows fine-grained control over buffer sizes for specialized use cases.
	/// Use `load` for default sizes unless you have specific requirements.
	///
	/// The `data` parameter is used as a type hint and seed value.
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
