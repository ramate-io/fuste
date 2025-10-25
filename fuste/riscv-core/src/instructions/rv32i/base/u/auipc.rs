use super::U;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;
use core::fmt::{self, Display};

/// Auipc: load upper immediate.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Auipc(U);

impl Auipc {
	pub const OPCODE: u32 = 0b0010111;
	pub const INSTRUCTION_NAME: &'static str = "auipc";

	#[inline(always)]
	pub fn new(u: U) -> Self {
		Self(u)
	}

	#[inline(always)]
	pub fn rd(&self) -> u8 {
		self.0.rd()
	}

	#[inline(always)]
	pub fn imm(&self) -> u32 {
		self.0.imm()
	}
}

impl Display for Auipc {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} x{}, 0x{:X}", Self::INSTRUCTION_NAME, self.rd(), self.imm())
	}
}

impl WordInstruction for Auipc {
	#[inline(always)]
	fn to_word(self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}

	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(U::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Auipc {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let U { rd, imm } = self.0;

		let pc = machine.registers().program_counter();
		let result = pc.wrapping_add(imm);

		machine.registers_mut().set(rd, result);

		machine.registers_mut().program_counter_mut().increment();

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_auipc_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		machine.registers_mut().program_counter_set(42);
		let program_counter_initial = machine.registers().program_counter();

		let imm = 2 << 12;

		let instruction = Auipc::new(U::new(1, imm));
		instruction.execute(&mut machine)?;

		// program counter after increment
		assert_eq!(machine.registers().program_counter(), program_counter_initial + 4);

		// stored in register
		assert_eq!(machine.registers().get(1), program_counter_initial + imm);

		Ok(())
	}

	#[test]
	fn test_auipc_from_word() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();

		machine.registers_mut().program_counter_set(42);
		let program_counter_initial = machine.registers().program_counter();

		let imm = 2 << 12;
		let word = imm + 0b1001_0111; // (rd 1, opcode 0010111 = auipc)

		let instruction = Auipc::from_word(word);
		instruction.execute(&mut machine)?;

		// program counter after increment
		assert_eq!(machine.registers().program_counter(), program_counter_initial + 4);

		// stored in register
		assert_eq!(machine.registers().get(1), program_counter_initial + imm);

		Ok(())
	}
}
