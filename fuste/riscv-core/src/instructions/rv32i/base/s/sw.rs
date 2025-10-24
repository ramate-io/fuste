use super::S;
use crate::instructions::{ExecutableInstruction, ExecutableInstructionError, WordInstruction};
use crate::machine::Machine;
use core::fmt::{self, Display};

/// SW: Store Word.
///
/// Quick reference: https://www.vicilogic.com/static/ext/RISCV/RV32I_BaseInstructionSet.pdf
#[derive(Debug)]
pub struct Sw(S);

impl Sw {
    pub const OPCODE: u32 = S::OPCODE; // 0b0100011
    pub const FUNCT3: u8 = 0b010;
    pub const INSTRUCTION_NAME: &'static str = "sw";

    #[inline(always)]
    pub fn new(s: S) -> Self {
        Self(s)
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
    pub fn imm(&self) -> i32 {
        self.0.imm()
    }

    #[inline(always)]
    pub fn funct3(&self) -> u8 {
        self.0.funct3()
    }
}

impl Display for Sw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} x{}, {}(x{})", Self::INSTRUCTION_NAME, self.rs2(), self.imm(), self.rs1())
    }
}

impl WordInstruction for Sw {
    #[inline(always)]
    fn to_word(self) -> u32 {
        self.0.to_word(Self::OPCODE)
    }

    #[inline(always)]
    fn from_word(word: u32) -> Self {
        Self(S::from_word(word))
    }
}

impl<const MEMORY_SIZE: usize> ExecutableInstruction<MEMORY_SIZE> for Sw {
    #[inline(always)]
    fn execute(self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), ExecutableInstructionError> {
        let S { rs1, rs2, imm, .. } = self.0;

        let base_addr = machine.registers().get(rs1 as usize);
        let eff_addr = base_addr.wrapping_add(imm as u32);

        let value = machine.registers().get(rs2 as usize);
        machine.memory_mut().write_word(eff_addr, value)?;

        machine.registers_mut().program_counter_mut().increment();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sw_inner_construction() -> Result<(), ExecutableInstructionError> {
        let mut machine = Machine::<1024>::new();

        // base in x1
        machine.registers_mut().set(1, 0x100);
        // value in x2
        machine.registers_mut().set(2, 0xDEAD_BEEF);

        let instruction = Sw::new(S::new(0b010, 1, 2, 8)); // store to 0x100 + 8
        instruction.execute(&mut machine)?;

        assert_eq!(machine.memory().read_word(0x108)?, 0xDEAD_BEEF);
        assert_eq!(machine.registers().program_counter(), 4);
        Ok(())
    }
}


