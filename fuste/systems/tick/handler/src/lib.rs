#![no_std]

use core::ops::ControlFlow;
use fuste_riscv_core::{
	instructions::{EbreakInterrupt, EcallInterrupt, ExecutableInstructionError},
	machine::{Machine, MachineError, MachineSystem},
};

/// The [EcallHandlerOps] trait provides the operations for handling ecall interrupts.
///
/// Specifically, it allows setting the ecall interrupt before ticking the plugin.
pub trait EcallHandlerOps<const MEMORY_SIZE: usize>: MachineSystem<MEMORY_SIZE> {
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

/// The [EbreakHandlerOps] trait provides the operations for handling ebreak interrupts.
///
/// Specifically, it allows setting the ebreak interrupt before ticking the plugin.
pub trait EbreakHandlerOps<const MEMORY_SIZE: usize>: MachineSystem<MEMORY_SIZE> {
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

/// The [InterruptHandler] plugin handles interrupts ticking and inner machine then delegating to the appropriate handler.
pub struct InterruptHandler<
	const MEMORY_SIZE: usize,
	Inner: MachineSystem<MEMORY_SIZE>,
	EcallHandler: MachineSystem<MEMORY_SIZE>,
	EbreakHandler: EbreakHandlerOps<MEMORY_SIZE>,
> {
	pub inner: Inner,
	pub ecall_handler: EcallHandler,
	pub ebreak_handler: EbreakHandler,
}

impl<
		const MEMORY_SIZE: usize,
		Inner: MachineSystem<MEMORY_SIZE>,
		EcallHandler: EcallHandlerOps<MEMORY_SIZE>,
		EbreakHandler: EbreakHandlerOps<MEMORY_SIZE>,
	> MachineSystem<MEMORY_SIZE>
	for InterruptHandler<MEMORY_SIZE, Inner, EcallHandler, EbreakHandler>
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
			))) => self.ecall_handler.tick_with_ecall_interrupt(machine, ecall_interrupt),
			Err(MachineError::InstructionError(ExecutableInstructionError::EbreakInterrupt(
				ebreak_interrupt,
			))) => self.ebreak_handler.tick_with_ebreak_interrupt(machine, ebreak_interrupt),
			Err(e) => Err(e),
		}
	}
}
