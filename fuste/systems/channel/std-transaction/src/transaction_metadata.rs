use crate::containers::{SignerBackendAddress, SignerBackendCache, TransactionBackendId};

pub struct TransactionMetadata {
	transaction_backend_id: TransactionBackendId,
	signer_backend_cache: SignerBackendCache,
}

impl Default for TransactionMetadata {
	fn default() -> Self {
		Self {
			transaction_backend_id: TransactionBackendId::default(),
			signer_backend_cache: SignerBackendCache::default(),
		}
	}
}

impl TransactionMetadata {
	pub fn add_signer(&mut self, signer: SignerBackendAddress) {
		self.signer_backend_cache.insert(signer);
	}

	pub fn get_signer_at_index(&self, index: usize) -> Option<&SignerBackendAddress> {
		self.signer_backend_cache.at_index(index)
	}

	pub fn transaction_backend_id(&self) -> &TransactionBackendId {
		&self.transaction_backend_id
	}

	pub fn set_transaction_backend_id(&mut self, transaction_backend_id: TransactionBackendId) {
		self.transaction_backend_id = transaction_backend_id;
	}
}
