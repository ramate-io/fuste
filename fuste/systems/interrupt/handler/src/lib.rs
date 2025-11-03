#![no_std]

use core::ops::ControlFlow;
use fuste_riscv_core::{
	instructions::{EbreakInterrupt, EcallInterrupt, ExecutableInstructionError},
	machine::{Machine, MachineError, MachineSystem},
};

/// The [EcallDispatcherOps] trait provides the operations for handling ecall interrupts.
///
/// Specifically, it allows setting the ecall interrupt before ticking the plugin.
pub trait EcallDispatcherOps<const MEMORY_SIZE: usize>: MachineSystem<MEMORY_SIZE> {
	fn set_ecall_interrupt(&mut self, interrupt: EcallInterrupt) -> Result<(), MachineError>;

	fn tick_with_ecall_interrupt(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
		ecall_interrupt: EcallInterrupt,
	) -> Result<ControlFlow<()>, MachineError> {
		self.set_ecall_interrupt(ecall_interrupt)?;
		self.tick(machine)
	}
}

/// The [EbreakDispatcherOps] trait provides the operations for handling ebreak interrupts.
///
/// Specifically, it allows setting the ebreak interrupt before ticking the plugin.
pub trait EbreakDispatcherOps<const MEMORY_SIZE: usize>: MachineSystem<MEMORY_SIZE> {
	fn set_ebreak_interrupt(&mut self, interrupt: EbreakInterrupt) -> Result<(), MachineError>;

	fn tick_with_ebreak_interrupt(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
		ebreak_interrupt: EbreakInterrupt,
	) -> Result<ControlFlow<()>, MachineError> {
		self.set_ebreak_interrupt(ebreak_interrupt)?;
		self.tick(machine)
	}
}

pub struct NoopEcallDispatcher<const MEMORY_SIZE: usize> {}

impl<const MEMORY_SIZE: usize> MachineSystem<MEMORY_SIZE> for NoopEcallDispatcher<MEMORY_SIZE> {
	fn tick(
		&mut self,
		_machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		Ok(ControlFlow::Continue(()))
	}
}

impl<const MEMORY_SIZE: usize> EcallDispatcherOps<MEMORY_SIZE>
	for NoopEcallDispatcher<MEMORY_SIZE>
{
	fn set_ecall_interrupt(&mut self, _interrupt: EcallInterrupt) -> Result<(), MachineError> {
		Ok(())
	}
}

pub struct NoopEbreakDispatcher<const MEMORY_SIZE: usize> {}

impl<const MEMORY_SIZE: usize> MachineSystem<MEMORY_SIZE> for NoopEbreakDispatcher<MEMORY_SIZE> {
	fn tick(
		&mut self,
		_machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		Ok(ControlFlow::Continue(()))
	}
}

impl<const MEMORY_SIZE: usize> EbreakDispatcherOps<MEMORY_SIZE>
	for NoopEbreakDispatcher<MEMORY_SIZE>
{
	fn set_ebreak_interrupt(&mut self, _interrupt: EbreakInterrupt) -> Result<(), MachineError> {
		Ok(())
	}
}

/// The [InterruptHandler] plugin handles interrupts ticking and inner machine then delegating to the appropriate handler.
pub struct InterruptHandler<
	const MEMORY_SIZE: usize,
	Inner: MachineSystem<MEMORY_SIZE>,
	EcallDispatcher: EcallDispatcherOps<MEMORY_SIZE>,
	EbreakDispatcher: EbreakDispatcherOps<MEMORY_SIZE>,
> {
	pub inner: Inner,
	pub ecall_dispatcher: EcallDispatcher,
	pub ebreak_dispatcher: EbreakDispatcher,
}

impl<
		const MEMORY_SIZE: usize,
		Inner: MachineSystem<MEMORY_SIZE>,
		EcallDispatcher: EcallDispatcherOps<MEMORY_SIZE>,
		EbreakDispatcher: EbreakDispatcherOps<MEMORY_SIZE>,
	> MachineSystem<MEMORY_SIZE>
	for InterruptHandler<MEMORY_SIZE, Inner, EcallDispatcher, EbreakDispatcher>
{
	/// Ticks the interrupt handler and delegates to the appropriate handler based on the interrupt type.
	///
	/// We inline because this compositional pattern will most often be used to form a call for every loop.
	/// Thus, this plugin will be called often.
	#[inline(always)]
	fn tick(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		match self.inner.tick(machine) {
			Ok(control_flow) => Ok(control_flow),
			Err(MachineError::InstructionError(ExecutableInstructionError::EcallInterrupt(
				ecall_interrupt,
			))) => self.ecall_dispatcher.tick_with_ecall_interrupt(machine, ecall_interrupt),
			Err(MachineError::InstructionError(ExecutableInstructionError::EbreakInterrupt(
				ebreak_interrupt,
			))) => self.ebreak_dispatcher.tick_with_ebreak_interrupt(machine, ebreak_interrupt),
			Err(e) => Err(e),
		}
	}
}
