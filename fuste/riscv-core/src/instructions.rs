use crate::machine::Machine;
pub mod rv32i;
pub use rv32i::Rv32iInstruction;
pub trait ParseableInstruction {
	fn from_word(word: u32) -> Self;
}

pub trait WordInstruction {
	fn from_word(word: u32) -> Self;
}

pub trait ExecutableInstruction<const MEMORY_SIZE: usize>: Sized + WordInstruction {
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError>;

	fn load_and_execute(
		word: u32,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<(), ExecutableInstructionError> {
		let instruction = Self::from_word(word);
		instruction.execute(machine)
	}
}

#[derive(Debug, PartialEq)]
pub enum ExecutableInstructionError {
	InvalidInstruction,
}
