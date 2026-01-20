use crate::{
	containers::{SignerBackendAddress, SignerBackendCache},
	ChannelSubsystem,
};
use fuste_channel::{ChannelError, ChannelStatus, ChannelStatusCode, ChannelSystemStatus};
use fuste_serial_channel::{Deserialize, Serialize};
use fuste_std_transaction::signer::{
	AddressBytes, PublicKeyBytes, TransactionSigner, TransactionSignerAtIndex,
};

pub struct TransactionSignerAtIndexSystem<
	'a,
	const ADDRESS_BYTES: usize,
	const PUBLIC_KEY_BYTES: usize,
> {
	cache: &'a SignerBackendCache,
}

impl<'a, const ADDRESS_BYTES: usize, const PUBLIC_KEY_BYTES: usize>
	TransactionSignerAtIndexSystem<'a, ADDRESS_BYTES, PUBLIC_KEY_BYTES>
{
	pub fn new(cache: &'a SignerBackendCache) -> Self {
		Self { cache }
	}
}

impl<'a, const ADDRESS_BYTES: usize, const PUBLIC_KEY_BYTES: usize> ChannelSubsystem
	for TransactionSignerAtIndexSystem<'a, ADDRESS_BYTES, PUBLIC_KEY_BYTES>
{
	fn handle_subsystem_open(
		&mut self,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError> {
		// Deserialize the transaction signer at index from the read buffer
		let transaction_signer_at_index = TransactionSignerAtIndex::try_from_bytes(read_buffer)
			.map_err(|_| ChannelError::Internal)?;

		// Get the signer at the index from the cache
		let signer = self.cache.at_index(transaction_signer_at_index.index() as usize);
		let signer: TransactionSigner<ADDRESS_BYTES, PUBLIC_KEY_BYTES> = match signer {
			Some(signer) => signer.clone().into(),
			None => TransactionSigner::default(),
		};

		// Serialize the transaction signer to the write buffer
		signer.try_write_to_buffer(write_buffer).map_err(|_| ChannelError::Internal)?;

		Ok(ChannelStatus::new(0, ChannelStatusCode::Success, ChannelSystemStatus::new(0)))
	}

	fn handle_subsystem_check(
		&mut self,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError> {
		self.handle_subsystem_open(read_buffer, write_buffer)
	}
}

impl<const ADDRESS_BYTES: usize, const PUBLIC_KEY_BYTES: usize> From<SignerBackendAddress>
	for TransactionSigner<ADDRESS_BYTES, PUBLIC_KEY_BYTES>
{
	fn from(signer: SignerBackendAddress) -> Self {
		TransactionSigner::new(
			AddressBytes::new(signer.clone().into()),
			PublicKeyBytes::new(signer.clone().into()),
		)
	}
}
