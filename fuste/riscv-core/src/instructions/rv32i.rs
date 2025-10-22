use crate::instructions::{ExecutableInstruction, ExecutableInstructionError};
use crate::machine::Machine;
use base::j::jal::Jal;
use base::r::{
	add::Add, and::And, or::Or, sll::Sll, slt::Slt, sltu::Sltu, sra::Sra, srl::Srl, sub::Sub,
	xor::Xor, R,
};
use base::u::{auipc::Auipc, lui::Lui};
pub mod base;

/// The reason for instruction not being an enum of different instruction types can be thought of as twofold:
///
/// 1. The overhead on construction of the structs at runtime decoding of words does not suit memory or time performance.
/// 2. In constratined environments, we also risk bloating program size by having decoded structs.
///
/// As a result, pulling out the decoding logic into a match statement is a good compromise.
/// The additional benefit is that the match statement reads like the table which describeds the instruction set.
#[derive(Debug)]
pub struct Rv32iInstruction<const MEMORY_SIZE: usize>;

impl<const MEMORY_SIZE: usize> Rv32iInstruction<MEMORY_SIZE> {
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
				let r = base::r::R::from_word(word);
				match (r.funct3(), r.funct7()) {
					(Add::FUNCT3, Add::FUNCT7) => Add::new(r).execute(machine),
					(Sub::FUNCT3, Sub::FUNCT7) => Sub::new(r).execute(machine),
					(Sll::FUNCT3, Sll::FUNCT7) => Sll::new(r).execute(machine),
					(Slt::FUNCT3, Slt::FUNCT7) => Slt::new(r).execute(machine),
					(Sltu::FUNCT3, Sltu::FUNCT7) => Sltu::new(r).execute(machine),
					(Xor::FUNCT3, Xor::FUNCT7) => Xor::new(r).execute(machine),
					(Srl::FUNCT3, Srl::FUNCT7) => Srl::new(r).execute(machine),
					(Sra::FUNCT3, Sra::FUNCT7) => Sra::new(r).execute(machine),
					(Or::FUNCT3, Or::FUNCT7) => Or::new(r).execute(machine),
					(And::FUNCT3, And::FUNCT7) => And::new(r).execute(machine),
					_ => Err(ExecutableInstructionError::InvalidInstruction),
				}
			}
			_ => Err(ExecutableInstructionError::InvalidInstruction),
		}
	}
}
