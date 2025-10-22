use super::I;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// EBREAK: Environment Break.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Ebreak(I);

impl Ebreak {
	pub const OPCODE: u32 = 0b1110011;
	pub const IMM: i32 = 0;

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

	#[inline(always)]
	pub fn to_word(&self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}
}

impl WordInstruction for Ebreak {
	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(I::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Ebreak {
	/// Ebreak instructions are used for privileged operations.
	///
	/// [Machine] does not have a privileged mode, so this instruction is a no-op.
	#[inline(always)]
	fn execute(
		self,
		_machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<(), ExecutableInstructionError> {
		Ok(())
	}
}
