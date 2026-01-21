#![no_std]

use core::ops::ControlFlow;
use fuste_ecall::Ecall;
use fuste_interrupt_handler::EcallDispatcherOps;
use fuste_riscv_core::{
	instructions::EcallInterrupt,
	machine::{Machine, MachineError, MachineSystem},
};

/// Marker trait for exit system dispatchers.
pub trait ExitSystemDispatcher<const MEMORY_SIZE: usize>: MachineSystem<MEMORY_SIZE> {}

/// Implement ExitSystemDispatcher for Option<T: ExitSystemDispatcher<MEMORY_SIZE>>
impl<const MEMORY_SIZE: usize, T: ExitSystemDispatcher<MEMORY_SIZE>>
	ExitSystemDispatcher<MEMORY_SIZE> for Option<T>
{
}

/// Marker trait for write system dispatchers.
pub trait WriteSystemDispatcher<const MEMORY_SIZE: usize>: MachineSystem<MEMORY_SIZE> {}

/// Implement WriteSystemDispatcher for Option<T: WriteSystemDispatcher<MEMORY_SIZE>>
impl<const MEMORY_SIZE: usize, T: WriteSystemDispatcher<MEMORY_SIZE>>
	WriteSystemDispatcher<MEMORY_SIZE> for Option<T>
{
}

/// Marker trait for write channel system dispatchers.
pub trait OpenChannelSystemDispatcher<const MEMORY_SIZE: usize>:
	MachineSystem<MEMORY_SIZE>
{
}

/// Implement WriteChannelSystemDispatcher for Option<T: WriteChannelSystemDispatcher<MEMORY_SIZE>>
impl<const MEMORY_SIZE: usize, T: OpenChannelSystemDispatcher<MEMORY_SIZE>>
	OpenChannelSystemDispatcher<MEMORY_SIZE> for Option<T>
{
}

/// Marker trait for read channel system dispatchers.
pub trait CheckChannelSystemDispatcher<const MEMORY_SIZE: usize>:
	MachineSystem<MEMORY_SIZE>
{
}

/// Implement ReadChannelSystemDispatcher for Option<T: ReadChannelSystemDispatcher<MEMORY_SIZE>>
impl<const MEMORY_SIZE: usize, T: CheckChannelSystemDispatcher<MEMORY_SIZE>>
	CheckChannelSystemDispatcher<MEMORY_SIZE> for Option<T>
{
}

pub struct NoopDispatcher<const MEMORY_SIZE: usize> {}

impl<const MEMORY_SIZE: usize> MachineSystem<MEMORY_SIZE> for NoopDispatcher<MEMORY_SIZE> {
	fn tick(
		&mut self,
		_machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		Ok(ControlFlow::Continue(()))
	}
}

impl<const MEMORY_SIZE: usize> ExitSystemDispatcher<MEMORY_SIZE> for NoopDispatcher<MEMORY_SIZE> {}
impl<const MEMORY_SIZE: usize> WriteSystemDispatcher<MEMORY_SIZE> for NoopDispatcher<MEMORY_SIZE> {}
impl<const MEMORY_SIZE: usize> OpenChannelSystemDispatcher<MEMORY_SIZE>
	for NoopDispatcher<MEMORY_SIZE>
{
}
impl<const MEMORY_SIZE: usize> CheckChannelSystemDispatcher<MEMORY_SIZE>
	for NoopDispatcher<MEMORY_SIZE>
{
}

/// The [EcallDispatcher] plugin handles ecall interrupts ticking and inner machine then delegating to the appropriate dispatcher.
///
/// The dispatchers are the plugins that will be ticked when the appropriate ecall is encountered.
pub struct EcallDispatcher<
	const MEMORY_SIZE: usize,
	ExitDispatcher: ExitSystemDispatcher<MEMORY_SIZE>,
	WriteDispatcher: WriteSystemDispatcher<MEMORY_SIZE>,
	OpenChannelDispatcher: OpenChannelSystemDispatcher<MEMORY_SIZE>,
	CheckChannelDispatcher: CheckChannelSystemDispatcher<MEMORY_SIZE>,
> {
	pub exit_dispatcher: ExitDispatcher,
	pub write_dispatcher: WriteDispatcher,
	pub open_channel_dispatcher: OpenChannelDispatcher,
	pub check_channel_dispatcher: CheckChannelDispatcher,
}

impl<
		const MEMORY_SIZE: usize,
		ExitDispatcher: ExitSystemDispatcher<MEMORY_SIZE>,
		WriteDispatcher: WriteSystemDispatcher<MEMORY_SIZE>,
		OpenChannelDispatcher: OpenChannelSystemDispatcher<MEMORY_SIZE>,
		CheckChannelDispatcher: CheckChannelSystemDispatcher<MEMORY_SIZE>,
	> MachineSystem<MEMORY_SIZE>
	for EcallDispatcher<
		MEMORY_SIZE,
		ExitDispatcher,
		WriteDispatcher,
		OpenChannelDispatcher,
		CheckChannelDispatcher,
	>
{
	/// Ticks the ecall dispatcher and delegates to the appropriate dispatcher based on the ecall word.
	///
	/// Notice that we don't inline because this shouldn't be called all that often by the machine.
	///
	/// Generally speaking, dispatchers will be called sparingly while handlers will be called frequently.
	/// Hence, handlers should be inlined and dispatchers should not be.
	fn tick(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		let ecall_word = machine.csrs().registers().get(17);
		let ecall = Ecall::try_from_u32(ecall_word)
			.map_err(|_e| MachineError::SystemError("invalid ecall word"))?;

		match ecall {
			Ecall::Exit => self.exit_dispatcher.tick(machine),
			Ecall::Write => self.write_dispatcher.tick(machine),
			Ecall::OpenChannel => self.open_channel_dispatcher.tick(machine),
			Ecall::CheckChannel => self.check_channel_dispatcher.tick(machine),
		}
	}
}

impl<
		const MEMORY_SIZE: usize,
		ExitDispatcher: ExitSystemDispatcher<MEMORY_SIZE>,
		WriteDispatcher: WriteSystemDispatcher<MEMORY_SIZE>,
		OpenChannelDispatcher: OpenChannelSystemDispatcher<MEMORY_SIZE>,
		CheckChannelDispatcher: CheckChannelSystemDispatcher<MEMORY_SIZE>,
	> EcallDispatcherOps<MEMORY_SIZE>
	for EcallDispatcher<
		MEMORY_SIZE,
		ExitDispatcher,
		WriteDispatcher,
		OpenChannelDispatcher,
		CheckChannelDispatcher,
	>
{
	#[inline(always)]
	fn set_ecall_interrupt(&mut self, _interrupt: EcallInterrupt) -> Result<(), MachineError> {
		// currently we don't do anything because we don't need the inner context for any of the sub dispatchers
		// Everything we need is written to the CSRs and the machine will handle the rest
		Ok(())
	}
}
