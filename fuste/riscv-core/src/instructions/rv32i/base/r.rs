#[derive(Debug)]
pub struct R {
	rd: u8,
	imm: u32,
}

impl R {
	#[inline(always)]
	pub fn from_word(word: u32) -> Self {
		Self {
			rd: ((word & 0b11111000000000000000000000000000) >> 7) as u8,
			imm: (word & 0b11111111111111111111111111111111) >> 12,
		}
	}

	#[inline(always)]
	pub fn rd(&self) -> u8 {
		self.rd
	}

	#[inline(always)]
	pub fn imm(&self) -> u32 {
		self.imm
	}
}
