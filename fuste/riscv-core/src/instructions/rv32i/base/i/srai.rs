use super::I;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;
use core::fmt::{self, Display};

/// SRAI: Shift Right Arithmetic Immediate.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Srai(I);

impl Srai {
	pub const OPCODE: u32 = 0b0010011;
	pub const FUNCT3: u8 = 0b101;
	pub const FUNCT7: u8 = 0b0100000; // SRAI is distinguished by funct7
	pub const INSTRUCTION_NAME: &'static str = "srai";

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
	pub fn shamt(&self) -> u8 {
		self.0.shamt()
	}

	#[inline(always)]
	pub fn funct3(&self) -> u8 {
		self.0.funct3()
	}
}

impl Display for Srai {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} x{}, x{}, {}", Self::INSTRUCTION_NAME, self.rd(), self.rs1(), self.shamt())
	}
}

impl WordInstruction for Srai {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(I::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Srai {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let I { rd, rs1, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register value
		let rs1_val = registers.get(rs1 as usize);

		// Perform arithmetic right shift using shamt (only lower 5 bits are used)
		let shift_amount = self.shamt() & 0b11111;
		let result = ((rs1_val as i32) >> shift_amount) as u32;

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
	fn test_srai_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 0b0000_0000_0000_0000_0000_0000_0000_1000); // 8

		let instruction = Srai::new(I::new(2, 0b101, 1, 2)); // shift by 2
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(2), 0b0000_0000_0000_0000_0000_0000_0000_0010); // 8 >> 2 = 2

		// Check PC was incremented by 4
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_srai_with_negative_number() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register with negative number
		machine.registers_mut().set(1, 0x80000000); // Negative number

		let instruction = Srai::new(I::new(2, 0b101, 1, 1)); // shift by 1
		instruction.execute(&mut machine)?;

		// Check result (arithmetic shift preserves sign bit)
		assert_eq!(machine.registers().get(2), 0xC0000000); // 0x80000000 >> 1 = 0xC0000000

		Ok(())
	}

	#[test]
	fn test_srai_from_word() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 0b0000_0000_0000_0000_0000_0000_0000_1000); // 8

		// Create SRAI instruction word: SRAI x2, x1, 2
		let i = I::new(2, 0b101, 1, 2);
		let word = i.to_word(0b0010011);
		let instruction = Srai::from_word(word);
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(2), 0b0000_0000_0000_0000_0000_0000_0000_0010); // 8 >> 2 = 2

		Ok(())
	}
}
