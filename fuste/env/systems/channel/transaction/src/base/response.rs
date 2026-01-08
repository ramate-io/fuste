use crate::base::signer::BaseSigner;

use crate::{TransactionScheme, TransactionSchemeId};
use fuste_serial_channel::{Deserialize, SerialChannelError, Serialize};

/// The base transaction scheme that should be installed with any transaction-based system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseTransaction<const N: usize, const P: usize, const K: usize> {
	signers: [Option<BaseSigner<N, P>>; K],
}

impl<const N: usize, const P: usize, const K: usize> BaseTransaction<N, P, K> {
	const DEFAULT_SIGNER: Option<BaseSigner<N, P>> = None;
}

impl<const N: usize, const P: usize, const K: usize> TransactionScheme
	for BaseTransaction<N, P, K>
{
	fn scheme_id() -> TransactionSchemeId {
		TransactionSchemeId((N as u32, P as u32))
	}
}

impl<const N: usize, const P: usize, const K: usize> Serialize for BaseTransaction<N, P, K> {
	fn try_to_bytes<const M: usize>(&self) -> Result<(usize, [u8; M]), SerialChannelError> {
		Ok((0, [0; M]))
	}
}

impl<const N: usize, const P: usize, const K: usize> Deserialize for BaseTransaction<N, P, K> {
	fn try_from_bytes(bytes: &[u8]) -> Result<Self, SerialChannelError> {
		let mut signers = [Self::DEFAULT_SIGNER; K];

		// Get the signers until the end of the bytes or the signers array is full.
		let mut i = 0;
		while let Some(signer) = bytes.get(0..N + P) {
			signers[i] =
				BaseSigner::try_from_bytes(signer).map(Some).unwrap_or(Self::DEFAULT_SIGNER);
			i += 1;
		}

		Ok(BaseTransaction { signers })
	}
}
