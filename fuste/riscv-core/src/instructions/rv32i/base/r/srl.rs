use super::R;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// SRL: Shift Right Logical.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Srl(R);

impl Srl {
	pub const OPCODE: u32 = 0b0110011;
	pub const FUNCT3: u8 = 0b101;
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

impl WordInstruction for Srl {
	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(R::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Srl {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let R { rd, rs1, rs2, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register values
		let rs1_val = registers.get(rs1 as usize);
		let rs2_val = registers.get(rs2 as usize);

		// Perform logical right shift (only use lower 5 bits of rs2 for shift amount)
		let shift_amount = rs2_val & 0b11111;
		let result = rs1_val >> shift_amount;

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
	fn test_srl_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();
		
		// Set up source registers
		machine.registers_mut().set(1, 0b0000_0000_0000_0000_0000_0000_0000_1000); // 8
		machine.registers_mut().set(2, 2); // shift by 2
		
		let instruction = Srl::new(R::new(3, 0b101, 1, 2, 0));
		instruction.execute(&mut machine)?;

		// Check result
		assert_eq!(machine.registers().get(3), 0b0000_0000_0000_0000_0000_0000_0000_0010); // 8 >> 2 = 2
		
		// Check PC was incremented
		assert_eq!(machine.registers().program_counter(), 1);

		Ok(())
	}

	#[test]
	fn test_srl_with_negative_number() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();
		
		// Set up source registers with negative number
		machine.registers_mut().set(1, 0x80000000); // Negative number
		machine.registers_mut().set(2, 1); // shift by 1
		
		let instruction = Srl::new(R::new(3, 0b101, 1, 2, 0));
		instruction.execute(&mut machine)?;

		// Check result (logical shift fills with zeros)
		assert_eq!(machine.registers().get(3), 0x40000000); // 0x80000000 >> 1 = 0x40000000

		Ok(())
	}
}
