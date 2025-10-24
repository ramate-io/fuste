use core::error::Error;
use core::fmt::{self, Display};

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
			return Err(MemoryError::AddressOutOfBounds(address));
		}
		Ok(self.memory[address as usize])
	}

	/// Write a byte to memory at the given address
	pub fn write_byte(&mut self, address: u32, value: u8) -> Result<(), MemoryError> {
		if address as usize >= SIZE {
			return Err(MemoryError::AddressOutOfBounds(address));
		}
		self.memory[address as usize] = value;
		Ok(())
	}

	/// Writes multiple bytes to memory at the given address
	pub fn write_bytes(&mut self, address: u32, bytes: &[u8]) -> Result<(), MemoryError> {
		if address as usize + bytes.len() > SIZE {
			return Err(MemoryError::AddressOutOfBounds(address));
		}
		self.memory[address as usize..address as usize + bytes.len()].copy_from_slice(bytes);
		Ok(())
	}

	/// Read a 32-bit word from memory at the given address (little-endian)
	pub fn read_word(&self, address: u32) -> Result<u32, MemoryError> {
		if address as usize + 3 >= SIZE {
			return Err(MemoryError::AddressOutOfBounds(address));
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
			return Err(MemoryError::AddressOutOfBounds(address));
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

	/// Loads a segment into memory starting at the given address
	pub fn load_segment(&mut self, address: u32, segment: &[u8]) -> Result<(), MemoryError> {
		if address as usize + segment.len() > SIZE {
			return Err(MemoryError::AddressOutOfBounds(address));
		}
		self.memory[address as usize..address as usize + segment.len()].copy_from_slice(segment);
		Ok(())
	}

	/// Loads a word segment into memory starting at the given address
	pub fn load_word_segment(&mut self, address: u32, segment: &[u32]) -> Result<(), MemoryError> {
		if address as usize + segment.len() * 4 > SIZE {
			return Err(MemoryError::AddressOutOfBounds(address));
		}
		for (i, word) in segment.iter().enumerate() {
			self.write_word(address + i as u32 * 4, *word)?;
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq)]
pub enum MemoryError {
	AddressOutOfBounds(u32),
}

impl Display for MemoryError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl Error for MemoryError {}
