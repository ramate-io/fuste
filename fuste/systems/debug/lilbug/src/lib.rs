pub mod exit;

use core::ops::ControlFlow;
use fuste_exit::ExitStatus;
use fuste_riscv_core::instructions::Rv32iInstruction;
use fuste_riscv_core::machine::{Machine, MachineError, MachineSystem};

pub trait LilBugComputer<const MEMORY_SIZE: usize>: MachineSystem<MEMORY_SIZE> {
	fn exit_status(&self) -> ExitStatus;
}

pub struct LilBugSystem<const MEMORY_SIZE: usize, Computer: LilBugComputer<MEMORY_SIZE>> {
	pub computer: Computer,
	pub log_program_counter: bool,
	pub log_registers: bool,
	pub log_instructions: bool,
	pub log_registers_at_end: bool,
	pub log_exit_status: bool,
}

impl<const MEMORY_SIZE: usize, Computer: LilBugComputer<MEMORY_SIZE>> MachineSystem<MEMORY_SIZE>
	for LilBugSystem<MEMORY_SIZE, Computer>
{
	fn tick(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		if self.log_program_counter {
			println!("program counter: 0x{:X}", machine.registers().program_counter());
		}
		if self.log_registers {
			println!("registers: {:?}", machine.registers());
		}

		let address = machine.registers().program_counter();
		let instruction = machine.memory().read_word(address).map_err(MachineError::MemoryError)?;
		if self.log_instructions {
			let decoded_instruction = Rv32iInstruction::<MEMORY_SIZE>::from_word(instruction)
				.map_err(|_e| {
					MachineError::SystemError("Failed to decode instruction for debugger")
				})?;
			println!("0x{address:08X}: {:40} <- 0b{:032b}", decoded_instruction, instruction);
		}
		let control_flow = self.computer.tick(machine)?;

		match control_flow {
			ControlFlow::Continue(()) => (),
			ControlFlow::Break(()) => {
				if self.log_registers_at_end {
					println!("registers at end: {:?}", machine.registers());
				}

				if self.log_exit_status {
					println!("Program exited with status: {:?}", self.computer.exit_status());
				}
			}
		}
		Ok(control_flow)
	}
}
