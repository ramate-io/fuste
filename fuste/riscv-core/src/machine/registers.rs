pub mod program_counter;
pub use program_counter::ProgramCounter;

/// The registers of the machine.
#[derive(Debug)]
pub struct Registers {
	// TODO: decide whether to represent the 0x0 as a special register or not. It is supposed to always be 0.
	general_purpose: [u32; 32],
	/// The program counter. This is a special register in the RISC-V ISA and is represnted as such here.
	program_counter: ProgramCounter,
}

impl Registers {
	pub fn new() -> Self {
		Self { general_purpose: [0; 32], program_counter: ProgramCounter::new() }
	}

	#[inline(always)]
	pub fn general_purpose(&self) -> &[u32; 32] {
		&self.general_purpose
	}

	#[inline(always)]
	pub fn general_purpose_mut(&mut self) -> &mut [u32; 32] {
		&mut self.general_purpose
	}

	#[inline(always)]
	pub fn get(&self, index: usize) -> u32 {
		self.general_purpose[index]
	}

	#[inline(always)]
	pub fn set(&mut self, index: u8, value: u32) {
		self.general_purpose[index as usize] = value;
	}

	#[inline(always)]
	pub fn program_counter(&self) -> u32 {
		self.program_counter.get()
	}

	#[inline(always)]
	pub fn program_counter_mut(&mut self) -> &mut ProgramCounter {
		&mut self.program_counter
	}

	#[inline(always)]
	pub fn program_counter_set(&mut self, value: u32) {
		self.program_counter.set(value);
	}
}
