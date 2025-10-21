/// The program counter register.
#[derive(Debug)]
pub struct ProgramCounter {
	program_counter: u32,
}

impl ProgramCounter {
	pub fn new() -> Self {
		Self { program_counter: 0 }
	}
}

impl ProgramCounter {
	pub fn get(&self) -> u32 {
		self.program_counter
	}

	pub fn set(&mut self, value: u32) {
		self.program_counter = value;
	}

	pub fn increment(&mut self) {
		// increment by 4 for RV32I alignment.
		self.program_counter += 4;
	}
}
