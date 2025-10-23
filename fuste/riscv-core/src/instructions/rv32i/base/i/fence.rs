use super::I;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// FENCE: Fence.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Fence(I);

impl Fence {
	pub const OPCODE: u32 = 0b0001111;

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
	pub fn fence_fm(&self) -> u8 {
		self.0.fence_fm()
	}

	#[inline(always)]
	pub fn fence_pi(&self) -> bool {
		self.0.fence_pi()
	}

	#[inline(always)]
	pub fn fence_po(&self) -> bool {
		self.0.fence_po()
	}

	#[inline(always)]
	pub fn fence_pr(&self) -> bool {
		self.0.fence_pr()
	}

	#[inline(always)]
	pub fn fence_pw(&self) -> bool {
		self.0.fence_pw()
	}

	#[inline(always)]
	pub fn fence_si(&self) -> bool {
		self.0.fence_si()
	}

	#[inline(always)]
	pub fn fence_so(&self) -> bool {
		self.0.fence_so()
	}

	#[inline(always)]
	pub fn fence_sr(&self) -> bool {
		self.0.fence_sr()
	}

	#[inline(always)]
	pub fn fence_sw(&self) -> bool {
		self.0.fence_sw()
	}

	#[inline(always)]
	pub fn funct7(&self) -> u8 {
		self.0.funct7()
	}
}

impl WordInstruction for Fence {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(I::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Fence {
	/// Fence instructions are used to synchronize the execution of instructions.
	///
	/// In the synchronous [Machine] this memory ordering is determined by the plugins.
	/// There is no true coprocessing, and all instructions are executed in a reasonable order.
	#[inline(always)]
	fn execute(
		self,
		_machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<(), ExecutableInstructionError> {
		Ok(())
	}
}
