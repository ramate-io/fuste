use crate::machine::Machine;
pub mod rv32i;
use rv32i::base::j::jal::Jal;
use rv32i::base::u::lui::Lui;

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

#[derive(Debug)]
pub struct Instruction<const MEMORY_SIZE: usize>;

impl<const MEMORY_SIZE: usize> Instruction<MEMORY_SIZE> {
	pub fn load_and_execute(
		word: u32,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<(), ExecutableInstructionError> {
		// The opcode is the most significant 7 bits of the word.
		let opcode = (word & 0b11111110000000000000000000000000) >> 27;

		match opcode {
			Lui::OPCODE => Lui::load_and_execute(word, machine),
			Jal::OPCODE => Jal::load_and_execute(word, machine),
			_ => Err(ExecutableInstructionError::InvalidInstruction),
		}
	}
}
