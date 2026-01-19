use crate::signer_stores::{SignerBackendAddress, SignerBackendIndex};
use fuste_transaction::base::signer::index::BaseSignerIndex;

impl<const N: usize, const P: usize, const K: usize> From<BaseSignerIndex<N, P, K>>
	for SignerBackendIndex
{
	fn from(index: BaseSignerIndex<N, P, K>) -> Self {
		Self {
			signers: index
				.signers()
				.into_iter()
				.filter_map(|signer| {
					signer
						.as_ref()
						.map(|signer| SignerBackendAddress::new(signer.address_bytes().to_vec()))
				})
				.collect(),
		}
	}
}
