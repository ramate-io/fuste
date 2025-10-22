use super::U;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;

/// LUI: load upper immediate.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Lui(U);

impl Lui {
	pub const OPCODE: u32 = 0b0110111;

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

	#[inline(always)]
	pub fn to_word(&self) -> u32 {
		self.0.to_word(Self::OPCODE)
	}
}

impl WordInstruction for Lui {
	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(U::from_word(word))
	}
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Lui {
	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let U { rd, imm } = self.0;

		let registers = machine.registers_mut();

		registers.set(rd, imm);

		registers.program_counter_mut().increment();

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_lui_inner_construction() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();
		let instruction = Lui::new(U::new(1, 2 << 12));
		instruction.execute(&mut machine)?;
		assert_eq!(machine.registers().get(1), 2 << 12);
		Ok(())
	}

	#[test]
	fn test_lui_from_word() -> Result<(), ExecutableInstructionError> {
		let mut machine = Machine::<1024>::new();
		let instruction = Lui::from_word(0b0000_0000_0000_0000_0010_0000_1011_0111);
		instruction.execute(&mut machine)?;
		assert_eq!(machine.registers().get(1), 2 << 12);
		Ok(())
	}
}
