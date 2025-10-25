use super::I;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;
use core::fmt::{self, Display};

/// LB: Load Byte.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Lb(I);

impl Lb {
	pub const OPCODE: u32 = 0b0000011;
	pub const FUNCT3: u8 = 0b000;
	pub const INSTRUCTION_NAME: &'static str = "lb";

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

impl Display for Lb {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} x{}, {}(x{})", Self::INSTRUCTION_NAME, self.rd(), self.imm(), self.rs1())
	}
}

impl WordInstruction for Lb {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(I::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Lb {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let I { rd, rs1, imm, .. } = self.0;

		// Get base address from source register
		let base_addr = machine.registers().get(rs1 as usize);

		// Calculate effective address
		let eff_addr = base_addr.wrapping_add(imm as u32);

		// Load byte from memory
		let byte_value = machine.memory().read_byte(eff_addr)?;

		let registers = machine.registers_mut();

		// Sign extend the byte to 32 bits
		let value = if (byte_value & 0x80) != 0 {
			// Negative byte, sign extend
			byte_value as u32 | 0xFFFFFF00
		} else {
			// Positive byte
			byte_value as u32
		};

		// Store loaded value in destination register
		registers.set(rd, value);

		// Increment program counter by 4 (word size)
		registers.program_counter_mut().increment();

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_lb_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up base address register
		machine.registers_mut().set(1, 0x100);

		// Set up memory at address 0x100
		machine.memory_mut().write_byte(0x100, 0x42)?;

		let instruction = Lb::new(I::new(2, 0b000, 1, 0)); // load from address 0x100
		instruction.execute(&mut machine)?;

		// Check loaded value (positive byte)
		assert_eq!(machine.registers().get(2), 0x42);

		// Check PC was incremented by 4
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_lb_with_negative_byte() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up base address register
		machine.registers_mut().set(1, 0x100);

		// Set up memory at address 0x100 with negative byte
		machine.memory_mut().write_byte(0x100, 0x80)?; // -128 in two's complement

		let instruction = Lb::new(I::new(2, 0b000, 1, 0)); // load from address 0x100
		instruction.execute(&mut machine)?;

		// Check loaded value (sign extended)
		assert_eq!(machine.registers().get(2), 0xFFFFFF80);

		Ok(())
	}

	#[test]
	fn test_lb_with_offset() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up base address register
		machine.registers_mut().set(1, 0x100);

		// Set up memory at address 0x104
		machine.memory_mut().write_byte(0x104, 0xAB)?;

		let instruction = Lb::new(I::new(2, 0b000, 1, 4)); // load from address 0x100 + 4 = 0x104
		instruction.execute(&mut machine)?;

		// Check loaded value (negative byte, sign extended)
		assert_eq!(machine.registers().get(2), 0xFFFFFFAB);

		Ok(())
	}
}
