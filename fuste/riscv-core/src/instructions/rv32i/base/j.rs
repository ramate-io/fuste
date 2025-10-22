pub mod jal;

#[derive(Debug)]
pub struct J {
	rd: u8,
	imm: i32,
}

impl J {
	#[inline(always)]
	pub fn new(rd: u8, imm: i32) -> Self {
		Self { rd, imm }
	}

	#[inline(always)]
	pub fn from_word(word: u32) -> Self {
		// Extract rd from bits [11:7]
		let rd = ((word & 0b0000_0000_0000_0000_0000_1111_1000_0000) >> 7) as u8;

		// Extract and reconstruct the immediate field
		// J-type immediate is encoded as:
		// imm[20] = word[31]
		// imm[10:1] = word[30:21]
		// imm[11] = word[20]
		// imm[19:12] = word[19:12]
		let imm_20 = (word & 0b1000_0000_0000_0000_0000_0000_0000_0000) >> 11; // bit 31 -> bit 20
		let imm_10_1 = (word & 0b0111_1111_1110_0000_0000_0000_0000_0000) >> 20; // bits 30:21 -> bits 10:1
		let imm_11 = (word & 0b0000_0000_0001_0000_0000_0000_0000_0000) >> 9; // bit 20 -> bit 11
		let imm_19_12 = (word & 0b0000_0000_0000_1111_1111_0000_0000_0000) >> 12; // bits 19:12 -> bits 19:12

		let imm_u32 = imm_20 | imm_10_1 | imm_11 | imm_19_12;

		// Sign extend the 21-bit immediate to 32 bits
		let imm = if (imm_u32 & 0b0000_0000_0001_0000_0000_0000_0000_0000) != 0 {
			imm_u32 | 0b1111_1111_1110_0000_0000_0000_0000_0000
		} else {
			imm_u32
		} as i32;

		Self { rd, imm }
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
	pub fn imm(&self) -> i32 {
		self.imm
	}

	#[inline(always)]
	pub fn offset(&self) -> i32 {
		// J-type instructions use the immediate shifted left by 1 bit
		self.imm << 1
	}

	#[inline(always)]
	pub fn word_imm(&self) -> u32 {
		// Reconstruct the immediate field for encoding
		let imm = self.imm as u32;
		let imm_20 = (imm & 0b0000_0000_0001_0000_0000_0000_0000_0000) << 11; // bit 20 -> bit 31
		let imm_10_1 = (imm & 0b0000_0000_0000_0000_0000_1111_1111_1110) << 20; // bits 10:1 -> bits 30:21
		let imm_11 = (imm & 0b0000_0000_0000_0000_0000_0000_0000_1000) << 9; // bit 11 -> bit 20
		let imm_19_12 = (imm & 0b0000_0000_0000_1111_1111_0000_0000_0000) << 12; // bits 19:12 -> bits 19:12

		imm_20 | imm_10_1 | imm_11 | imm_19_12
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
	fn test_simple_jal_encoding() {
		// Test with rd=1, imm=0, opcode=1101111
		let j = J::new(1, 0);

		// Check individual components
		assert_eq!(j.word_rd(), 0b0000_0000_0000_0000_0000_0000_1000_0000); // rd=1 << 7
		assert_eq!(j.word_imm(), 0b0000_0000_0000_0000_0000_0000_0000_0000); // imm=0

		// Test the to_word method step by step
		let opcode = 111; // 0b1101111
		let word_opcode = opcode & 0b0000_0000_0000_0000_0000_0000_0111_1111;
		let word_rd = j.word_rd();
		let word_imm = j.word_imm();
		let word = word_opcode | word_rd | word_imm;

		// Expected: opcode=1101111 (bits 6:0), rd=1 (bits 11:7), imm=0 (bits 31:12)
		// 0b0000_0000_0000_0000_0000_0000_1011_0111
		let expected = 0b0000_0000_0000_0000_0000_0000_1011_0111;
		assert_eq!(word, expected);
	}

	#[test]
	fn test_immediate_extraction() {
		// Test immediate extraction with a known pattern
		let word = 0b0000_0000_0000_0000_0000_0000_1011_0111; // imm=0
		let j = J::from_word(word);
		assert_eq!(j.imm(), 0);
		assert_eq!(j.offset(), 0);

		// Test with non-zero immediate
		let word_with_imm = 0b0000_0000_0000_0000_0000_0000_1011_0111 | (1 << 31); // imm[20] = 1
		let j_with_imm = J::from_word(word_with_imm);
		assert_eq!(j_with_imm.imm(), -1048576); // sign-extended 21-bit immediate
		assert_eq!(j_with_imm.offset(), -2097152); // offset = imm << 1
	}
}
