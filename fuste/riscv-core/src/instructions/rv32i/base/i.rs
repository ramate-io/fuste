pub mod addi;
pub mod andi;
pub mod jalr;
pub mod lb;
pub mod lbu;
pub mod lh;
pub mod lhu;
pub mod lw;
pub mod ori;
pub mod slli;
pub mod slti;
pub mod sltiu;
pub mod srai;
pub mod srli;
pub mod xori;

#[derive(Debug)]
pub struct I {
	rd: u8,
	funct3: u8,
	rs1: u8,
	imm: i32,
}

impl I {
	pub const OPCODE: u32 = 0b0010011;

	#[inline(always)]
	pub fn new(rd: u8, funct3: u8, rs1: u8, imm: i32) -> Self {
		Self { rd, funct3, rs1, imm }
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
			// bits [31:20] - 12-bit immediate, sign-extended
			imm: {
				let imm_raw = (word & 0b1111_1111_1111_0000_0000_0000_0000_0000) >> 20;
				// Sign extend the 12-bit immediate
				if (imm_raw & 0b1000_0000_0000) != 0 {
					imm_raw | 0b1111_1111_1111_1111_1111_0000_0000_0000
				} else {
					imm_raw
				}
			} as i32,
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
	pub fn imm(&self) -> i32 {
		self.imm
	}

	#[inline(always)]
	pub fn shamt(&self) -> u8 {
		// For shift instructions, the shift amount is in the lower 5 bits of imm
		(self.imm & 0b11111) as u8
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
	pub fn word_imm(&self) -> u32 {
		// occupies bits [31:20]
		(self.imm as u32) << 20
	}

	#[inline(always)]
	pub fn to_word(&self, opcode: u32) -> u32 {
		// occupies the bits [6:0]
		let word_opcode = opcode & 0b0000_0000_0000_0000_0000_0000_0111_1111;

		word_opcode | self.word_rd() | self.word_funct3() | self.word_rs1() | self.word_imm()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_word_and_to_word() {
		// Test with a known ADDI instruction encoding
		let original_word = 0b0000_0000_0000_0000_0000_0000_1001_0011; // ADDI x1, x0, 0
		let i = I::from_word(original_word);
		let reconstructed_word = i.to_word(0b0010011);

		assert_eq!(reconstructed_word, original_word);
	}

	#[test]
	fn test_field_extraction() {
		// Test field extraction with known values
		let word = 0b0000_0000_0000_0000_0000_0000_1001_0011; // ADDI x1, x0, 0
		let i = I::from_word(word);

		assert_eq!(i.rd(), 1);
		assert_eq!(i.funct3(), 0);
		assert_eq!(i.rs1(), 0);
		assert_eq!(i.imm(), 0);
	}

	#[test]
	fn test_immediate_sign_extension() {
		// Test negative immediate
		let word = 0b1111_1111_1111_0000_0000_0000_1001_0011; // ADDI x1, x0, -1
		let i = I::from_word(word);

		assert_eq!(i.imm(), -1);
	}

	#[test]
	fn test_shamt_extraction() {
		// Test shift amount extraction for SLLI

		//              |f7         |shamt  |rd      |rs1  |f3   |opcode
		let word = 0b000_0000__0_0001__0_0000___000__00011__0010011; // SLLI x1, x0, 1
		let i = I::from_word(word);

		assert_eq!(i.shamt(), 1);
	}
}
