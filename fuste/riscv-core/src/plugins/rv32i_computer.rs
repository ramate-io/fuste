use crate::instructions::Rv32iInstruction;
use crate::machine::Machine;
use crate::machine::MachineError;
use crate::machine::MachinePlugin;
use core::fmt::Write;

/// A macro which takes a list of [WordInsructions] and returns a [u32; n] array of words.
#[macro_export]
macro_rules! program {
	( $( $x:expr ),* ) => {
		[ $( WordInstruction::to_word($x) ),* ]
	};
}

/// The ControlFlowComputer that use the machine to implement a control flow computer.
/// On each tick, it reads the instruction at the program counter and executes it.
///
/// Updates of the program counter are internal to [Instruction]s.
pub struct Rv32iComputer;

impl<const MEMORY_SIZE: usize> MachinePlugin<MEMORY_SIZE> for Rv32iComputer {
	fn tick(&mut self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), MachineError> {
		// get the next instruction
		let program_counter = machine.registers().program_counter();
		let instruction =
			machine.memory().read_word(program_counter).map_err(MachineError::MemoryError)?;

		// write the instruction to the machine log
		let log = machine.log_mut();
		writeln!(log, "0x{:X}: 0b{:b}", program_counter, instruction).unwrap();

		Rv32iInstruction::load_and_execute(program_counter, instruction, machine)
			.map_err(MachineError::InstructionError)?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::instructions::rv32i::{Addi, Blt, Ebreak, Jal, Lw, I};
	use crate::instructions::{ExecutableInstructionError, WordInstruction};

	#[test]
	fn test_rv32i_computer() -> Result<(), MachineError> {
		let mut machine = Machine::<1024>::new();
		let program = program![
			Lw::new(I::new(2, 0b010, 1, 0)),
			Lw::new(I::new(3, 0b010, 2, 0)),
			Lw::new(I::new(4, 0b010, 3, 0)),
			Lw::new(I::new(5, 0b010, 4, 0)),
			Lw::new(I::new(6, 0b010, 5, 0)),
			Lw::new(I::new(7, 0b010, 6, 0)),
			Lw::new(I::new(8, 0b010, 7, 0)),
			Lw::new(I::new(9, 0b010, 8, 0))
		];
		machine
			.memory_mut()
			.load_word_segment(0, &program)
			.map_err(MachineError::MemoryError)?;

		// run the program for the needed ticks

		Ok(())
	}

	#[test]
	fn test_counter_program() -> Result<(), MachineError> {
		let mut machine = Machine::<1024>::new();

		// Program: Increment counter by 2 from 3 to 33
		// x1 = counter (starts at 3)
		// x3 = target value to exceed (31)
		// x4 = loop counter for iterations

		let program = program![
			// Initialize registers
			Addi::of(1, 0, 3),  // x1 = 3 (counter)
			Addi::of(3, 0, 31), // x3 = 31 (target)
			Addi::of(4, 0, 0),  // x4 = 0 (loop counter)
			// Loop: add 2 to counter
			Addi::of(1, 1, 2), // x1 = x1 + 2 (counter += 2)
			Addi::of(4, 4, 1), // x4 = x4 + 1 (increment loop counter)
			// Check if counter < target (x1 < x3) - use BLT (funct3=100)
			Blt::of(3, 1, 8), // if x3 < x1, break loop (branch forward 2 instructions)
			// Jump back to loop start
			Jal::of(2, -12), // jump back to add instruction (4 words back)
			// Ebreak instruction to exit the program
			Ebreak::of(0, 0)
		];

		// Load program into memory
		machine
			.memory_mut()
			.load_word_segment(0, &program)
			.map_err(MachineError::MemoryError)?;

		// Run the program
		let mut computer = Rv32iComputer;

		// Execute enough ticks to complete the program
		// The program should run for about 16 iterations (3, 5, 7, ..., 33)
		for _ in 0..100 {
			match computer.tick(&mut machine) {
				Ok(()) => (),
				Err(MachineError::InstructionError(
					ExecutableInstructionError::EbreakInterrupt(_e),
				)) => {
					break;
				}
				Err(e) => return Err(e),
			}
		}

		// Check final state
		assert_eq!(machine.registers().get(1), 33); // counter should be 33
		assert_eq!(machine.registers().get(4), 15); // loop counter should be 15 (0 to 14 iterations)

		Ok(())
	}
}
