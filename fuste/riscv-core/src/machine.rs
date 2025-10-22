pub mod memory;
pub use memory::Memory;
pub mod registers;
use crate::instructions::{ExecutableInstructionError, Rv32iInstruction};
pub use registers::Registers;

/// The machine is the memory layout against which the plugins operate.
pub struct Machine<const MEMORY_SIZE: usize> {
	memory: Memory<MEMORY_SIZE>,
	registers: Registers,
}

/// The [MachinePlugin] trait tells the machine what to do at each tick.
pub trait MachinePlugin<const MEMORY_SIZE: usize> {
	fn tick(&mut self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), MachineError>;
}

impl<const MEMORY_SIZE: usize> Machine<MEMORY_SIZE> {
	/// Creates a new machine instance with all memory and registers initialized to zero.
	pub fn new() -> Self {
		Self { memory: Memory::new(), registers: Registers::new() }
	}

	/// Borrows the memory of the machine.
	#[inline(always)]
	pub fn memory(&self) -> &Memory<MEMORY_SIZE> {
		&self.memory
	}

	/// Borrows the memory of the machine mutably.
	#[inline(always)]
	pub fn memory_mut(&mut self) -> &mut Memory<MEMORY_SIZE> {
		&mut self.memory
	}

	/// Borrows the registers of the machine.
	#[inline(always)]
	pub fn registers(&self) -> &Registers {
		&self.registers
	}

	/// Borrows the registers of the machine mutably.
	#[inline(always)]
	pub fn registers_mut(&mut self) -> &mut Registers {
		&mut self.registers
	}

	/// Runs the machine with the given plugin.
	pub fn run<P: MachinePlugin<MEMORY_SIZE>>(
		&mut self,
		plugin: &mut P,
	) -> Result<(), MachineError> {
		loop {
			plugin.tick(self)?;
		}
	}
}

/// The ControlFlowComputer that use the machine to implement a control flow computer.
/// On each tick, it reads the instruction at the program counter and executes it.
///
/// Updates of the program counter are internal to [Instruction]s.
pub struct Rv32iComputer;

impl<const MEMORY_SIZE: usize> MachinePlugin<MEMORY_SIZE> for Rv32iComputer {
	fn tick(&mut self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), MachineError> {
		let program_counter = machine.registers().program_counter();
		let instruction =
			machine.memory().read_word(program_counter).map_err(MachineError::MemoryError)?;
		Rv32iInstruction::load_and_execute(instruction, machine)
			.map_err(MachineError::InstructionError)?;

		Ok(())
	}
}

#[derive(Debug, PartialEq)]
pub enum MachineError {
	MemoryError(memory::MemoryError),
	InstructionError(ExecutableInstructionError),
}
