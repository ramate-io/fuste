#![no_std]

use fuste_ecall::Ecall;
use fuste_riscv_core::machine::{Machine, MachineError, MachinePlugin};

pub struct EcallHandler<
	const MEMORY_SIZE: usize,
	ExitHandler: MachinePlugin<MEMORY_SIZE>,
	WriteHandler: MachinePlugin<MEMORY_SIZE>,
	WriteChannelHandler: MachinePlugin<MEMORY_SIZE>,
	ReadChannelHandler: MachinePlugin<MEMORY_SIZE>,
> {
	pub exit_handler: ExitHandler,
	pub write_handler: WriteHandler,
	pub write_channel_handler: WriteChannelHandler,
	pub read_channel_handler: ReadChannelHandler,
}

impl<
		const MEMORY_SIZE: usize,
		ExitHandler: MachinePlugin<MEMORY_SIZE>,
		WriteHandler: MachinePlugin<MEMORY_SIZE>,
		WriteChannelHandler: MachinePlugin<MEMORY_SIZE>,
		ReadChannelHandler: MachinePlugin<MEMORY_SIZE>,
	> MachinePlugin<MEMORY_SIZE>
	for EcallHandler<MEMORY_SIZE, ExitHandler, WriteHandler, WriteChannelHandler, ReadChannelHandler>
{
	fn tick(&mut self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), MachineError> {
		let ecall_word = machine.csrs().registers().get(17);
		let ecall = Ecall::try_from_u32(ecall_word)
			.map_err(|_e| MachineError::PluginError("invalid ecall word"))?;

		match ecall {
			Ecall::Exit => self.exit_handler.tick(machine),
			Ecall::Write => self.write_handler.tick(machine),
			Ecall::WriteChannel => self.write_channel_handler.tick(machine),
			Ecall::ReadChannel => self.read_channel_handler.tick(machine),
		}
	}
}
