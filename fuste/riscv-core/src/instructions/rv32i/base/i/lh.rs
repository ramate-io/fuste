use super::I;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;
use core::fmt::{self, Display};

/// LH: Load Halfword.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Lh(I);

impl Lh {
	pub const OPCODE: u32 = 0b0000011;
	pub const FUNCT3: u8 = 0b001;
	pub const INSTRUCTION_NAME: &'static str = "lh";

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

impl Display for Lh {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} x{}, {}(x{})", Self::INSTRUCTION_NAME, self.rd(), self.imm(), self.rs1())
	}
}

impl WordInstruction for Lh {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(I::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Lh {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let I { rd, rs1, imm, .. } = self.0;

		// Get base address from source register
		let base_addr = machine.registers().get(rs1 as usize);

		// Calculate effective address
		let eff_addr = base_addr.wrapping_add(imm as u32);

		// Load halfword from memory (2 bytes, little-endian)
		let byte0 = machine.memory().read_byte(eff_addr)?;
		let byte1 = machine.memory().read_byte(eff_addr + 1)?;
		let halfword_value = (byte1 as u16) << 8 | (byte0 as u16);

		let registers = machine.registers_mut();

		// Sign extend the halfword to 32 bits
		let value = if (halfword_value & 0x8000) != 0 {
			// Negative halfword, sign extend
			halfword_value as u32 | 0xFFFF0000
		} else {
			// Positive halfword
			halfword_value as u32
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
	fn test_lh_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up base address register
		machine.registers_mut().set(1, 0x100);

		// Set up memory at address 0x100
		machine.memory_mut().write_byte(0x100, 0x34)?;
		machine.memory_mut().write_byte(0x101, 0x12)?;

		let instruction = Lh::new(I::new(2, 0b001, 1, 0)); // load from address 0x100
		instruction.execute(&mut machine)?;

		// Check loaded value (positive halfword)
		assert_eq!(machine.registers().get(2), 0x1234);

		// Check PC was incremented by 4
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_lh_with_negative_halfword() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up base address register
		machine.registers_mut().set(1, 0x100);

		// Set up memory at address 0x100 with negative halfword
		machine.memory_mut().write_byte(0x100, 0x00)?;
		machine.memory_mut().write_byte(0x101, 0x80)?; // -32768 in two's complement

		let instruction = Lh::new(I::new(2, 0b001, 1, 0)); // load from address 0x100
		instruction.execute(&mut machine)?;

		// Check loaded value (sign extended)
		assert_eq!(machine.registers().get(2), 0xFFFF8000);

		Ok(())
	}

	#[test]
	fn test_lh_with_offset() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up base address register
		machine.registers_mut().set(1, 0x100);

		// Set up memory at address 0x104
		machine.memory_mut().write_byte(0x104, 0xCD)?;
		machine.memory_mut().write_byte(0x105, 0xAB)?;

		let instruction = Lh::new(I::new(2, 0b001, 1, 4)); // load from address 0x100 + 4 = 0x104
		instruction.execute(&mut machine)?;

		// Check loaded value (negative halfword, sign extended)
		assert_eq!(machine.registers().get(2), 0xFFFFABCD);

		Ok(())
	}
}
