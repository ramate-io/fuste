pub mod transaction_signer_at_index;

use crate::containers::{
	HartIndex, SignerBackendAddress, SignerBackendCache, TransactionBackendId,
};

pub struct TransactionMetadata {
	transaction_backend_id: TransactionBackendId,
	hart_index: HartIndex,
	signer_backend_cache: SignerBackendCache,
}

impl Default for TransactionMetadata {
	fn default() -> Self {
		Self {
			transaction_backend_id: TransactionBackendId::default(),
			hart_index: HartIndex::default(),
			signer_backend_cache: SignerBackendCache::default(),
		}
	}
}

impl TransactionMetadata {
	pub fn signer_backend_cache(&self) -> &SignerBackendCache {
		&self.signer_backend_cache
	}

	pub fn add_signer(&mut self, signer: SignerBackendAddress) {
		self.signer_backend_cache.insert(signer);
	}

	pub fn get_signer_at_index(&self, index: usize) -> Option<&SignerBackendAddress> {
		self.signer_backend_cache.at_index(index)
	}

	pub fn hart_index(&self) -> &HartIndex {
		&self.hart_index
	}

	pub fn set_hart_index(&mut self, hart_index: HartIndex) {
		self.hart_index = hart_index;
	}

	pub fn transaction_backend_id(&self) -> &TransactionBackendId {
		&self.transaction_backend_id
	}

	pub fn set_transaction_backend_id(&mut self, transaction_backend_id: TransactionBackendId) {
		self.transaction_backend_id = transaction_backend_id;
	}
}
