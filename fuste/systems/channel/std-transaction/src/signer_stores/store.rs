use super::{
	HartIndex, SignerBackendIndex, SignerStoreBackend, SignerStoreBackendError, SignerStoreError,
	UserSignerIndex,
};
use fuste_channel::{
	systems::ChannelSystem, ChannelError, ChannelStatus, ChannelStatusCode, ChannelSystemStatus,
};
use fuste_serial_channel::Deserialize;
use fuste_std_signer_stores::signer_store::SignerStore;

pub struct SignerStorage<
	const ADDRESS_BYTES: usize,
	const PUBLIC_KEY_BYTES: usize,
	const SIGNER_COUNT: usize,
	const TYPE_NAME_BYTES: usize,
	const VALUE_BYTES: usize,
	S: SignerStoreBackend,
> {
	backend: S,
	hart_index: HartIndex,
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
		S: SignerStoreBackend,
	> SignerStorage<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES, S>
{
	pub fn new(backend: S, hart_index: HartIndex) -> Self {
		Self { backend, hart_index }
	}

	/// Writes bytes to the signer store backend.
	pub fn write_bytes_to_backend(
		&self,
		user_signer_index: impl Into<UserSignerIndex>,
		type_bytes: &[u8],
		value_bytes: &[u8],
	) -> Result<(), SignerStoreError> {
		self.backend
			.write(&self.hart_index, &user_signer_index.into(), type_bytes, value_bytes)
			.map_err(SignerStoreError::BackendError)
	}

	/// Reads bytes from the signer store backend.
	pub fn read_bytes_from_backend(
		&self,
		user_signer_index: impl Into<UserSignerIndex>,
		type_bytes: &[u8],
	) -> Result<Option<Vec<u8>>, SignerStoreError> {
		self.backend
			.read(&self.hart_index, &user_signer_index.into(), type_bytes)
			.map_err(SignerStoreError::BackendError)
	}

	/// Reads bytes from the signer store backend into the given buffer.
	pub fn read_bytes_from_backend_into(
		&self,
		user_signer_index: UserSignerIndex,
		type_bytes: &[u8],
		buffer: &mut [u8],
	) -> Result<(), SignerStoreError> {
		self.backend
			.read(&self.hart_index, &user_signer_index, type_bytes)
			.map_err(SignerStoreError::BackendError)
			.and_then(|value| {
				if let Some(value) = value {
					buffer.copy_from_slice(&value);
					Ok(())
				} else {
					Err(SignerStoreError::BackendError(SignerStoreBackendError::ReadError(
						"No value found".to_string(),
					)))
				}
			})
	}
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
		S: SignerStoreBackend,
	> ChannelSystem
	for SignerStorage<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES, S>
{
	fn handle_open(
		&self,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError> {
		let signer_store = SignerStore::<
			ADDRESS_BYTES,
			PUBLIC_KEY_BYTES,
			SIGNER_COUNT,
			TYPE_NAME_BYTES,
			VALUE_BYTES,
		>::try_from_bytes(read_buffer)
		.map_err(|_| ChannelError::Internal)?;

		let user_signer_index = UserSignerIndex(SignerBackendIndex::from_transaction_signer_index(
			signer_store.signer_index().clone(),
		));

		self.write_bytes_to_backend(
			user_signer_index,
			signer_store.type_bytes(),
			signer_store.bytes(),
		)
		.map_err(|_| ChannelError::Internal)?;

		Ok(ChannelStatus::new(0, ChannelStatusCode::Success, ChannelSystemStatus::new(0)))
	}

	fn handle_check(
		&self,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError> {
		Ok(ChannelStatus::new(0, ChannelStatusCode::Success, ChannelSystemStatus::new(0)))
	}
}
