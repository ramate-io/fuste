use fuste_channel::ChannelSystemId;
use fuste_serial_channel::{Deserialize, SerialChannelError, Serialize};
use fuste_std_transaction::signer::TransactionSigner;

/// Anything that's a signer index must be able to be converted to and from a transaction signer index.
pub trait SignerIndex<const N: usize, const P: usize, const K: usize>:
	TryInto<TransactionSignerIndex<N, P, K>>
{
	const CHANNEL_SYSTEM_ID: ChannelSystemId;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransactionSignerIndex<const N: usize, const P: usize, const K: usize> {
	signers: [Option<TransactionSigner<N, P>>; K],
}

impl<const N: usize, const P: usize, const K: usize> TransactionSignerIndex<N, P, K> {
	const DEFAULT_SIGNER: Option<TransactionSigner<N, P>> = None;

	pub fn new(signers: [Option<TransactionSigner<N, P>>; K]) -> Self {
		Self { signers }
	}

	pub fn signers(&self) -> &[Option<TransactionSigner<N, P>>] {
		&self.signers
	}
}

impl<const N: usize, const P: usize, const K: usize> Serialize for TransactionSignerIndex<N, P, K> {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		if buffer.len() < K * (N + P) {
			return Err(SerialChannelError::SerializedBufferTooSmall((K * (N + P)) as u32));
		}

		for i in 0..K {
			let signer = &self.signers[i];
			if let Some(signer) = signer {
				signer.try_write_to_buffer(&mut buffer[i * (N + P)..i * (N + P) + N + P])?;
			} else {
				// write zeros
				buffer[i * (N + P)..i * (N + P) + N + P].fill(0);
			}
		}

		Ok(K * (N + P))
	}
}

impl<const N: usize, const P: usize, const K: usize> Deserialize
	for TransactionSignerIndex<N, P, K>
{
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		let mut signers = [Self::DEFAULT_SIGNER; K];

		// Support buffer casting s.t. a smaller buffer can still be used to deserialize into a signer index of gthe size.
		// In particular, this makes communication on the channel more flexible without heap allocations.
		let mut i = 0;
		while let Some(bytes) = buffer.get(i * (N + P)..i * (N + P) + N + P) {
			let (_remaining_buffer, signer) =
				TransactionSigner::<N, P>::try_from_bytes_with_remaining_buffer(bytes)?;

			if signer == TransactionSigner::<N, P>::DEFAULT_SIGNER {
				signers[i] = None;
			} else {
				signers[i] = Some(signer);
			}

			i += 1;
		}

		Ok((&buffer[K * (N + P)..], Self { signers }))
	}
}

/// Transaction signer is a signer index.
impl<const N: usize, const P: usize, const K: usize> SignerIndex<N, P, K>
	for TransactionSignerIndex<N, P, K>
{
	const CHANNEL_SYSTEM_ID: ChannelSystemId = ChannelSystemId::constant(0x516d);
}

impl<const N: usize, const P: usize, const L: usize, const K: usize>
	TryFrom<[TransactionSigner<N, P>; L]> for TransactionSignerIndex<N, P, K>
{
	type Error = SerialChannelError;

	fn try_from(transaction_signers: [TransactionSigner<N, P>; L]) -> Result<Self, Self::Error> {
		if L > K {
			return Err(SerialChannelError::SerializedBufferTooSmall(K as u32));
		}

		let mut signers = [Self::DEFAULT_SIGNER; K];
		for i in 0..L {
			signers[i] = Some(transaction_signers[i].clone());
		}
		Ok(Self { signers })
	}
}

impl<const N: usize, const P: usize, const K: usize, const L: usize> SignerIndex<N, P, K>
	for [TransactionSigner<N, P>; L]
{
	const CHANNEL_SYSTEM_ID: ChannelSystemId = ChannelSystemId::constant(0x516d);
}
