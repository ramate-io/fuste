pub mod base_signer;

use fuste_channel::{
	systems::ChannelSystem, ChannelError, ChannelStatus, ChannelStatusCode, ChannelSystemStatus,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct SignerBackendAddress(Vec<u8>);

impl SignerBackendAddress {
	pub fn new(address_bytes: Vec<u8>) -> Self {
		Self(address_bytes)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SignerBackendIndex {
	signers: BTreeSet<SignerBackendAddress>,
}

impl SignerBackendIndex {
	pub fn new(signers: BTreeSet<SignerBackendAddress>) -> Self {
		Self { signers }
	}

	pub fn add_address(&mut self, address: SignerBackendAddress) {
		self.signers.insert(address);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HartIndex(pub SignerBackendIndex);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserSignerIndex(pub SignerBackendIndex);

#[derive(thiserror::Error, Debug)]
pub enum SignerStoreBackendError {
	#[error("Failed to write to signer store backend: {0}")]
	WriteError(String),
	#[error("Failed to read from signer store backend: {0}")]
	ReadError(String),
}

pub trait SignerStoreBackend {
	/// Writes value bytes to the signer store backend for the given hart and user signer.
	fn write(
		&self,
		hart_index: &HartIndex,
		user_signer_index: &UserSignerIndex,
		value_bytes: &[u8],
	) -> Result<(), SignerStoreBackendError>;

	/// Reads value bytes from the signer store backend for the given hart and user signer.
	fn read(
		&self,
		hart_index: &HartIndex,
		user_signer_index: &UserSignerIndex,
	) -> Result<Option<Vec<u8>>, SignerStoreBackendError>;
}

#[derive(thiserror::Error, Debug)]
pub enum SignerStoreError {
	/// An error occurred while writing to the signer store backend.
	#[error("Failure in signer store backend: {0}")]
	BackendError(#[from] SignerStoreBackendError),
}

pub struct SignerStore<const N: usize, const P: usize, S: SignerStoreBackend> {
	backend: S,
	hart_index: HartIndex,
}

impl<const N: usize, const P: usize, S: SignerStoreBackend> SignerStore<N, P, S> {
	pub fn new(backend: S, hart_index: HartIndex) -> Self {
		Self { backend, hart_index }
	}

	/// Writes bytes to the signer store backend.
	pub fn write_bytes_to_backend(
		&self,
		user_signer_index: impl Into<UserSignerIndex>,
		value_bytes: &[u8],
	) -> Result<(), SignerStoreError> {
		self.backend
			.write(&self.hart_index, &user_signer_index.into(), value_bytes)
			.map_err(SignerStoreError::BackendError)
	}

	/// Reads bytes from the signer store backend.
	pub fn read_bytes_from_backend(
		&self,
		user_signer_index: impl Into<UserSignerIndex>,
	) -> Result<Option<Vec<u8>>, SignerStoreError> {
		self.backend
			.read(&self.hart_index, &user_signer_index.into())
			.map_err(SignerStoreError::BackendError)
	}

	/// Reads bytes from the signer store backend into the given buffer.
	pub fn read_bytes_from_backend_into(
		&self,
		user_signer_index: UserSignerIndex,
		buffer: &mut [u8],
	) -> Result<(), SignerStoreError> {
		self.backend
			.read(&self.hart_index, &user_signer_index)
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

impl<const N: usize, const P: usize, S: SignerStoreBackend> ChannelSystem for SignerStore<N, P, S> {
	fn handle_open(
		&self,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError> {
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
