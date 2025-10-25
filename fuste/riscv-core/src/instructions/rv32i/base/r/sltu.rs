use super::R;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;
use core::fmt::{self, Display};

/// SLTU: Set Less Than Unsigned.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Sltu(R);

impl Sltu {
	pub const INSTRUCTION_NAME: &'static str = "sltu";
	pub const OPCODE: u32 = 0b0110011;
	pub const FUNCT3: u8 = 0b011;
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
}

impl Display for Sltu {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} x{}, x{}, {}", Self::INSTRUCTION_NAME, self.rd(), self.rs1(), self.rs2())
	}
}

impl WordInstruction for Sltu {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(R::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Sltu {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let R { rd, rs1, rs2, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register values
		let rs1_val = registers.get(rs1 as usize);
		let rs2_val = registers.get(rs2 as usize);

		// Perform unsigned comparison
		let result = if rs1_val < rs2_val { 1 } else { 0 };

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
	fn test_sltu_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers
		machine.registers_mut().set(1, 5);
		machine.registers_mut().set(2, 10);

		let instruction = Sltu::new(R::new(3, 0b011, 1, 2, 0));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 1); // 5 < 10 = true

		// Check PC was incremented by 4 (word size)
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_sltu_with_negative_numbers() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers with negative numbers (unsigned comparison)
		machine.registers_mut().set(1, 0xFFFFFFFF); // 0xFFFFFFFF (large unsigned)
		machine.registers_mut().set(2, 0);

		let instruction = Sltu::new(R::new(3, 0b011, 1, 2, 0));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 0); // 0xFFFFFFFF < 0 = false (unsigned)

		Ok(())
	}

	#[test]
	fn test_sltu_equal_values() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers with equal values
		machine.registers_mut().set(1, 5);
		machine.registers_mut().set(2, 5);

		let instruction = Sltu::new(R::new(3, 0b011, 1, 2, 0));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 0); // 5 < 5 = false

		Ok(())
	}
}
