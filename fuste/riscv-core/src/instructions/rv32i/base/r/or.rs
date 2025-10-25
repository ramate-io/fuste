use super::R;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;
use core::fmt::{self, Display};

/// OR: OR.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Or(R);

impl Or {
	pub const INSTRUCTION_NAME: &'static str = "or";
	pub const OPCODE: u32 = 0b0110011;
	pub const FUNCT3: u8 = 0b110;
	pub const FUNCT7: u8 = 0b0000000;

	#[inline(always)]
	pub fn new(r: R) -> Self {
		Self(r)
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
	pub fn rs2(&self) -> u8 {
		self.0.rs2()
	}

	#[inline(always)]
	pub fn funct3(&self) -> u8 {
		self.0.funct3()
	}

	#[inline(always)]
	pub fn funct7(&self) -> u8 {
		self.0.funct7()
	}
}

impl Display for Or {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} x{}, x{}, {}", Self::INSTRUCTION_NAME, self.rd(), self.rs1(), self.rs2())
	}
}

impl WordInstruction for Or {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(R::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Or {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let R { rd, rs1, rs2, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register values
		let rs1_val = registers.get(rs1 as usize);
		let rs2_val = registers.get(rs2 as usize);

		// Perform OR operation
		let result = rs1_val | rs2_val;

		// Store result in destination register
		registers.set(rd, result);

		// Increment program counter
		registers.program_counter_mut().increment();

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_or_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers
		machine.registers_mut().set(1, 0b1010);
		machine.registers_mut().set(2, 0b1100);

		let instruction = Or::new(R::new(3, 0b110, 1, 2, 0));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 0b1110); // 1010 | 1100 = 1110

		// Check PC was incremented by 4 (word size)
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_or_with_zeros() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers
		machine.registers_mut().set(1, 0);
		machine.registers_mut().set(2, 0);

		let instruction = Or::new(R::new(3, 0b110, 1, 2, 0));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 0); // 0 | 0 = 0

		Ok(())
	}

	#[test]
	fn test_or_with_ones() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers
		machine.registers_mut().set(1, 0xFFFFFFFF);
		machine.registers_mut().set(2, 0xFFFFFFFF);

		let instruction = Or::new(R::new(3, 0b110, 1, 2, 0));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 0xFFFFFFFF); // 0xFFFFFFFF | 0xFFFFFFFF = 0xFFFFFFFF

		Ok(())
	}
}
