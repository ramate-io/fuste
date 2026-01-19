use crate::base::{
	id::Id,
	signer::{BaseSigner, SystemBufferAddress},
	Base,
};

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
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		let mut written_len = self.id.try_write_to_buffer(buffer)?;
		for signer in self.signers.iter() {
			if let Some(signer) = signer {
				written_len += signer.try_write_to_buffer(&mut buffer[written_len..])?;
			} else {
				written_len += SystemBufferAddress::BYTES_LENGTH;
			}
		}
		Ok(I + K * (N + P + SystemBufferAddress::BYTES_LENGTH))
	}
}

impl<const N: usize, const P: usize, const K: usize, const I: usize> Deserialize
	for BaseTransaction<N, P, K, I>
{
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		let (remaining_buffer, id) = Id::try_from_bytes_with_remaining_buffer(buffer)?;

		let mut signers = [Self::DEFAULT_SIGNER; K];

		// Get the signers until the end of the bytes or the signers array is full.
		let mut i = 0;
		while let Some(signer) = remaining_buffer.get(
			I + i * (N + P + SystemBufferAddress::BYTES_LENGTH)
				..I + i * (N + P + SystemBufferAddress::BYTES_LENGTH)
					+ (N + P + SystemBufferAddress::BYTES_LENGTH),
		) {
			if let Ok((_remaining_buffer, signer)) =
				BaseSigner::try_from_bytes_with_remaining_buffer(signer)
			{
				signers[i] = Some(signer);
			} else {
				signers[i] = None;
			}
			i += 1;
		}

		Ok((remaining_buffer, BaseTransaction { signers, id }))
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
