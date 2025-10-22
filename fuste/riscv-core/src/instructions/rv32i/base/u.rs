pub mod lui;

#[derive(Debug)]
pub struct U {
	rd: u8,
	imm: u32,
}

impl U {
	#[inline(always)]
	pub fn new(rd: u8, imm: u32) -> Self {
		Self { rd, imm }
	}

	#[inline(always)]
	pub fn from_word(word: u32) -> Self {
		Self {
			// bits 7 - 11 inclusive
			rd: ((word & 0b0000_0001_1111_0000_0000_0000_0000_0000) >> 20) as u8,
			// bits 12 - 31 inclusive
			imm: (word & 0b0000_0000_0000_1111_1111_1111_1111_1111),
		}
	}

	#[inline(always)]
	pub fn rd(&self) -> u8 {
		// mask the rd to 5 bits
		self.rd & 0b11111
	}

	#[inline(always)]
	pub fn word_rd(&self) -> u32 {
		// occupies bits 7 - 11 inclusive
		(self.rd() as u32) << 20
	}

	#[inline(always)]
	pub fn imm(&self) -> u32 {
		// mask the imm to 20 bits
		self.imm & 0b1111_1111_1111_1111_1111
	}

	#[inline(always)]
	pub fn word_imm(&self) -> u32 {
		// occupies bits 12 - 31 exclusive (which are the least significant bits of the word, so no need to shift)
		self.imm()
	}

	#[inline(always)]
	pub fn to_word(&self, opcode: u32) -> u32 {
		// opcode is expected to be unshifted, so we shift it left by to occupy bits 0 - 6 inclusive
		let word_opcode = (opcode << 25) & 0b1111_1110_0000_0000_0000_0000_0000_0000;

		word_opcode | self.word_rd() | self.word_imm()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_word() {
		let u = U::new(1, 2);

		let word = u.to_word(0b0110111);

		assert_eq!(word, 0b0110_1110_0001_0000_0000_0000_0000_0010);
	}
}
