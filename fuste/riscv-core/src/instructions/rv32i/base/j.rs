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
		//                   |i20  |i[10:1]    |i11|i[19:12]   |rd   |opcode |
		let rd = ((word & 0b0__00_0000_0000__0__0000_0000__1111_1__0000000) >> 7) as u8;

		// Extract and reconstruct the immediate field
		// J-type immediate is encoded as:
		// imm[20] = word[31]
		// imm[10:1] = word[30:21]
		// imm[11] = word[20]
		// imm[19:12] = word[19:12]

		//                       |i20  |i[10:1]    |i11|i[19:12]   |rd   |opcode |
		let imm_20 = (word & 0b1__00_0000_0000__0__0000_0000__0000_0__000_0000) >> 11; // bit 31 -> bit 20

		//                         |i20  |i[10:1]    |i11|i[19:12]   |rd   |opcode |
		let imm_10_1 = (word & 0b0__11_1111_1111__0__0000_0000__0000_0__000_0000) >> 20; // bits 30:21 -> bits 10:1

		//                       |i20  |i[10:1]    |i11|i[19:12]   |rd   |opcode |
		let imm_11 = (word & 0b0__00_0000_0000__1__0000_0000__0000_0__000_0000) >> 9; // bit 20 -> bit 11

		//                         |i20  |i[10:1]    |i11|i[19:12]   |rd   |opcode |
		let imm_19_12 = word & 0b0__00_0000_0000__0__1111_1111__0000_0__000_0000; // bits 19:12 -> bits 19:12

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
	pub fn word_imm(&self) -> u32 {
		// Reconstruct the immediate field for encoding
		let imm = self.imm as u32;
		let imm_20 = (imm & 0b0000_0000_0001_0000_0000_0000_0000_0000) << 11; // bit 20 -> bit 31
		let imm_10_1 = (imm & 0b0000_0000_0000_0000_0000_0111_1111_1110) << 20; // bits 10:1 -> bits 30:21
		let imm_11 = (imm & 0b0000_0000_0000_0000_0000_1000_0000_0000) << 9; // bit 11 -> bit 20
		let imm_19_12 = imm & 0b0000_0000_0000_1111_1111_0000_0000_0000; // bits 19:12 -> bits 19:12

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
		let word = j.to_word(0b0110111);

		assert_eq!(word, 0b0000_0000_0000_0000_0000_0000_1011_0111);
	}

	#[test]
	fn test_immediate_extraction_zero() {
		// Test immediate extraction with a known pattern
		let word = 0b0000_0000_0000_0000_0000_0000_1011_0111; // imm=0
		let j = J::from_word(word);
		assert_eq!(j.imm(), 0);
	}

	#[test]
	fn test_immediate_extraction_positive() {
		// Test immediate extraction with a known pattern
		let word_with_imm =
			0b0000_0000_0000_0000_0000_0000_1011_0111
				| (0 << 20) | (1 << 21)
				| (1 << 20) | (1 << 12); // imm[20] = 0, imm[10:1] = 1, imm[11] = 1, imm[19:12] = 1

		let expected_imm = 0b0000_0000_0000_0000_0001_1000_0000_0010i32;
		let j = J::from_word(word_with_imm);
		assert_eq!(j.imm(), expected_imm);
	}

	#[test]
	fn test_immediate_extraction_negative_simple() {
		// Test with non-zero immediate
		let word_with_imm = 0b0000_0000_0000_0000_0000_0000_1011_0111 | (1 << 31); // imm[20] = 1
		let j_with_imm = J::from_word(word_with_imm);
		assert_eq!(j_with_imm.imm(), -1048576); // sign-extended 21-bit immediate
	}

	#[test]
	fn test_immediate_extraction_negative_complex() {
		// Test with non-zero immediate
		let word_with_imm =
			0b0000_0000_0000_0000_0000_0000_1011_0111
				| (1 << 31) | (1 << 21)
				| (1 << 20) | (1 << 12); // imm[20] = 1, imm[10:1] = 1, imm[11] = 1, imm[19:12] = 1

		// let expected_imm: i32 = 0b1111_1111_1111_0000_0001_1000_0000_0010;
		let expected_imm = -1042430i32;
		let j_with_imm = J::from_word(word_with_imm);
		assert_eq!(j_with_imm.imm(), expected_imm); // sign-extended 21-bit immediate
	}
}
