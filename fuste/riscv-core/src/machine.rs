pub mod memory;
pub use memory::Memory;
pub mod registers;
use crate::instructions::ExecutableInstructionError;
use crate::log::RingBuffer;
use core::error::Error;
use core::fmt::{self, Display};
pub use registers::Registers;

/// The machine is the memory layout against which the plugins operate.
pub struct Machine<const MEMORY_SIZE: usize> {
	memory: Memory<MEMORY_SIZE>,
	registers: Registers,
	#[cfg(debug_assertions)]
	log: RingBuffer<4096>,
}

/// The [MachinePlugin] trait tells the machine what to do at each tick.
pub trait MachinePlugin<const MEMORY_SIZE: usize> {
	fn tick(&mut self, machine: &mut Machine<MEMORY_SIZE>) -> Result<(), MachineError>;
}

impl<const MEMORY_SIZE: usize> Machine<MEMORY_SIZE> {
	/// Creates a new machine instance with all memory and registers initialized to zero.
	pub fn new() -> Self {
		Self { memory: Memory::new(), registers: Registers::new(), log: RingBuffer::new() }
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

	#[cfg(debug_assertions)]
	#[inline(always)]
	pub fn log(&self) -> &RingBuffer<4096> {
		&self.log
	}

	#[cfg(debug_assertions)]
	#[inline(always)]
	pub fn log_mut(&mut self) -> &mut RingBuffer<4096> {
		&mut self.log
	}
}

#[derive(Debug, PartialEq)]
pub enum MachineError {
	MemoryError(memory::MemoryError),
	InstructionError(ExecutableInstructionError),
}

impl Display for MachineError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			MachineError::MemoryError(e) => write!(f, "MemoryError: {}", e),
			MachineError::InstructionError(e) => write!(f, "InstructionError: {}", e),
		}
	}
}

impl Error for MachineError {}
