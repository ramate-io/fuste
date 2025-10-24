use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, InvalidInstruction};
use crate::machine::Machine;
pub use base::b::{beq::Beq, bge::Bge, bgeu::Bgeu, blt::Blt, bltu::Bltu, bne::Bne, B};
pub use base::i::{
	addi::Addi, andi::Andi, ebreak::Ebreak, ecall::Ecall, fence::Fence, jalr::Jalr, lb::Lb,
	lbu::Lbu, lh::Lh, lhu::Lhu, lw::Lw, ori::Ori, slli::Slli, slti::Slti, sltiu::Sltiu, srai::Srai,
	srli::Srli, xori::Xori, I,
};
pub use base::j::{jal::Jal, J};
pub use base::r::{
	add::Add, and::And, or::Or, sll::Sll, slt::Slt, sltu::Sltu, sra::Sra, srl::Srl, sub::Sub,
	xor::Xor, R,
};
pub use base::s::{sb::Sb, sh::Sh, sw::Sw, S};
pub use base::u::{auipc::Auipc, lui::Lui};
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
		address: u32,
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
			// JALR has its own opcode
			Jalr::OPCODE => Jalr::load_and_execute(word, machine),
			// B format shares an opcode
			B::OPCODE => {
				// For B-type instructions, we need to check funct3
				let b = base::b::B::from_word(word);
				match b.funct3() {
					Beq::FUNCT3 => Beq::new(b).execute(machine),
					Bne::FUNCT3 => Bne::new(b).execute(machine),
					Blt::FUNCT3 => Blt::new(b).execute(machine),
					Bge::FUNCT3 => Bge::new(b).execute(machine),
					Bltu::FUNCT3 => Bltu::new(b).execute(machine),
					Bgeu::FUNCT3 => Bgeu::new(b).execute(machine),
					_ => Err(ExecutableInstructionError::InvalidInstruction(InvalidInstruction {
						word,
						address,
					})),
				}
			}
			// Load instructions share an opcode
			Lw::OPCODE => {
				// For load instructions, we need to check funct3
				let i = base::i::I::from_word(word);
				match i.funct3() {
					Lb::FUNCT3 => Lb::new(i).execute(machine),
					Lh::FUNCT3 => Lh::new(i).execute(machine),
					Lw::FUNCT3 => Lw::new(i).execute(machine),
					Lbu::FUNCT3 => Lbu::new(i).execute(machine),
					Lhu::FUNCT3 => Lhu::new(i).execute(machine),
					_ => Err(ExecutableInstructionError::InvalidInstruction(InvalidInstruction {
						word,
						address,
					})),
				}
			}
			// Store instructions share an opcode
			S::OPCODE => {
				let s = base::s::S::from_word(word);
				match s.funct3() {
					Sb::FUNCT3 => Sb::new(s).execute(machine),
					Sh::FUNCT3 => Sh::new(s).execute(machine),
					Sw::FUNCT3 => Sw::new(s).execute(machine),
					_ => Err(ExecutableInstructionError::InvalidInstruction(InvalidInstruction {
						word,
						address,
					})),
				}
			}
			// I format shares an opcode
			I::OPCODE => {
				// For I-type instructions, we need to check funct3
				let i = base::i::I::from_word(word);
				match i.funct3() {
					Addi::FUNCT3 => Addi::new(i).execute(machine),
					Slti::FUNCT3 => Slti::new(i).execute(machine),
					Sltiu::FUNCT3 => Sltiu::new(i).execute(machine),
					Xori::FUNCT3 => Xori::new(i).execute(machine),
					Ori::FUNCT3 => Ori::new(i).execute(machine),
					Andi::FUNCT3 => Andi::new(i).execute(machine),
					Slli::FUNCT3 => Slli::new(i).execute(machine),
					// For SRLI and SRAI, both have funct3=101, distinguished by funct7
					Srli::FUNCT3 => {
						// Check if it's SRAI (funct7=0100000) or SRLI (funct7=0000000)
						// For I format, funct7 is in bits [31:25] of the immediate field
						match i.funct7() {
							Srai::FUNCT7 => Srai::new(i).execute(machine),
							Srl::FUNCT7 => Srli::new(i).execute(machine),
							_ => Err(ExecutableInstructionError::InvalidInstruction(
								InvalidInstruction { word, address },
							)),
						}
					}
					_ => Err(ExecutableInstructionError::InvalidInstruction(InvalidInstruction {
						word,
						address,
					})),
				}
			}
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
					_ => Err(ExecutableInstructionError::InvalidInstruction(InvalidInstruction {
						word,
						address,
					})),
				}
			}
			// Fence has its own opcode
			Fence::OPCODE => Fence::load_and_execute(word, machine),
			// Environment instructions have their own structure
			Ecall::OPCODE => {
				let i = base::i::I::from_word(word);
				match i.imm() {
					Ecall::IMM => Ecall::new(i).execute(machine),
					Ebreak::IMM => Ebreak::new(i).execute(machine),
					_ => Err(ExecutableInstructionError::InvalidInstruction(InvalidInstruction {
						word,
						address,
					})),
				}
			}
			_ => Err(ExecutableInstructionError::InvalidInstruction(InvalidInstruction {
				word,
				address,
			})),
		}
	}
}
