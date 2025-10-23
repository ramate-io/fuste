pub mod auipc;
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
			// bits [11:7]
			rd: ((word & 0b0000_0000_0000_0000_0000_1111_1000_0000) >> 7) as u8,
			// bits [31:12] with first 12 bits masked to 0
			imm: (word & 0b1111_1111_1111_1111_1111_0000_0000_0000),
		}
	}

	#[inline(always)]
	pub fn rd(&self) -> u8 {
		// mask the rd to 5 bits for safety
		self.rd & 0b11111
	}

	#[inline(always)]
	pub fn word_rd(&self) -> u32 {
		// occupies bits [11:7]
		(self.rd() as u32) << 7
	}

	#[inline(always)]
	pub fn imm(&self) -> u32 {
		// mask the imm s.t. the first 12 bits are 0
		self.imm & 0b1111_1111_1111_1111_1111_0000_0000_0000
	}

	#[inline(always)]
	pub fn word_imm(&self) -> u32 {
		// occupies bits [31:12]
		self.imm()
	}

	#[inline(always)]
	pub fn to_word(&self, opcode: u32) -> u32 {
		// occupies the bits [6:0]
		let word_opcode = opcode & 0b0000_0000_0000_0000_0000_0000_0111_1111;

		word_opcode | self.word_rd() | self.word_imm()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_word() {
		let u = U::new(1, 2 << 12);

		let word = u.to_word(0b011_0111);

		assert_eq!(word, 0b0000_0000_0000_0000_0010_0000_1011_0111);
	}
}
