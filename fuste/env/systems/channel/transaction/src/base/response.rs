use crate::base::{id::Id, signer::BaseSigner, Base};

use crate::{response::TransactionDataResponse, TransactionScheme, TransactionSchemeId};
use fuste_serial_channel::{Deserialize, SerialChannelError, Serialize};

/// The base transaction scheme that should be installed with any transaction-based system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseTransaction<const N: usize, const P: usize, const K: usize, const I: usize> {
	signers: [Option<BaseSigner<N, P>>; K],
	id: Id<I>,
}

impl<const N: usize, const P: usize, const K: usize, const I: usize> BaseTransaction<N, P, K, I> {
	const DEFAULT_SIGNER: Option<BaseSigner<N, P>> = None;
}

impl<const N: usize, const P: usize, const K: usize, const I: usize> TransactionScheme
	for BaseTransaction<N, P, K, I>
{
	fn scheme_id() -> TransactionSchemeId {
		let mut id = [0; 32];

		// first 4 bytes should be N
		id[..4].copy_from_slice(&N.to_le_bytes());
		// next 4 bytes should be P
		id[4..8].copy_from_slice(&P.to_le_bytes());
		// next 4 bytes should be K
		id[8..12].copy_from_slice(&K.to_le_bytes());
		// next 4 bytes should be I
		id[12..16].copy_from_slice(&I.to_le_bytes());

		TransactionSchemeId(id)
	}
}

impl<const N: usize, const P: usize, const K: usize, const I: usize> Serialize
	for BaseTransaction<N, P, K, I>
{
	fn try_to_bytes<const M: usize>(&self) -> Result<(usize, [u8; M]), SerialChannelError> {
		Ok((0, [0; M]))
	}
}

impl<const N: usize, const P: usize, const K: usize, const I: usize> Deserialize
	for BaseTransaction<N, P, K, I>
{
	fn try_from_bytes(bytes: &[u8]) -> Result<Self, SerialChannelError> {
		// the first I bytes should be the id
		if bytes.len() < I {
			return Err(SerialChannelError::SerializedBufferTooSmall(I as u32));
		}
		let id = Id::copy_from_slice(&bytes[..I]);

		let mut signers = [Self::DEFAULT_SIGNER; K];

		// Get the signers until the end of the bytes or the signers array is full.
		let mut i = 0;
		while let Some(signer) = bytes.get(I + i * (N + P)..I + i * (N + P) + (N + P)) {
			signers[i] =
				BaseSigner::try_from_bytes(signer).map(Some).unwrap_or(Self::DEFAULT_SIGNER);
			i += 1;
		}

		Ok(BaseTransaction { signers, id })
	}
}

impl<const N: usize, const P: usize, const K: usize, const I: usize>
	TransactionDataResponse<BaseSigner<N, P>, Id<I>, Base<BaseSigner<N, P>, Id<I>, Self>>
	for BaseTransaction<N, P, K, I>
{
	fn signers(&self) -> &[Option<BaseSigner<N, P>>] {
		&self.signers
	}

	fn id(&self) -> &Id<I> {
		&self.id
	}
}
