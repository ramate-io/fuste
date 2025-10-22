use core::fmt::{self, Write};

pub struct RingBuffer<const N: usize> {
	buf: [u8; N],
	head: usize,
}

impl<const N: usize> RingBuffer<N> {
	pub const fn new() -> Self {
		Self { buf: [0; N], head: 0 }
	}

	pub fn push_byte(&mut self, b: u8) {
		let idx = self.head % N;
		self.buf[idx] = b;
		self.head = self.head.wrapping_add(1);
	}

	pub fn dump(&self, mut f: impl FnMut(u8)) {
		for i in 0..N {
			let idx = (self.head + i) % N;
			f(self.buf[idx]);
		}
	}

	pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
		core::str::from_utf8(&self.buf)
	}
}

impl<const N: usize> Write for RingBuffer<N> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for &b in s.as_bytes() {
			self.push_byte(b);
		}
		Ok(())
	}
}
