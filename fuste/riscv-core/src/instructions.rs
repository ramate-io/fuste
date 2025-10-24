use crate::machine::memory::MemoryError;
use crate::machine::Machine;
pub mod rv32i;
pub use core::error::Error;
pub use core::fmt::{self, Display};
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
pub struct InvalidInstruction {
	word: u32,
	address: u32,
}

#[derive(Debug, PartialEq)]
pub enum ExecutableInstructionError {
	EbreakExit(EbreakExit),
	InvalidInstruction(InvalidInstruction),
	MemoryError(MemoryError),
}

impl Display for ExecutableInstructionError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ExecutableInstructionError::InvalidInstruction(w) => {
				write!(f, "InvalidInstruction: 0b{:b} at 0x{:X}", w.word, w.address)
			}
			ExecutableInstructionError::EbreakExit(e) => {
				write!(f, "EbreakExit: {:?}", e)
			}
			ExecutableInstructionError::MemoryError(e) => {
				write!(f, "MemoryError: {:?}", e)
			}
		}
	}
}

impl From<MemoryError> for ExecutableInstructionError {
	fn from(error: MemoryError) -> Self {
		ExecutableInstructionError::MemoryError(error)
	}
}
