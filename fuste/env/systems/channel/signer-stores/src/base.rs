use crate::StoreSignerIndex;
use fuste_serial_channel::{Deserialize, SerialChannelError, Serialize};
use fuste_transaction::base::signer::{BaseSigner, SystemBufferAddress};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BaseSignerIndex<const N: usize, const P: usize, const K: usize> {
	index: [Option<BaseSigner<N, P>>; K],
}

impl<const N: usize, const P: usize, const K: usize> BaseSignerIndex<N, P, K> {
	const DEFAULT_SIGNER: Option<BaseSigner<N, P>> = None;

	pub fn new(signers: [Option<BaseSigner<N, P>>; K]) -> Self {
		Self { index: signers }
	}

	pub fn empty() -> Self {
		Self { index: [Self::DEFAULT_SIGNER; K] }
	}

	/// Adds a best signer to the first None value in the index.
	pub fn add(&mut self, signer: BaseSigner<N, P>) {
		for i in 0..K {
			if self.index[i].is_none() {
				self.index[i] = Some(signer);
				break;
			}
		}
	}

	pub fn signers(&self) -> &[Option<BaseSigner<N, P>>] {
		&self.index
	}

	/// Removes all the None values from the index.
	pub fn normalize(self) -> Self {
		let mut index = [Self::DEFAULT_SIGNER; K];
		let mut i = 0;
		for signer in self.index.iter() {
			if let Some(signer) = signer {
				index[i] = Some(signer.clone());
				i += 1;
			}
		}
		Self { index }
	}
}

impl<const N: usize, const P: usize, const K: usize> Serialize for BaseSignerIndex<N, P, K> {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		let mut written_len = 0;
		for signer in self.index.iter() {
			if let Some(signer) = signer {
				written_len += signer.try_write_to_buffer(&mut buffer[written_len..])?;
			} else {
				written_len += N + P + SystemBufferAddress::BYTES_LENGTH;
			}
		}
		Ok(self.index.len() * (N + P + SystemBufferAddress::BYTES_LENGTH))
	}
}

impl<const N: usize, const P: usize, const K: usize> Deserialize for BaseSignerIndex<N, P, K> {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		let mut index = [Self::DEFAULT_SIGNER; K];
		let mut out_buffer = buffer;
		for i in 0..K {
			if let Ok((remaining_buffer, signer)) =
				BaseSigner::try_from_bytes_with_remaining_buffer(buffer)
			{
				index[i] = Some(signer);
				out_buffer = remaining_buffer;
			}
		}
		Ok((out_buffer, Self { index }))
	}
}

/// All base signers are store signers.
impl<const N: usize, const P: usize, const K: usize> StoreSignerIndex<BaseSigner<N, P>>
	for BaseSignerIndex<N, P, K>
{
	fn index_signers(&self) -> &[Option<BaseSigner<N, P>>] {
		&self.index
	}
}
