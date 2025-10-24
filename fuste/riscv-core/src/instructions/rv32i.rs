use crate::instructions::{
	ExecutableInstruction, ExecutableInstructionError, InvalidInstruction, WordInstruction,
};
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
use core::fmt::{self, Display};

#[derive(Debug)]
pub enum Rv32iInstructionError {
	InvalidInstruction(u32),
	WordInstructionError(u32),
}

impl Display for Rv32iInstructionError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Rv32iInstructionError::InvalidInstruction(i) => {
				write!(f, "InvalidInstruction: 0b{:b}", i)
			}
			Rv32iInstructionError::WordInstructionError(word) => {
				write!(f, "WordInstructionError: 0b{:b}", word)
			}
		}
	}
}

/// The reason for instruction not being an enum of different instruction types can be thought of as twofold:
///
/// 1. The overhead on construction of the structs at runtime decoding of words does not suit memory or time performance.
/// 2. In constratined environments, we also risk bloating program size by having decoded structs.
///
/// As a result, pulling out the decoding logic into a match statement is a good compromise.
/// The additional benefit is that the match statement reads like the table which describeds the instruction set.
#[derive(Debug)]
pub enum Rv32iInstruction<const MEMORY_SIZE: usize> {
	Lui(Lui),
	Auipc(Auipc),
	Jal(Jal),
	Jalr(Jalr),
	Beq(Beq),
	Bne(Bne),
	Blt(Blt),
	Bge(Bge),
	Bltu(Bltu),
	Bgeu(Bgeu),
	Lb(Lb),
	Lh(Lh),
	Lw(Lw),
	Lbu(Lbu),
	Lhu(Lhu),
	Sb(Sb),
	Sh(Sh),
	Sw(Sw),
	Addi(Addi),
	Slti(Slti),
	Sltiu(Sltiu),
	Xori(Xori),
	Ori(Ori),
	Andi(Andi),
	Slli(Slli),
	Srli(Srli),
	Srai(Srai),
	Add(Add),
	Sub(Sub),
	Sll(Sll),
	Slt(Slt),
	Sltu(Sltu),
	Xor(Xor),
	Srl(Srl),
	Sra(Sra),
	Or(Or),
	And(And),
	Fence(Fence),
	Ecall(Ecall),
	Ebreak(Ebreak),
}

impl<const MEMORY_SIZE: usize> Rv32iInstruction<MEMORY_SIZE> {
	pub fn from_word(word: u32) -> Result<Self, Rv32iInstructionError> {
		// The opcode is the least significant 7 bits of the word.
		let opcode = word & 0b0000_0000_0000_0000_0000_0000_0111_1111;

		// The main reason for not parsing into structs containing all the information
		// is the overhead on construction of the structs.
		// For semantic clarity, we may change this in the short run, however.
		match opcode {
			// U format is one that doesn't share an opcode throughout its members
			Lui::OPCODE => Ok(Rv32iInstruction::Lui(Lui::from_word(word))),
			Auipc::OPCODE => Ok(Rv32iInstruction::Auipc(Auipc::from_word(word))),
			// J format is just JAL
			Jal::OPCODE => Ok(Rv32iInstruction::Jal(Jal::from_word(word))),
			// JALR has its own opcode
			Jalr::OPCODE => Ok(Rv32iInstruction::Jalr(Jalr::from_word(word))),
			// B format shares an opcode
			B::OPCODE => {
				// For B-type instructions, we need to check funct3
				let b = base::b::B::from_word(word);
				match b.funct3() {
					Beq::FUNCT3 => Ok(Rv32iInstruction::Beq(Beq::new(b))),
					Bne::FUNCT3 => Ok(Rv32iInstruction::Bne(Bne::new(b))),
					Blt::FUNCT3 => Ok(Rv32iInstruction::Blt(Blt::new(b))),
					Bge::FUNCT3 => Ok(Rv32iInstruction::Bge(Bge::new(b))),
					Bltu::FUNCT3 => Ok(Rv32iInstruction::Bltu(Bltu::new(b))),
					Bgeu::FUNCT3 => Ok(Rv32iInstruction::Bgeu(Bgeu::new(b))),
					_ => Err(Rv32iInstructionError::InvalidInstruction(word)),
				}
			}
			// Load instructions share an opcode
			Lw::OPCODE => {
				// For load instructions, we need to check funct3
				let i = base::i::I::from_word(word);
				match i.funct3() {
					Lb::FUNCT3 => Ok(Rv32iInstruction::Lb(Lb::new(i))),
					Lh::FUNCT3 => Ok(Rv32iInstruction::Lh(Lh::new(i))),
					Lw::FUNCT3 => Ok(Rv32iInstruction::Lw(Lw::new(i))),
					Lbu::FUNCT3 => Ok(Rv32iInstruction::Lbu(Lbu::new(i))),
					Lhu::FUNCT3 => Ok(Rv32iInstruction::Lhu(Lhu::new(i))),
					_ => Err(Rv32iInstructionError::InvalidInstruction(word)),
				}
			}
			// Store instructions share an opcode
			S::OPCODE => {
				let s = base::s::S::from_word(word);
				match s.funct3() {
					Sb::FUNCT3 => Ok(Rv32iInstruction::Sb(Sb::new(s))),
					Sh::FUNCT3 => Ok(Rv32iInstruction::Sh(Sh::new(s))),
					Sw::FUNCT3 => Ok(Rv32iInstruction::Sw(Sw::new(s))),
					_ => Err(Rv32iInstructionError::InvalidInstruction(word)),
				}
			}
			// I format shares an opcode
			I::OPCODE => {
				// For I-type instructions, we need to check funct3
				let i = base::i::I::from_word(word);
				match i.funct3() {
					Addi::FUNCT3 => Ok(Rv32iInstruction::Addi(Addi::new(i))),
					Slti::FUNCT3 => Ok(Rv32iInstruction::Slti(Slti::new(i))),
					Sltiu::FUNCT3 => Ok(Rv32iInstruction::Sltiu(Sltiu::new(i))),
					Xori::FUNCT3 => Ok(Rv32iInstruction::Xori(Xori::new(i))),
					Ori::FUNCT3 => Ok(Rv32iInstruction::Ori(Ori::new(i))),
					Andi::FUNCT3 => Ok(Rv32iInstruction::Andi(Andi::new(i))),
					Slli::FUNCT3 => Ok(Rv32iInstruction::Slli(Slli::new(i))),
					// For SRLI and SRAI, both have funct3=101, distinguished by funct7
					Srli::FUNCT3 => {
						// Check if it's SRAI (funct7=0100000) or SRLI (funct7=0000000)
						// For I format, funct7 is in bits [31:25] of the immediate field
						match i.funct7() {
							Srai::FUNCT7 => Ok(Rv32iInstruction::Srai(Srai::new(i))),
							Srl::FUNCT7 => Ok(Rv32iInstruction::Srli(Srli::new(i))),
							_ => Err(Rv32iInstructionError::InvalidInstruction(word)),
						}
					}
					_ => Err(Rv32iInstructionError::InvalidInstruction(word)),
				}
			}
			// R format shares an opcode
			R::OPCODE => {
				// For R-type instructions, we need to check funct3 and funct7
				let r = base::r::R::from_word(word);
				match (r.funct3(), r.funct7()) {
					(Add::FUNCT3, Add::FUNCT7) => Ok(Rv32iInstruction::Add(Add::new(r))),
					(Sub::FUNCT3, Sub::FUNCT7) => Ok(Rv32iInstruction::Sub(Sub::new(r))),
					(Sll::FUNCT3, Sll::FUNCT7) => Ok(Rv32iInstruction::Sll(Sll::new(r))),
					(Slt::FUNCT3, Slt::FUNCT7) => Ok(Rv32iInstruction::Slt(Slt::new(r))),
					(Sltu::FUNCT3, Sltu::FUNCT7) => Ok(Rv32iInstruction::Sltu(Sltu::new(r))),
					(Xor::FUNCT3, Xor::FUNCT7) => Ok(Rv32iInstruction::Xor(Xor::new(r))),
					(Srl::FUNCT3, Srl::FUNCT7) => Ok(Rv32iInstruction::Srl(Srl::new(r))),
					(Sra::FUNCT3, Sra::FUNCT7) => Ok(Rv32iInstruction::Sra(Sra::new(r))),
					(Or::FUNCT3, Or::FUNCT7) => Ok(Rv32iInstruction::Or(Or::new(r))),
					(And::FUNCT3, And::FUNCT7) => Ok(Rv32iInstruction::And(And::new(r))),
					_ => Err(Rv32iInstructionError::InvalidInstruction(word)),
				}
			}
			// Fence has its own opcode
			Fence::OPCODE => Ok(Rv32iInstruction::Fence(Fence::from_word(word))),
			// Environment instructions have their own structure
			Ecall::OPCODE => {
				let i = base::i::I::from_word(word);
				match i.imm() {
					Ecall::IMM => Ok(Rv32iInstruction::Ecall(Ecall::new(i))),
					Ebreak::IMM => Ok(Rv32iInstruction::Ebreak(Ebreak::new(i))),
					_ => Err(Rv32iInstructionError::InvalidInstruction(word)),
				}
			}
			_ => Err(Rv32iInstructionError::InvalidInstruction(word)),
		}
	}

	pub fn to_word(self) -> u32 {
		match self {
			Rv32iInstruction::Lui(lui) => lui.to_word(),
			Rv32iInstruction::Auipc(auipc) => auipc.to_word(),
			Rv32iInstruction::Jal(jal) => jal.to_word(),
			Rv32iInstruction::Jalr(jalr) => jalr.to_word(),
			Rv32iInstruction::Beq(beq) => beq.to_word(),
			Rv32iInstruction::Bne(bne) => bne.to_word(),
			Rv32iInstruction::Blt(blt) => blt.to_word(),
			Rv32iInstruction::Bge(bge) => bge.to_word(),
			Rv32iInstruction::Bltu(bltu) => bltu.to_word(),
			Rv32iInstruction::Bgeu(bgeu) => bgeu.to_word(),
			Rv32iInstruction::Lb(lb) => lb.to_word(),
			Rv32iInstruction::Lh(lh) => lh.to_word(),
			Rv32iInstruction::Lw(lw) => lw.to_word(),
			Rv32iInstruction::Lbu(lbu) => lbu.to_word(),
			Rv32iInstruction::Lhu(lhu) => lhu.to_word(),
			Rv32iInstruction::Sb(sb) => sb.to_word(),
			Rv32iInstruction::Sh(sh) => sh.to_word(),
			Rv32iInstruction::Sw(sw) => sw.to_word(),
			Rv32iInstruction::Addi(addi) => addi.to_word(),
			Rv32iInstruction::Slti(slti) => slti.to_word(),
			Rv32iInstruction::Sltiu(sltiu) => sltiu.to_word(),
			Rv32iInstruction::Xori(xori) => xori.to_word(),
			Rv32iInstruction::Ori(ori) => ori.to_word(),
			Rv32iInstruction::Andi(andi) => andi.to_word(),
			Rv32iInstruction::Slli(slli) => slli.to_word(),
			Rv32iInstruction::Srli(srli) => srli.to_word(),
			Rv32iInstruction::Srai(srai) => srai.to_word(),
			Rv32iInstruction::Add(add) => add.to_word(),
			Rv32iInstruction::Sub(sub) => sub.to_word(),
			Rv32iInstruction::Sll(sll) => sll.to_word(),
			Rv32iInstruction::Slt(slt) => slt.to_word(),
			Rv32iInstruction::Sltu(sltu) => sltu.to_word(),
			Rv32iInstruction::Xor(xor) => xor.to_word(),
			Rv32iInstruction::Srl(srl) => srl.to_word(),
			Rv32iInstruction::Sra(sra) => sra.to_word(),
			Rv32iInstruction::Or(or) => or.to_word(),
			Rv32iInstruction::And(and) => and.to_word(),
			Rv32iInstruction::Fence(fence) => fence.to_word(),
			Rv32iInstruction::Ecall(ecall) => ecall.to_word(),
			Rv32iInstruction::Ebreak(ebreak) => ebreak.to_word(),
		}
	}

	pub fn execute(
		self,
		address: u32,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<(), ExecutableInstructionError> {
		match self {
			Rv32iInstruction::Lui(lui) => lui.execute(machine),
			Rv32iInstruction::Auipc(auipc) => auipc.execute(machine),
			Rv32iInstruction::Jal(jal) => jal.execute(machine),
			Rv32iInstruction::Jalr(jalr) => jalr.execute(machine),
			Rv32iInstruction::Beq(beq) => beq.execute(machine),
			Rv32iInstruction::Bne(bne) => bne.execute(machine),
			Rv32iInstruction::Blt(blt) => blt.execute(machine),
			Rv32iInstruction::Bge(bge) => bge.execute(machine),
			Rv32iInstruction::Bltu(bltu) => bltu.execute(machine),
			Rv32iInstruction::Bgeu(bgeu) => bgeu.execute(machine),
			Rv32iInstruction::Lb(lb) => lb.execute(machine),
			Rv32iInstruction::Lh(lh) => lh.execute(machine),
			Rv32iInstruction::Lw(lw) => lw.execute(machine),
			Rv32iInstruction::Lbu(lbu) => lbu.execute(machine),
			Rv32iInstruction::Lhu(lhu) => lhu.execute(machine),
			Rv32iInstruction::Sb(sb) => sb.execute(machine),
			Rv32iInstruction::Sh(sh) => sh.execute(machine),
			Rv32iInstruction::Sw(sw) => sw.execute(machine),
			Rv32iInstruction::Addi(addi) => addi.execute(machine),
			Rv32iInstruction::Slti(slti) => slti.execute(machine),
			Rv32iInstruction::Sltiu(sltiu) => sltiu.execute(machine),
			Rv32iInstruction::Xori(xori) => xori.execute(machine),
			Rv32iInstruction::Ori(ori) => ori.execute(machine),
			Rv32iInstruction::Andi(andi) => andi.execute(machine),
			Rv32iInstruction::Slli(slli) => slli.execute(machine),
			Rv32iInstruction::Srli(srli) => srli.execute(machine),
			Rv32iInstruction::Srai(srai) => srai.execute(machine),
			Rv32iInstruction::Add(add) => add.execute(machine),
			Rv32iInstruction::Sub(sub) => sub.execute(machine),
			Rv32iInstruction::Sll(sll) => sll.execute(machine),
			Rv32iInstruction::Slt(slt) => slt.execute(machine),
			Rv32iInstruction::Sltu(sltu) => sltu.execute(machine),
			Rv32iInstruction::Xor(xor) => xor.execute(machine),
			Rv32iInstruction::Srl(srl) => srl.execute(machine),
			Rv32iInstruction::Sra(sra) => sra.execute(machine),
			Rv32iInstruction::Or(or) => or.execute(machine),
			Rv32iInstruction::And(and) => and.execute(machine),
			Rv32iInstruction::Fence(fence) => fence.execute(machine),
			Rv32iInstruction::Ecall(ecall) => ecall.execute(machine),
			Rv32iInstruction::Ebreak(ebreak) => ebreak.execute(machine),
			_ => Err(ExecutableInstructionError::InvalidInstruction(InvalidInstruction {
				word: self.to_word(),
				address,
			})),
		}
	}
}
