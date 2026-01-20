use fuste_std_signer_stores::signer_index::TransactionSignerIndex;
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
pub struct SignerBackendCache {
	signers: BTreeSet<SignerBackendAddress>,
}

impl Default for SignerBackendCache {
	fn default() -> Self {
		Self::new(BTreeSet::new())
	}
}

impl SignerBackendCache {
	pub fn new(signers: BTreeSet<SignerBackendAddress>) -> Self {
		Self { signers }
	}

	pub fn insert(&mut self, address: SignerBackendAddress) {
		self.signers.insert(address);
	}

	pub fn at_index(&self, index: usize) -> Option<&SignerBackendAddress> {
		self.signers.iter().nth(index)
	}

	pub fn contains(&self, address: &SignerBackendAddress) -> bool {
		self.signers.contains(address)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SignerBackendIndex {
	signers: SignerBackendCache,
}

impl SignerBackendIndex {
	pub fn new(signers: BTreeSet<SignerBackendAddress>) -> Self {
		Self { signers: SignerBackendCache::new(signers) }
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
		Self { signers: SignerBackendCache::new(signers) }
	}

	pub fn iter_signers(&self) -> impl Iterator<Item = &SignerBackendAddress> {
		self.signers.signers.iter()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HartIndex(pub SignerBackendIndex);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserSignerIndex(pub SignerBackendIndex);

impl UserSignerIndex {
	pub fn iter_signers(&self) -> impl Iterator<Item = &SignerBackendAddress> {
		self.0.iter_signers()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransactionBackendId(pub Vec<u8>);

impl TransactionBackendId {
	pub fn new(id: Vec<u8>) -> Self {
		Self(id)
	}
}

impl Default for TransactionBackendId {
	fn default() -> Self {
		Self(vec![0; 32])
	}
}

impl<const N: usize> From<TransactionBackendId> for [u8; N] {
	fn from(id: TransactionBackendId) -> [u8; N] {
		let mut out = [0u8; N];
		let src: &[u8] = id.0.as_ref();
		let len = core::cmp::min(N, src.len());
		out[..len].copy_from_slice(&src[..len]);
		out
	}
}
