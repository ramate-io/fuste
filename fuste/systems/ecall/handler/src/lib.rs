#![no_std]

use core::ops::ControlFlow;
use fuste_ecall::Ecall;
use fuste_interrupt_handler::EcallHandlerOps;
use fuste_riscv_core::{
	instructions::EcallInterrupt,
	machine::{Machine, MachineError, MachineSystem},
};

/// Marker trait for exit system handlers.
pub trait ExitSystemHandler<const MEMORY_SIZE: usize>: MachineSystem<MEMORY_SIZE> {}

/// Marker trait for write system handlers.
pub trait WriteSystemHandler<const MEMORY_SIZE: usize>: MachineSystem<MEMORY_SIZE> {}

/// Marker trait for write channel system handlers.
pub trait WriteChannelSystemHandler<const MEMORY_SIZE: usize>: MachineSystem<MEMORY_SIZE> {}

/// Marker trait for read channel system handlers.
pub trait ReadChannelSystemHandler<const MEMORY_SIZE: usize>: MachineSystem<MEMORY_SIZE> {}

/// The [EcallHandler] plugin handles ecall interrupts ticking and inner machine then delegating to the appropriate handler.
///
/// The handlers are the plugins that will be ticked when the appropriate ecall is encountered.
pub struct EcallHandler<
	const MEMORY_SIZE: usize,
	ExitHandler: ExitSystemHandler<MEMORY_SIZE>,
	WriteHandler: WriteSystemHandler<MEMORY_SIZE>,
	WriteChannelHandler: WriteChannelSystemHandler<MEMORY_SIZE>,
	ReadChannelHandler: ReadChannelSystemHandler<MEMORY_SIZE>,
> {
	pub exit_handler: ExitHandler,
	pub write_handler: WriteHandler,
	pub write_channel_handler: WriteChannelHandler,
	pub read_channel_handler: ReadChannelHandler,
}

impl<
		const MEMORY_SIZE: usize,
		ExitHandler: ExitSystemHandler<MEMORY_SIZE>,
		WriteHandler: WriteSystemHandler<MEMORY_SIZE>,
		WriteChannelHandler: WriteChannelSystemHandler<MEMORY_SIZE>,
		ReadChannelHandler: ReadChannelSystemHandler<MEMORY_SIZE>,
	> MachineSystem<MEMORY_SIZE>
	for EcallHandler<MEMORY_SIZE, ExitHandler, WriteHandler, WriteChannelHandler, ReadChannelHandler>
{
	/// Ticks the ecall handler and delegates to the appropriate handler based on the ecall word.
	///
	/// Notice that we don't inline because this shouldn't be called all that often by the machine.
	fn tick(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		let ecall_word = machine.csrs().registers().get(17);
		let ecall = Ecall::try_from_u32(ecall_word)
			.map_err(|_e| MachineError::SystemError("invalid ecall word"))?;

		match ecall {
			Ecall::Exit => self.exit_handler.tick(machine),
			Ecall::Write => self.write_handler.tick(machine),
			Ecall::WriteChannel => self.write_channel_handler.tick(machine),
			Ecall::ReadChannel => self.read_channel_handler.tick(machine),
		}
	}
}

impl<
		const MEMORY_SIZE: usize,
		ExitHandler: ExitSystemHandler<MEMORY_SIZE>,
		WriteHandler: WriteSystemHandler<MEMORY_SIZE>,
		WriteChannelHandler: WriteChannelSystemHandler<MEMORY_SIZE>,
		ReadChannelHandler: ReadChannelSystemHandler<MEMORY_SIZE>,
	> EcallHandlerOps<MEMORY_SIZE>
	for EcallHandler<MEMORY_SIZE, ExitHandler, WriteHandler, WriteChannelHandler, ReadChannelHandler>
{
	#[inline(always)]
	fn set_ecall_interrupt(&mut self, _interrupt: EcallInterrupt) -> Result<(), MachineError> {
		// currently we don't do anything because we don't need the inner context for any of the sub handlers
		// Everything we need is written to the CSRs and the machine will handle the rest
		Ok(())
	}
}
