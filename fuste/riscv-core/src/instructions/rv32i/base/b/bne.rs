use super::B;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// BNE: Branch if Not Equal.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Bne(B);

impl Bne {
	pub const OPCODE: u32 = 0b1100011;
	pub const FUNCT3: u8 = 0b001;

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

	#[inline(always)]
	pub fn to_word(&self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}
}

impl WordInstruction for Bne {
	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(B::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Bne {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let B { rs1, rs2, imm, .. } = self.0;

		let registers = machine.registers_mut();

		// Get source register values
		let rs1_val = registers.get(rs1 as usize);
		let rs2_val = registers.get(rs2 as usize);

		// Check if rs1 != rs2
		if rs1_val != rs2_val {
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
	fn test_bne_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();
		
		// Set up registers with different values
		machine.registers_mut().set(1, 10);
		machine.registers_mut().set(2, 20);
		
		let instruction = Bne::new(B::new(0b001, 1, 2, 8)); // branch offset 8
		instruction.execute(&mut machine)?;

		// Check PC was set to 0 + 8 = 8 (branch taken)
		assert_eq!(machine.registers().program_counter(), 8);

		Ok(())
	}

	#[test]
	fn test_bne_branch_not_taken() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();
		
		// Set up registers with equal values
		machine.registers_mut().set(1, 10);
		machine.registers_mut().set(2, 10);
		
		let instruction = Bne::new(B::new(0b001, 1, 2, 8)); // branch offset 8
		instruction.execute(&mut machine)?;

		// Check PC was incremented by 4 (branch not taken)
		assert_eq!(machine.registers().program_counter(), 4);

		Ok(())
	}

	#[test]
	fn test_bne_with_negative_offset() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();
		
		// Set up registers with different values
		machine.registers_mut().set(1, 5);
		machine.registers_mut().set(2, 15);
		
		let instruction = Bne::new(B::new(0b001, 1, 2, -4)); // branch offset -4
		instruction.execute(&mut machine)?;

		// Check PC was set to 0 + (-4) = 0xFFFFFFFC (branch taken)
		assert_eq!(machine.registers().program_counter(), 0xFFFFFFFC);

		Ok(())
	}
}
