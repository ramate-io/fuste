use super::I;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// LW: Load Word.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Lw(I);

impl Lw {
	pub const OPCODE: u32 = 0b0000011;
	pub const FUNCT3: u8 = 0b010;

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

impl WordInstruction for Lw {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(I::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Lw {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let I { rd, rs1, imm, .. } = self.0;

		// Get base address from source register
		let base_addr = machine.registers().get(rs1 as usize);

		// Calculate effective address
		let eff_addr = base_addr.wrapping_add(imm as u32);

		// Load word from memory
		let value = machine.memory().read_word(eff_addr)?;

		let registers = machine.registers_mut();

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
	fn test_lw_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up base address register
		machine.registers_mut().set(1, 0x100);

		// Set up memory at address 0x100
		machine.memory_mut().write_word(0x100, 0x12345678)?;

		let instruction = Lw::new(I::new(2, 0b010, 1, 0)); // load from address 0x100
		instruction.execute(&mut machine)?;

		// Check loaded value
		assert_eq!(machine.registers().get(2), 0x12345678);

		// Check PC was incremented by 4
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_lw_with_offset() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up base address register
		machine.registers_mut().set(1, 0x100);

		// Set up memory at address 0x104
		machine.memory_mut().write_word(0x104, 0xDEADBEEF)?;

		let instruction = Lw::new(I::new(2, 0b010, 1, 4)); // load from address 0x100 + 4 = 0x104
		instruction.execute(&mut machine)?;

		// Check loaded value
		assert_eq!(machine.registers().get(2), 0xDEADBEEF);

		Ok(())
	}

	#[test]
	fn test_lw_from_word() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up base address register
		machine.registers_mut().set(1, 0x200);

		// Set up memory at address 0x200
		machine.memory_mut().write_word(0x200, 0xCAFEBABE)?;

		// Create LW instruction word: LW x2, x1, 0
		let i = I::new(2, 0b010, 1, 0);
		let word = i.to_word(0b0000011);
		let instruction = Lw::from_word(word);
		instruction.execute(&mut machine)?;

		// Check loaded value
		assert_eq!(machine.registers().get(2), 0xCAFEBABE);

		Ok(())
	}
}
