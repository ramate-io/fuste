use crate::base::signer::BaseSigner;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BaseSignerIndex<const N: usize, const P: usize, const K: usize> {
	index: [Option<BaseSigner<N, P>>; K],
}

impl<const N: usize, const P: usize, const K: usize> BaseSignerIndex<N, P, K> {
	pub fn new(signers: [Option<BaseSigner<N, P>>; K]) -> Self {
		Self { index: signers }
	}

	pub fn signers(&self) -> &[Option<BaseSigner<N, P>>] {
		&self.index
	}
}
