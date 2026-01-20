use crate::signer_stores::{store::SignerStorage, SignerStoreBackend};
use crate::transaction_metadata::transaction_signer_at_index::TransactionSignerAtIndexSystem;
use crate::transaction_metadata::TransactionMetadata;
use crate::{ChannelSubsystem, ChannelSystem};
use core::ops::ControlFlow;
use fuste_channel::{ChannelError, ChannelStatus};
use fuste_ecall_dispatcher::OpenChannelSystemDispatcher;
use fuste_riscv_core::machine::MachineSystem;
use fuste_riscv_core::machine::{Machine, MachineError};
use fuste_std_transaction::signer::TransactionSignerAtIndex;
use fuste_std_transaction::TransactionScheme;

pub struct StdTransaction<
	const ADDRESS_BYTES: usize,
	const PUBLIC_KEY_BYTES: usize,
	const SIGNER_COUNT: usize,
	const TYPE_NAME_BYTES: usize,
	const VALUE_BYTES: usize,
	S: SignerStoreBackend,
> {
	transaction_metadata: TransactionMetadata,
	backend: S,
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
		S: SignerStoreBackend,
	> StdTransaction<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES, S>
{
	pub fn as_signer_storage(
		&self,
	) -> SignerStorage<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES, S>
	{
		SignerStorage::new(
			&self.backend,
			&self.transaction_metadata.hart_index(),
			&self.transaction_metadata.signer_backend_cache(),
		)
	}

	pub fn as_transaction_signer_at_index_system(
		&self,
	) -> TransactionSignerAtIndexSystem<ADDRESS_BYTES, PUBLIC_KEY_BYTES> {
		TransactionSignerAtIndexSystem::new(&self.transaction_metadata.signer_backend_cache())
	}
}

impl<
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
		S: SignerStoreBackend,
	> ChannelSystem
	for StdTransaction<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES, S>
{
	fn handle_check(
		&mut self,
		channel_system_id: fuste_channel::ChannelSystemId,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError> {
		match channel_system_id {
			TransactionSignerAtIndex::CHANNEL_SYSTEM_ID => self
				.as_transaction_signer_at_index_system()
				.handle_subsystem_check(read_buffer, write_buffer),
			_ => Err(ChannelError::invalid_system()),
		}
	}

	fn handle_open(
		&mut self,
		channel_system_id: fuste_channel::ChannelSystemId,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError> {
		match channel_system_id {
			TransactionSignerAtIndex::CHANNEL_SYSTEM_ID => self
				.as_transaction_signer_at_index_system()
				.handle_subsystem_open(read_buffer, write_buffer),
			_ => Err(ChannelError::invalid_system()),
		}
	}
}

impl<
		const MEMORY_SIZE: usize,
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
		S: SignerStoreBackend,
	> MachineSystem<MEMORY_SIZE>
	for StdTransaction<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES, S>
{
	fn tick(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		Ok(ControlFlow::Continue(()))
	}
}

impl<
		const MEMORY_SIZE: usize,
		const ADDRESS_BYTES: usize,
		const PUBLIC_KEY_BYTES: usize,
		const SIGNER_COUNT: usize,
		const TYPE_NAME_BYTES: usize,
		const VALUE_BYTES: usize,
		S: SignerStoreBackend,
	> OpenChannelSystemDispatcher<MEMORY_SIZE>
	for StdTransaction<ADDRESS_BYTES, PUBLIC_KEY_BYTES, SIGNER_COUNT, TYPE_NAME_BYTES, VALUE_BYTES, S>
{
}
