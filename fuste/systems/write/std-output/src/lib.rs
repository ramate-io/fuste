use core::ops::ControlFlow;
use fuste_riscv_core::machine::{Machine, MachineError, MachineSystem};
use fuste_std_output::Stdout;

pub struct StdOutputSystem<const MEMORY_SIZE: usize>;

impl<const MEMORY_SIZE: usize> MachineSystem<MEMORY_SIZE> for StdOutputSystem<MEMORY_SIZE> {
	fn tick(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		let write_fd = machine.csrs().registers().get(10);
		let write_buffer_address = machine.csrs().registers().get(11);
		let write_buffer_length = machine.csrs().registers().get(12);
		let write_buffer = machine
			.memory()
			.read_bytes(write_buffer_address, write_buffer_length)
			.map_err(MachineError::MemoryError)?;

		if write_fd == Stdout::to_const_u32() {
			// print the write buffer to stdout
			print!("{}", String::from_utf8_lossy(write_buffer));
			// write 0 to the result register a3
			machine.csrs_mut().registers_mut().set(13, 0);
			machine.csrs_mut().registers_mut().program_counter_mut().increment();
			machine.commit_csrs();
		}

		Ok(ControlFlow::Continue(()))
	}
}
