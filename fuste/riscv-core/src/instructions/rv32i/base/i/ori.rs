use super::I;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;
use core::fmt::{self, Display};

/// ORI: OR Immediate.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Ori(I);

impl Ori {
	pub const OPCODE: u32 = 0b0010011;
	pub const FUNCT3: u8 = 0b110;
	pub const INSTRUCTION_NAME: &'static str = "ori";

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
}

impl Display for Ori {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} x{}, x{}, {}", Self::INSTRUCTION_NAME, self.rd(), self.rs1(), self.imm())
	}
}

impl WordInstruction for Ori {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(I::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Ori {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let I { rd, rs1, imm, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register value
		let rs1_val = registers.get(rs1 as usize);

		// Perform OR with immediate
		let result = rs1_val | (imm as u32);

		// Store result in destination register
		registers.set(rd, result);

		// Increment program counter by 4 (word size)
		registers.program_counter_mut().increment();

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ori_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 0b1010);

		let instruction = Ori::new(I::new(2, 0b110, 1, 0b1100));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(2), 0b1110); // 1010 | 1100 = 1110

		// Check PC was incremented by 4
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_ori_with_zero() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 0b1010);

		let instruction = Ori::new(I::new(2, 0b110, 1, 0));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(2), 0b1010); // 1010 | 0 = 1010

		Ok(())
	}

	#[test]
	fn test_ori_with_ones() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 0xFFFFFFFF);

		let instruction = Ori::new(I::new(2, 0b110, 1, -1)); // -1 = 0xFFFFFFFF
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(2), 0xFFFFFFFF); // 0xFFFFFFFF | 0xFFFFFFFF = 0xFFFFFFFF

		Ok(())
	}
}
