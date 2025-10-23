use super::I;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// ADDI: Add Immediate.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Addi(I);

impl Addi {
	pub const OPCODE: u32 = 0b0010011;
	pub const FUNCT3: u8 = 0b000;

	#[inline(always)]
	pub fn of(rd: u8, rs1: u8, imm: i32) -> Self {
		Self(I::new(rd, Self::FUNCT3, rs1, imm))
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
}

impl WordInstruction for Addi {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(I::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Addi {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let I { rd, rs1, imm, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register value
		let rs1_val = registers.get(rs1 as usize);

		// Perform addition with immediate
		let result = rs1_val.wrapping_add(imm as u32);

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
	fn test_addi_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 10);

		let instruction = Addi::new(I::new(2, 0, 1, 5));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(2), 15); // 10 + 5 = 15

		// Check PC was incremented by 4
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_addi_with_negative_immediate() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 10);

		let instruction = Addi::new(I::new(2, 0, 1, -3));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(2), 7); // 10 + (-3) = 7

		Ok(())
	}

	#[test]
	fn test_addi_with_overflow() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register with value that will overflow
		machine.registers_mut().set(1, 0xFFFFFFFF);

		let instruction = Addi::new(I::new(2, 0, 1, 1));
		instruction.execute(&mut machine)?;

		// Check result wraps around
		assert_eq!(machine.registers().get(2), 0); // 0xFFFFFFFF + 1 = 0 (wrapping)

		Ok(())
	}

	#[test]
	fn test_addi_from_word() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 20);

		// Create ADDI instruction word: ADDI x2, x1, 5
		let i = I::new(2, 0, 1, 5);
		let word = i.to_word(0b0010011);
		let instruction = Addi::from_word(word);
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(2), 25); // 20 + 5 = 25

		Ok(())
	}
}
