use super::{
	HartIndex, SignerBackendAddress, SignerBackendIndex, SignerStoreBackend,
	SignerStoreBackendError, SignerStoreError, UserSignerIndex,
};
use fuste_channel::{
	systems::ChannelSystem, ChannelError, ChannelStatus, ChannelStatusCode, ChannelSystemStatus,
};
use fuste_serial_channel::Deserialize;
use fuste_std_signer_stores::signer_store::{Op, SignerStore};
use std::collections::BTreeSet;

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
	authenticated_signers: BTreeSet<SignerBackendAddress>,
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
		Self { backend, hart_index, authenticated_signers: BTreeSet::new() }
	}

	pub fn is_signer_authenticated(&self, signer_address: SignerBackendAddress) -> bool {
		self.authenticated_signers.contains(&signer_address)
	}

	pub fn is_signer_index_authenticated(&self, signer_index: UserSignerIndex) -> bool {
		let mut all_signers_authenticated = true;
		for signer in signer_index.iter_signers() {
			if !self.authenticated_signers.contains(&signer) {
				all_signers_authenticated = false;
				break;
			}
		}
		all_signers_authenticated
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
		read_write_buffer: &mut [u8],
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

		let op = signer_store.op();

		// If Op::STORE is set, write to backend
		if op.contains(Op::STORE) {
			self.write_bytes_to_backend(
				user_signer_index.clone(),
				signer_store.type_bytes(),
				signer_store.bytes(),
			)
			.map_err(|_| ChannelError::Internal)?;
		}

		// If Op::LOAD is set, read from backend and write to read_write_buffer
		if op.contains(Op::LOAD) {
			match self.read_bytes_from_backend(user_signer_index, signer_store.type_bytes()) {
				Ok(Some(value_bytes)) => {
					if value_bytes.len() <= read_write_buffer.len() {
						read_write_buffer[..value_bytes.len()].copy_from_slice(&value_bytes);
					} else {
						return Err(ChannelError::Internal);
					}
				}
				Ok(None) => {
					// No value found, write zeros
					let len = VALUE_BYTES.min(read_write_buffer.len());
					read_write_buffer[..len].fill(0);
				}
				Err(_) => return Err(ChannelError::Internal),
			}
		}

		Ok(ChannelStatus::new(0, ChannelStatusCode::Success, ChannelSystemStatus::new(0)))
	}

	fn handle_check(
		&self,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError> {
		// handle_check only reads from backend, never writes
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

		// Read from backend and write to write_buffer
		match self.read_bytes_from_backend(user_signer_index, signer_store.type_bytes()) {
			Ok(Some(value_bytes)) => {
				if value_bytes.len() <= write_buffer.len() {
					write_buffer[..value_bytes.len()].copy_from_slice(&value_bytes);
				} else {
					return Err(ChannelError::Internal);
				}
			}
			Ok(None) => {
				// No value found, write zeros
				let len = VALUE_BYTES.min(write_buffer.len());
				write_buffer[..len].fill(0);
			}
			Err(_) => return Err(ChannelError::Internal),
		}

		Ok(ChannelStatus::new(0, ChannelStatusCode::Success, ChannelSystemStatus::new(0)))
	}
}
