#![no_std]

use fuste_riscv_core::{
	instructions::ExecutableInstructionError,
	machine::{Machine, MachineError, MachinePlugin},
};

pub struct InterruptHandler<
	const MEMORY_SIZE: usize,
	Inner: MachinePlugin<MEMORY_SIZE>,
	EcallHandler: MachinePlugin<MEMORY_SIZE>,
	EbreakHandler: MachinePlugin<MEMORY_SIZE>,
> {
	pub inner: Inner,
	pub ecall_handler: EcallHandler,
	pub ebreak_handler: EbreakHandler,
}

impl<
		const MEMORY_SIZE: usize,
		Inner: MachinePlugin<MEMORY_SIZE>,
		EcallHandler: MachinePlugin<MEMORY_SIZE>,
		EbreakHandler: MachinePlugin<MEMORY_SIZE>,
	> MachinePlugin<MEMORY_SIZE>
	for InterruptHandler<MEMORY_SIZE, Inner, EcallHandler, EbreakHandler>
{
	fn tick(&mut self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), MachineError> {
		match self.inner.tick(machine) {
			Ok(()) => Ok(()),
			Err(MachineError::InstructionError(ExecutableInstructionError::EcallInterrupt(_e))) => {
				self.ecall_handler.tick(machine)
			}
			Err(MachineError::InstructionError(ExecutableInstructionError::EbreakInterrupt(
				_e,
			))) => self.ebreak_handler.tick(machine),
			Err(e) => Err(e),
		}
	}
}
