use super::I;
use crate::instructions::{
	EbreakExit, ExecutableInstruction, ExecutableInstructionError, WordInstruction,
};
use crate::machine::Machine;
use core::fmt::{self, Display};

/// EBREAK: Environment Break.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Ebreak(I);

impl Ebreak {
	pub const OPCODE: u32 = 0b1110011;
	pub const IMM: i32 = 1;
	pub const FUNCT3: u8 = 0b111;
	pub const RS1: u8 = 0;
	pub const INSTRUCTION_NAME: &'static str = "ebreak";

	#[inline(always)]
	pub fn of(rd: u8, imm: i32) -> Self {
		Self(I::new(rd, Self::FUNCT3, Self::RS1, imm))
	}

	#[inline(always)]
	pub fn new(i: I) -> Self {
		Self(i)
	}

	#[inline(always)]
	pub fn rd(&self) -> u8 {
		self.0.rd()
	}

	#[inline(always)]
	pub fn rs1(&self) -> u8 {
		self.0.rs1()
	}

	#[inline(always)]
	pub fn imm(&self) -> i32 {
		self.0.imm()
	}

	#[inline(always)]
	pub fn funct3(&self) -> u8 {
		self.0.funct3()
	}

	#[inline(always)]
	pub fn funct7(&self) -> u8 {
		self.0.funct7()
	}
}

impl Display for Ebreak {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", Self::INSTRUCTION_NAME)
	}
}

impl WordInstruction for Ebreak {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(I::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Ebreak {
	/// Ebreak simply exits the program with the current address and word.
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		Err(ExecutableInstructionError::EbreakExit(EbreakExit {
			address: machine.registers().program_counter(),
			word: self.to_word(),
		}))
	}
}
