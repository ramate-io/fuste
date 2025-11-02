use fuste_riscv_core::machine::{Machine, MachineError, MachinePlugin};
use fuste_std_output::Stdout;

pub struct StdOutputPlugin<const MEMORY_SIZE: usize>;

impl<const MEMORY_SIZE: usize> MachinePlugin<MEMORY_SIZE> for StdOutputPlugin<MEMORY_SIZE> {
	fn tick(&mut self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), MachineError> {
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

		Ok(())
	}
}
