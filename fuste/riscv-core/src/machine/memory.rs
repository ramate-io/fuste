/// TODO: worry about privileged memory later.
/// Size is the number of bytes in the memory. It is a u32 since we are implement RVI32 for now.
pub struct Memory<const SIZE: usize> {
	pub memory: [u8; SIZE],
}

impl<const SIZE: usize> Memory<SIZE> {
	/// Create a new memory instance with all bytes initialized to zero
	pub const fn new() -> Self {
		Self { memory: [0u8; SIZE] }
	}

	/// Read a byte from memory at the given address
	pub fn read_byte(&self, address: u32) -> Result<u8, MemoryError> {
		if address as usize >= SIZE {
			return Err(MemoryError::AddressOutOfBounds);
		}
		Ok(self.memory[address as usize])
	}

	/// Write a byte to memory at the given address
	pub fn write_byte(&mut self, address: u32, value: u8) -> Result<(), MemoryError> {
		if address as usize >= SIZE {
			return Err(MemoryError::AddressOutOfBounds);
		}
		self.memory[address as usize] = value;
		Ok(())
	}

	/// Read a 32-bit word from memory at the given address (little-endian)
	pub fn read_word(&self, address: u32) -> Result<u32, MemoryError> {
		if address as usize + 3 >= SIZE {
			return Err(MemoryError::AddressOutOfBounds);
		}
		let addr = address as usize;
		Ok(u32::from_le_bytes([
			self.memory[addr],
			self.memory[addr + 1],
			self.memory[addr + 2],
			self.memory[addr + 3],
		]))
	}

	/// Write a 32-bit word to memory at the given address (little-endian)
	pub fn write_word(&mut self, address: u32, value: u32) -> Result<(), MemoryError> {
		if address as usize + 3 >= SIZE {
			return Err(MemoryError::AddressOutOfBounds);
		}
		let bytes = value.to_le_bytes();
		let addr = address as usize;
		self.memory[addr] = bytes[0];
		self.memory[addr + 1] = bytes[1];
		self.memory[addr + 2] = bytes[2];
		self.memory[addr + 3] = bytes[3];
		Ok(())
	}

	/// Get the size of the memory in bytes
	pub const fn size() -> u32 {
		SIZE as u32
	}
}

#[derive(Debug, PartialEq)]
pub enum MemoryError {
	AddressOutOfBounds,
}
