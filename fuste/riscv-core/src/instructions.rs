use crate::machine::memory::MemoryError;
use crate::machine::Machine;
pub mod rv32i;
pub use rv32i::Rv32iInstruction;

pub trait WordInstruction {
	fn to_word(self) -> u32;

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
pub struct EbreakExit {
	address: u32,
	word: u32,
}

#[derive(Debug, PartialEq)]
pub enum ExecutableInstructionError {
	EbreakExit(EbreakExit),
	InvalidInstruction(u32),
	MemoryError(MemoryError),
}

impl From<MemoryError> for ExecutableInstructionError {
	fn from(error: MemoryError) -> Self {
		ExecutableInstructionError::MemoryError(error)
	}
}
