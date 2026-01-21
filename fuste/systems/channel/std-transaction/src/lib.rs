pub mod containers;
pub mod signer_stores;
pub mod std_transaction;
pub mod transaction_metadata;

use fuste_channel::{ChannelError, ChannelStatus, ChannelSystemId};
use fuste_ecall::Ecall;
use fuste_riscv_core::machine::{Machine, MachineError};

pub trait ChannelSystem {
	fn handle_channel_tick<const MEMORY_SIZE: usize>(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<(), MachineError> {
		// system id is in a0
		let system_id = machine.csrs().registers().get(17);
		let channel_system_id = ChannelSystemId::new(system_id);

		// read buffer is in a1 with length in a2
		let read_buffer_address = machine.csrs().registers().get(18);
		let read_buffer_length = machine.csrs().registers().get(19);

		let write_buffer_address = machine.csrs().registers().get(20);
		let write_buffer_length = machine.csrs().registers().get(21);

		let ecall = machine.csrs().registers().get(17);
		let ecall =
			Ecall::try_from_u32(ecall).map_err(|_e| MachineError::SystemError("invalid ecall"))?;

		// read the read buffer into a dynamic store
		// this is just for debugging environment simplicity
		let read_buffer = machine
			.memory()
			.read_bytes(read_buffer_address, read_buffer_length)
			.map_err(MachineError::MemoryError)?
			.to_vec();

		// write buffer is in a3 with length in a4
		let write_buffer = machine
			.memory_mut()
			.read_bytes_mut(write_buffer_address, write_buffer_length)
			.map_err(MachineError::MemoryError)?;

		let channel_status = match ecall {
			Ecall::OpenChannel => self.handle_open(channel_system_id, &read_buffer, write_buffer),
			Ecall::CheckChannel => self.handle_check(channel_system_id, &read_buffer, write_buffer),
			_ => unreachable!(),
		}
		.map_err(|_e| MachineError::SystemError("channel operation failed"))?;

		// write the channel status code to a5
		machine
			.csrs_mut()
			.registers_mut()
			.set(17, channel_status.code().clone().to_u32());

		// write the size of the buffer written to a4
		machine.csrs_mut().registers_mut().set(18, channel_status.size());

		// write the channel system status to a6
		machine
			.csrs_mut()
			.registers_mut()
			.set(19, channel_status.system_status().clone().to_u32());

		// increment the program counter
		machine.commit_csrs();

		Ok(())
	}

	fn handle_open(
		&mut self,
		channel_system_id: ChannelSystemId,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError>;

	fn handle_check(
		&mut self,
		channel_system_id: ChannelSystemId,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError>;
}

pub trait ChannelSubsystem {
	fn handle_subsystem_open(
		&mut self,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError>;

	fn handle_subsystem_check(
		&mut self,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError>;
}
