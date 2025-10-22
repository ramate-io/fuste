use super::J;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// JAL: Jump and Link.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Jal(J);

impl Jal {
	pub const OPCODE: u32 = 0b1101111;

	#[inline(always)]
	pub fn new(j: J) -> Self {
		Self(j)
	}

	#[inline(always)]
	pub fn rd(&self) -> u8 {
		self.0.rd()
	}

	#[inline(always)]
	pub fn imm(&self) -> i32 {
		self.0.imm()
	}
}

impl WordInstruction for Jal {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(J::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Jal {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let J { rd, .. } = self.0;

		let registers = machine.registers_mut();
		let current_pc = registers.program_counter();

		// Store return address (PC + 4) in destination register
		registers.set(rd, current_pc + 4);

		// Calculate jump target: PC + offset
		let target_pc = (current_pc as i32 + self.imm()) as u32;

		// Update program counter to jump target
		registers.program_counter_mut().set(target_pc);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_jal_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();
		machine.registers_mut().program_counter_set(100);

		let imm = 0; // No jump offset
		let instruction = Jal::new(J::new(1, imm));
		instruction.execute(&mut machine)?;

		// Return address should be stored in register 1
		assert_eq!(machine.registers().get(1), 104); // PC + 4

		// PC should be updated to PC + offset
		assert_eq!(machine.registers().program_counter(), 100); // PC + (0 << 1)

		Ok(())
	}

	#[test]
	fn test_jal_with_positive_offset() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();
		machine.registers_mut().program_counter_set(100);

		let imm = 10; // Positive offset
		let instruction = Jal::new(J::new(1, imm));
		instruction.execute(&mut machine)?;

		// Return address should be stored in register 1
		assert_eq!(machine.registers().get(1), 104); // PC + 4

		// PC should be updated to PC + offset
		assert_eq!(machine.registers().program_counter(), 110); // PC + 10

		Ok(())
	}

	#[test]
	fn test_jal_with_negative_offset() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();
		machine.registers_mut().program_counter_set(100);

		// Create a negative immediate
		let imm = -4;
		let instruction = Jal::new(J::new(1, imm));
		instruction.execute(&mut machine)?;

		// Return address should be stored in register 1
		assert_eq!(machine.registers().get(1), 104); // PC + 4

		// PC should be updated to PC + offset
		assert_eq!(machine.registers().program_counter(), 96); // PC + (-4) = PC - 4

		Ok(())
	}

	#[test]
	fn test_jal_from_word() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();
		machine.registers_mut().program_counter_set(160_000_000);

		// Create a JAL instruction word: rd=1, imm=-1042430, opcode=1101111
		let word_with_imm =
			0b0000_0000_0000_0000_0000_0000_1110_0111
				| (1 << 31) | (1 << 21)
				| (1 << 20) | (1 << 12); // imm[20] = 1, imm[10:1] = 1, imm[11] = 1, imm[19:12] = 1
		let instruction = Jal::from_word(word_with_imm);
		instruction.execute(&mut machine)?;

		// Return address should be stored in register 1
		assert_eq!(machine.registers().get(1), 160_000_000 + 4); // PC + 4

		// PC should remain the same (imm=0, offset=0)
		assert_eq!(machine.registers().program_counter(), 160_000_000 - 1042430);

		Ok(())
	}
}
