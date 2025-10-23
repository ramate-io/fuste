use super::I;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// SLTIU: Set Less Than Immediate Unsigned.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Sltiu(I);

impl Sltiu {
	pub const OPCODE: u32 = 0b0010011;
	pub const FUNCT3: u8 = 0b011;

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

impl WordInstruction for Sltiu {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(I::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Sltiu {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let I { rd, rs1, imm, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register value
		let rs1_val = registers.get(rs1 as usize);

		// Perform unsigned comparison
		let result = if rs1_val < (imm as u32) { 1 } else { 0 };

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
	fn test_sltiu_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 5);

		let instruction = Sltiu::new(I::new(2, 0b011, 1, 10));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(2), 1); // 5 < 10 = true

		// Check PC was incremented by 4
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_sltiu_with_negative_numbers() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register with negative number (unsigned comparison)
		machine.registers_mut().set(1, 0xFFFFFFFF); // Large unsigned value

		let instruction = Sltiu::new(I::new(2, 0b011, 1, 0));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(2), 0); // 0xFFFFFFFF < 0 = false (unsigned)

		Ok(())
	}

	#[test]
	fn test_sltiu_false() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 15);

		let instruction = Sltiu::new(I::new(2, 0b011, 1, 10));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(2), 0); // 15 < 10 = false

		Ok(())
	}
}
