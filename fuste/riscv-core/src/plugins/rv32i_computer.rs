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
