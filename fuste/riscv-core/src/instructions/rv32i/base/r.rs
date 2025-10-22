pub mod add;

#[derive(Debug)]
pub struct R {
	rd: u8,
	funct3: u8,
	rs1: u8,
	rs2: u8,
	funct7: u8,
}

impl R {
	pub const OPCODE: u32 = 0110011;

	#[inline(always)]
	pub fn new(rd: u8, funct3: u8, rs1: u8, rs2: u8, funct7: u8) -> Self {
		Self { rd, funct3, rs1, rs2, funct7 }
	}

	#[inline(always)]
	pub fn from_word(word: u32) -> Self {
		Self {
			// bits [11:7]
			rd: ((word & 0b0000_0000_0000_0000_0000_1111_1000_0000) >> 7) as u8,
			// bits [14:12]
			funct3: ((word & 0b0000_0000_0000_0000_0111_0000_0000_0000) >> 12) as u8,
			// bits [19:15]
			rs1: ((word & 0b0000_0000_0000_1111_1000_0000_0000_0000) >> 15) as u8,
			// bits [24:20]
			rs2: ((word & 0b0000_0000_0001_1111_0000_0000_0000_0000) >> 20) as u8,
			// bits [31:25]
			funct7: ((word & 0b1111_1110_0000_0000_0000_0000_0000_0000) >> 25) as u8,
		}
	}

	#[inline(always)]
	pub fn rd(&self) -> u8 {
		// mask the rd to 5 bits for safety
		self.rd & 0b11111
	}

	#[inline(always)]
	pub fn funct3(&self) -> u8 {
		// mask the funct3 to 3 bits for safety
		self.funct3 & 0b111
	}

	#[inline(always)]
	pub fn rs1(&self) -> u8 {
		// mask the rs1 to 5 bits for safety
		self.rs1 & 0b11111
	}

	#[inline(always)]
	pub fn rs2(&self) -> u8 {
		// mask the rs2 to 5 bits for safety
		self.rs2 & 0b11111
	}

	#[inline(always)]
	pub fn funct7(&self) -> u8 {
		// mask the funct7 to 7 bits for safety
		self.funct7 & 0b1111111
	}

	#[inline(always)]
	pub fn word_rd(&self) -> u32 {
		// occupies bits [11:7]
		(self.rd() as u32) << 7
	}

	#[inline(always)]
	pub fn word_funct3(&self) -> u32 {
		// occupies bits [14:12]
		(self.funct3() as u32) << 12
	}

	#[inline(always)]
	pub fn word_rs1(&self) -> u32 {
		// occupies bits [19:15]
		(self.rs1() as u32) << 15
	}

	#[inline(always)]
	pub fn word_rs2(&self) -> u32 {
		// occupies bits [24:20]
		(self.rs2() as u32) << 20
	}

	#[inline(always)]
	pub fn word_funct7(&self) -> u32 {
		// occupies bits [31:25]
		(self.funct7() as u32) << 25
	}

	#[inline(always)]
	pub fn to_word(&self, opcode: u32) -> u32 {
		// occupies the bits [6:0]
		let word_opcode = opcode & 0b0000_0000_0000_0000_0000_0000_0111_1111;

		word_opcode
			| self.word_rd()
			| self.word_funct3()
			| self.word_rs1()
			| self.word_rs2()
			| self.word_funct7()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_word_and_to_word() {
		// Test with a known ADD instruction encoding
		let original_word = 0b0000_0000_0000_0000_0000_0000_0011_0011; // ADD x0, x0, x0
		let r = R::from_word(original_word);
		let reconstructed_word = r.to_word(0b0110011);

		assert_eq!(reconstructed_word, original_word);
	}

	#[test]
	fn test_field_extraction() {
		// Test field extraction with known values
		let word = 0b0000_0000_0000_0000_0000_0000_0011_0011; // ADD x0, x0, x0
		let r = R::from_word(word);

		assert_eq!(r.rd(), 0);
		assert_eq!(r.funct3(), 0);
		assert_eq!(r.rs1(), 0);
		assert_eq!(r.rs2(), 0);
		assert_eq!(r.funct7(), 0);
	}
}
