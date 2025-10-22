pub mod beq;
pub mod bge;
pub mod bgeu;
pub mod blt;
pub mod bltu;
pub mod bne;

#[derive(Debug)]
pub struct B {
	funct3: u8,
	rs1: u8,
	rs2: u8,
	imm: i32,
}

impl B {
	pub const OPCODE: u32 = 0b1100011;

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
			// bits [31:25] and [11:8] and [7] - B-type immediate reconstruction
			imm: {
				let imm_12 = (word & 0b1000_0000_0000_0000_0000_0000_0000_0000) >> 19; // bit 31 -> imm[12]
				let imm_4_1 = (word & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 7; // bits [11:8] -> imm[4:1]
				let imm_10_5 = (word & 0b0111_1110_0000_0000_0000_0000_0000_0000) >> 20; // bits [30:25] -> imm[10:5]
				let imm_11 = (word & 0b0000_0000_0000_0000_0000_0000_1000_0000) << 4; // bit 7 -> imm[11]

				let imm_raw = imm_12 | imm_11 | imm_10_5 | imm_4_1;

				// Sign extend the 13-bit immediate
				if (imm_raw & 0b0001_0000_0000_0000) != 0 {
					imm_raw | 0b1111_1111_1111_1111_1110_0000_0000_0000
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
	pub fn offset(&self) -> i32 {
		// B-type instructions use the immediate as a branch offset
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
		// B-type immediate reconstruction
		let imm = self.imm as u32;
		let imm_11 = (imm & 0b1000_0000_0000_0000) >> 11; // bit 11
		let imm_4_1 = (imm & 0b0000_0000_0000_1111) >> 1; // bits [4:1]
		let imm_10_5 = (imm & 0b0000_0111_1110_0000) >> 5; // bits [10:5]
		let imm_12 = (imm & 0b0001_0000_0000_0000) >> 12; // bit 12

		(imm_12 << 31) | (imm_10_5 << 25) | (imm_11 << 7) | (imm_4_1 << 8)
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
	fn test_basic_functionality() {
		// Test basic B format functionality
		let b = B::new(0b000, 1, 2, 8); // BEQ x1, x2, 8

		assert_eq!(b.funct3(), 0);
		assert_eq!(b.rs1(), 1);
		assert_eq!(b.rs2(), 2);
		assert_eq!(b.offset(), 8);
		assert_eq!(b.imm(), 8);
	}

	#[test]
	fn test_immediate_extraction_zero() {
		// Test immediate extraction with a known pattern
		let word = 0b0000_0000_0000_0000_0000_0000_0011_0111; // imm=0
		let b = B::from_word(word);
		assert_eq!(b.imm(), 0);
	}

	#[test]
	fn test_immediate_extraction_positive() {
		// Test immediate extraction with a known pattern
		let word =
			0b0000_0000_0000_0000_0000_0000_0011_0111
				| (0 << 31) | (1 << 30)
				| (1 << 11) | (1 << 7); // bit[31] -> imm[12] = 0, bit[30] -> imm[10] = 1, bit[11] -> imm[4] = 1, bit[7] -> imm[11] = 1

		let expected_imm = 0b0000_0000_0000_0000_0000_1100_0001_0000i32;
		let b = B::from_word(word);
		assert_eq!(b.imm(), expected_imm);
	}

	#[test]
	fn test_immediate_extraction_negative() {
		// Test immediate extraction with a known pattern
		let word =
			0b0000_0000_0000_0000_0000_0000_0011_0111
				| (1 << 31) | (1 << 30)
				| (1 << 11) | (1 << 7); // bit[31] -> imm[12] = 1, bit[30] -> imm[10] = 1, bit[11] -> imm[4] = 1, bit[7] -> imm[11] = 1

		// let expected_imm = 0b1111_1111_1111_1111_1111_1100_0001_0000i32;
		let expected_imm = -1008i32;
		let b = B::from_word(word);
		assert_eq!(b.imm(), expected_imm);
	}
}
