use super::U;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError};
use crate::machine::Machine;

#[derive(Debug)]
pub struct Lui(U);

impl Lui {
	pub const OPCODE: u32 = 0b0110111;
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Lui {
	#[inline(always)]
	fn from_word(word: u32) -> Self {
		Self(U::from_word(word))
	}

	#[inline(always)]
	fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
		let U { rd, imm } = self.0;

		machine.registers_mut().set(rd, imm);

		Ok(())
	}
}
