use super::I;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;
use core::fmt::{self, Display};

/// JALR: Jump and Link Register.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Jalr(I);

impl Jalr {
	pub const OPCODE: u32 = 0b1100111;
	pub const FUNCT3: u8 = 0b000;
	pub const INSTRUCTION_NAME: &'static str = "jalr";

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

impl Display for Jalr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} x{}, {}(x{})", Self::INSTRUCTION_NAME, self.rd(), self.imm(), self.rs1())
	}
}

impl WordInstruction for Jalr {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(I::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Jalr {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let I { rd, rs1, imm, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register value
		let rs1_val = registers.get(rs1 as usize);

		// Calculate target address: (rs1 + imm) & ~1 (clear LSB for alignment)
		let target_addr = (rs1_val.wrapping_add(imm as u32)) & !1;

		// Save return address (current PC + 4) in destination register
		let current_pc = registers.program_counter();
		registers.set(rd, current_pc + 4);

		// Set PC to target address
		registers.program_counter_mut().set(target_addr);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_jalr_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 0x1000);

		let instruction = Jalr::new(I::new(2, 0b000, 1, 0x100)); // jump to 0x1000 + 0x100 = 0x1100
		instruction.execute(&mut machine)?;

		// Check return address was saved
		assert_eq!(machine.registers().get(2), 4); // PC was 0, so return address is 4

		// Check PC was set to target address
		assert_eq!(machine.registers().program_counter(), 0x1100);

		Ok(())
	}

	#[test]
	fn test_jalr_with_negative_offset() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 0x1000);

		let instruction = Jalr::new(I::new(2, 0b000, 1, -0x100)); // jump to 0x1000 - 0x100 = 0xF00
		instruction.execute(&mut machine)?;

		// Check return address was saved
		assert_eq!(machine.registers().get(2), 4);

		// Check PC was set to target address
		assert_eq!(machine.registers().program_counter(), 0xF00);

		Ok(())
	}

	#[test]
	fn test_jalr_from_word() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source register
		machine.registers_mut().set(1, 0x2000);

		// Create JALR instruction word: JALR x2, x1, 0x200
		let i = I::new(2, 0b000, 1, 0x200);
		let word = i.to_word(0b1100111);
		let instruction = Jalr::from_word(word);
		instruction.execute(&mut machine)?;

		// Check return address was saved
		assert_eq!(machine.registers().get(2), 4);

		// Check PC was set to target address
		assert_eq!(machine.registers().program_counter(), 0x2200);

		Ok(())
	}
}
