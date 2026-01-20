pub mod store;

use crate::containers::{HartIndex, UserSignerIndex};

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
		type_bytes: &[u8],
		value_bytes: &[u8],
	) -> Result<(), SignerStoreBackendError>;

	/// Reads value bytes from the signer store backend for the given hart and user signer.
	fn read(
		&self,
		hart_index: &HartIndex,
		user_signer_index: &UserSignerIndex,
		type_bytes: &[u8],
	) -> Result<Option<Vec<u8>>, SignerStoreBackendError>;
}

#[derive(thiserror::Error, Debug)]
pub enum SignerStoreError {
	/// An error occurred while writing to the signer store backend.
	#[error("Failure in signer store backend: {0}")]
	BackendError(#[from] SignerStoreBackendError),
}
