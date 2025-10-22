use super::B;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// BLTU: Branch if Less Than Unsigned.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Bltu(B);

impl Bltu {
	pub const OPCODE: u32 = 0b1100011;
	pub const FUNCT3: u8 = 0b110;

	#[inline(always)]
	pub fn new(b: B) -> Self {
		Self(b)
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
	pub fn offset(&self) -> i32 {
		self.0.offset()
	}

	#[inline(always)]
	pub fn funct3(&self) -> u8 {
		self.0.funct3()
	}
}

impl WordInstruction for Bltu {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(B::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Bltu {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let B { rs1, rs2, imm, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register values
		let rs1_val = registers.get(rs1 as usize);
		let rs2_val = registers.get(rs2 as usize);

		// Check if rs1 < rs2 (unsigned comparison)
		if rs1_val < rs2_val {
			// Branch taken: PC = PC + offset
			let current_pc = registers.program_counter();
			let target_pc = current_pc.wrapping_add(imm as u32);
			registers.program_counter_mut().set(target_pc);
		} else {
			// Branch not taken: PC = PC + 4
			registers.program_counter_mut().increment();
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bltu_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up registers: rs1 < rs2 (unsigned)
		machine.registers_mut().set(1, 5);
		machine.registers_mut().set(2, 10);

		let instruction = Bltu::new(B::new(0b110, 1, 2, 8)); // branch offset 8
		instruction.execute(&mut machine)?;

		// Check PC was set to 0 + 8 = 8 (branch taken)
		assert_eq!(machine.registers().program_counter(), 8);

		Ok(())
	}

	#[test]
	fn test_bltu_branch_not_taken() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up registers: rs1 >= rs2 (unsigned)
		machine.registers_mut().set(1, 15);
		machine.registers_mut().set(2, 10);

		let instruction = Bltu::new(B::new(0b110, 1, 2, 8)); // branch offset 8
		instruction.execute(&mut machine)?;

		// Check PC was incremented by 4 (branch not taken)
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_bltu_with_large_values() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		// Set up registers with large unsigned values
		machine.registers_mut().set(1, 0x80000000); // Large unsigned value
		machine.registers_mut().set(2, 0xFFFFFFFF); // Even larger unsigned value

		let instruction = Bltu::new(B::new(0b110, 1, 2, 8)); // branch offset 8
		instruction.execute(&mut machine)?;

		// Check PC was set to 0 + 8 = 8 (branch taken)
		assert_eq!(machine.registers().program_counter(), 8);

		Ok(())
	}
}
