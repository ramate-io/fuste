pub mod load;
pub mod store;

use fuste_channel::{
	systems::ChannelSystem, ChannelError, ChannelStatus, ChannelStatusCode, ChannelSystemStatus,
};
use fuste_std_signer_stores::signer_index::TransactionSignerIndex;
use fuste_std_signer_stores::signer_store::SignerStore;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct SignerBackendAddress(Vec<u8>);

impl From<&[u8]> for SignerBackendAddress {
	fn from(value: &[u8]) -> Self {
		Self(value.to_vec())
	}
}

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

	pub fn from_transaction_signer_index<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
	>(
		transaction_signer_index: TransactionSignerIndex<
			ADDRESS_BYTES,
			PUBLIC_KEY_BYTES,
			SIGNER_COUNT,
		>,
	) -> Self {
		let mut signers = BTreeSet::new();
		for signer in transaction_signer_index.signers() {
			if let Some(signer) = signer {
				signers.insert(SignerBackendAddress::from(signer.address().as_bytes()));
			}
		}
		Self { signers }
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
