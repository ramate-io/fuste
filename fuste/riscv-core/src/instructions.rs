use crate::machine::Machine;
pub mod rv32i;
use rv32i::base::j::jal::Jal;
use rv32i::base::r::{add::Add, R};
use rv32i::base::u::{auipc::Auipc, lui::Lui};

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
		// The opcode is the least significant 7 bits of the word.
		let opcode = word & 0b0000_0000_0000_0000_0000_0000_0111_1111;

		// The main reason for not parsing into structs containing all the information
		// is the overhead on construction of the structs.
		// For semantic clarity, we may change this in the short run, however.
		match opcode {
			// U format is one that doesn't share an opcode throughout its members
			Lui::OPCODE => Lui::load_and_execute(word, machine),
			Auipc::OPCODE => Auipc::load_and_execute(word, machine),
			// J format is just JAL
			Jal::OPCODE => Jal::load_and_execute(word, machine),
			// R format shares an opcode
			R::OPCODE => {
				// For R-type instructions, we need to check funct3 and funct7
				let r = rv32i::base::r::R::from_word(word);
				match (r.funct3(), r.funct7()) {
					(Add::FUNCT3, Add::FUNCT7) => Add::new(r).execute(machine),
					_ => Err(ExecutableInstructionError::InvalidInstruction),
				}
			}
			_ => Err(ExecutableInstructionError::InvalidInstruction),
		}
	}
}
