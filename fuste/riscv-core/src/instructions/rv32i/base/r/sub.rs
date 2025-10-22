use super::R;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// SUB: Subtract.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Sub(R);

impl Sub {
	pub const OPCODE: u32 = 0b0110011;
	pub const FUNCT3: u8 = 0b000;
	pub const FUNCT7: u8 = 0b0100000;

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

impl WordInstruction for Sub {
	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(R::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Sub {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let R { rd, rs1, rs2, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register values
		let rs1_val = registers.get(rs1 as usize);
		let rs2_val = registers.get(rs2 as usize);

		// Perform subtraction
		let result = rs1_val.wrapping_sub(rs2_val);

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
	fn test_sub_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers
		machine.registers_mut().set(1, 20);
		machine.registers_mut().set(2, 10);

		let instruction = Sub::new(R::new(3, 0, 1, 2, 0b0100000));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 10); // 20 - 10 = 10

		// Check PC was incremented by 4 (word size)
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_sub_with_underflow() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers with values that will underflow
		machine.registers_mut().set(1, 0);
		machine.registers_mut().set(2, 1);

		let instruction = Sub::new(R::new(3, 0, 1, 2, 0b0100000));
		instruction.execute(&mut machine)?;

		// Check result wraps around
		assert_eq!(machine.registers().get(3), 0xFFFFFFFF); // 0 - 1 = 0xFFFFFFFF (wrapping)

		Ok(())
	}

	#[test]
	fn test_sub_from_word() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up source registers
		machine.registers_mut().set(1, 15);
		machine.registers_mut().set(2, 5);

		// Create SUB instruction word: SUB x3, x1, x2
		// Use the R format's to_word method to construct it properly
		let r = R::new(3, 0, 1, 2, 0b0100000);
		let word = r.to_word(0b0110011);
		let instruction = Sub::from_word(word);
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 10); // 15 - 5 = 10

		Ok(())
	}
}
