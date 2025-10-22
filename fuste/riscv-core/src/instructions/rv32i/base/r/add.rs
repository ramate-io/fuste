use super::R;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// ADD: Add.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Add(R);

impl Add {
	pub const OPCODE: u32 = 0b0110011;
	pub const FUNCT3: u8 = 0b000;
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

	#[inline(always)]
	pub fn to_word(&self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}
}

impl WordInstruction for Add {
	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(R::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Add {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let R { rd, rs1, rs2, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register values
		let rs1_val = registers.get(rs1 as usize);
		let rs2_val = registers.get(rs2 as usize);

		// Perform addition
		let result = rs1_val.wrapping_add(rs2_val);

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
	fn test_add_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers
		machine.registers_mut().set(1, 10);
		machine.registers_mut().set(2, 20);

		let instruction = Add::new(R::new(3, 0, 1, 2, 0));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 30); // 10 + 20 = 30

		// Check PC was incremented
		assert_eq!(machine.registers().program_counter(), 1);

		Ok(())
	}

	#[test]
	fn test_add_with_overflow() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers with values that will overflow
		machine.registers_mut().set(1, 0xFFFFFFFF);
		machine.registers_mut().set(2, 1);

		let instruction = Add::new(R::new(3, 0, 1, 2, 0));
		instruction.execute(&mut machine)?;

		// Check result wraps around
		assert_eq!(machine.registers().get(3), 0); // 0xFFFFFFFF + 1 = 0 (wrapping)

		Ok(())
	}

	#[test]
	fn test_add_from_word() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers
		machine.registers_mut().set(1, 5);
		machine.registers_mut().set(2, 7);

		// Create ADD instruction word: ADD x3, x1, x2
		// rd=3, funct3=0, rs1=1, rs2=2, funct7=0, opcode=0110011
		let word = 0b0000_0000_0000_0001_0000_0001_1011_0011;
		let instruction = Add::from_word(word);
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 12); // 5 + 7 = 12

		Ok(())
	}

	#[test]
	fn test_add_zero_registers() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers
		machine.registers_mut().set(1, 15);
		machine.registers_mut().set(2, 0);

		let instruction = Add::new(R::new(3, 0, 1, 2, 0));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 15); // 15 + 0 = 15

		Ok(())
	}
}
