pub mod memory;
pub use memory::Memory;
pub mod registers;
pub use registers::Registers;
pub struct Machine<const MEMORY_SIZE: usize> {
	memory: Memory<MEMORY_SIZE>,
	registers: Registers,
}

impl<const MEMORY_SIZE: usize> Machine<MEMORY_SIZE> {
	pub fn new() -> Self {
		Self { memory: Memory::new(), registers: Registers::new() }
	}

	#[inline(always)]
	pub fn memory(&self) -> &Memory<MEMORY_SIZE> {
		&self.memory
	}

	#[inline(always)]
	pub fn memory_mut(&mut self) -> &mut Memory<MEMORY_SIZE> {
		&mut self.memory
	}

	#[inline(always)]
	pub fn registers(&self) -> &Registers {
		&self.registers
	}

	#[inline(always)]
	pub fn registers_mut(&mut self) -> &mut Registers {
		&mut self.registers
	}
}
