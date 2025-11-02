#![no_std]

use core::ops::ControlFlow;
use fuste_exit::ExitStatus;
use fuste_riscv_core::machine::{Machine, MachineError, MachineSystem};

pub struct ExitSystem<const MEMORY_SIZE: usize> {
	pub syscall_status: ExitStatus,
}

impl<const MEMORY_SIZE: usize> ExitSystem<MEMORY_SIZE> {
	pub fn new() -> Self {
		// Initially this is successful.
		Self { syscall_status: ExitStatus::Success }
	}
}

impl<const MEMORY_SIZE: usize> MachineSystem<MEMORY_SIZE> for ExitSystem<MEMORY_SIZE> {
	fn tick(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		// Assume this has correctly been called by whatever higher order system.
		// We just need to store the status and break.
		let syscall_status_address = machine.csrs().registers().get(10);
		let syscall_status = machine
			.memory()
			.read_word(syscall_status_address)
			.map_err(MachineError::MemoryError)?;
		self.syscall_status = ExitStatus::try_from_u32(syscall_status)
			.map_err(|_e| MachineError::SystemError("invalid exit status"))?;
		Ok(ControlFlow::Break(()))
	}
}
