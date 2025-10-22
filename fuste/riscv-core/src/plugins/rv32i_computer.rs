use crate::instructions::Rv32iInstruction;
use crate::machine::Machine;
use crate::machine::MachineError;
use crate::machine::MachinePlugin;

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
		let program_counter = machine.registers().program_counter();
		let instruction =
			machine.memory().read_word(program_counter).map_err(MachineError::MemoryError)?;
		Rv32iInstruction::load_and_execute(instruction, machine)
			.map_err(MachineError::InstructionError)?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::instructions::rv32i::{Lw, I};
	use crate::instructions::WordInstruction;

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
}
