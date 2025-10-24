#[derive(Debug)]
pub struct S {
	funct3: u8,
	rs1: u8,
	rs2: u8,
	imm: i32,
}

impl S {
	pub const OPCODE: u32 = 0b0100011;

	#[inline(always)]
	pub fn new(funct3: u8, rs1: u8, rs2: u8, imm: i32) -> Self {
		Self { funct3, rs1, rs2, imm }
	}

	#[inline(always)]
	pub fn from_word(word: u32) -> Self {
		Self {
			// bits [14:12]
			funct3: ((word & 0b0000_0000_0000_0000_0111_0000_0000_0000) >> 12) as u8,
			// bits [19:15]
			rs1: ((word & 0b0000_0000_0000_1111_1000_0000_0000_0000) >> 15) as u8,
			// bits [24:20]
			rs2: ((word & 0b0000_0000_1111_1000_0000_0000_0000_0000) >> 20) as u8,
			// bits [31:25] and [11:7] - S-type immediate reconstruction
			imm: {
				let imm_11_5 = (word & 0b1111_1110_0000_0000_0000_0000_0000_0000) >> 20; // bits [31:25] -> imm[11:5]
				let imm_4_0 = (word & 0b0000_0000_0000_0000_0000_1111_1000_0000) >> 7; // bits [11:7] -> imm[4:0]

				let imm_raw = imm_11_5 | imm_4_0;

				// Sign extend the 12-bit immediate
				if (imm_raw & 0b0000_0000_0000_1000_0000_0000) != 0 {
					imm_raw | 0b1111_1111_1111_1111_1111_0000_0000_0000
				} else {
					imm_raw
				}
			} as i32,
		}
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
	pub fn imm(&self) -> i32 {
		self.imm
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
	pub fn word_imm(&self) -> u32 {
		// S-type immediate reconstruction across [31:25] and [11:7]
		let imm = self.imm as u32;
		let imm_11_5 = (imm & 0b0000_0000_0000_1111_1110_0000) << 20; // imm[11:5]
		let imm_4_0 = (imm & 0b0000_0000_0000_0000_0001_1111) << 7; // imm[4:0]

		imm_11_5 | imm_4_0
	}

	#[inline(always)]
	pub fn to_word(&self, opcode: u32) -> u32 {
		// occupies the bits [6:0]
		let word_opcode = opcode & 0b0000_0000_0000_0000_0000_0000_0111_1111;

		word_opcode | self.word_funct3() | self.word_rs1() | self.word_rs2() | self.word_imm()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_word_and_to_word_zero() {
		// Test with a minimal S encoding (all fields zero)
		let original_word = 0b0000_0000_0000_0000_0000_0000_0010_0011; // opcode 0100011
		let s = S::from_word(original_word);
		let reconstructed_word = s.to_word(S::OPCODE);

		assert_eq!(reconstructed_word, original_word);
	}

	#[test]
	fn test_roundtrip_fields() {
		let s = S::new(0b010, 4, 3, 0x7F); // SW x3, 127(x4)
		let word = s.to_word(S::OPCODE);
		let decoded = S::from_word(word);

		assert_eq!(decoded.funct3(), 0b010);
		assert_eq!(decoded.rs1(), 4);
		assert_eq!(decoded.rs2(), 3);
		assert_eq!(decoded.imm(), 0x7F);
	}

	#[test]
	fn test_immediate_sign_extension_negative() {
		// Build a word with imm = -1 (all 12 immediate bits set)
		let word = (0b111_1111 << 25) | (0b1_1111 << 7) | S::OPCODE; // funct3/rs1/rs2 zero
		let s = S::from_word(word);
		assert_eq!(s.imm(), -1);
	}

	#[test]
	fn test_immediate_sign_extension_positive() {
		// Build a word with imm = 1 (all 12 immediate bits set)
		let word = (0b000_0000 << 25) | (0b0_0001 << 7) | S::OPCODE; // funct3/rs1/rs2 zero
		let s = S::from_word(word);
		assert_eq!(s.imm(), 1);
	}
}
