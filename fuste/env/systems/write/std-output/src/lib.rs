#![no_std]

use core::fmt::{self, Write};
use fuste_write::{write, WriteSystemId};
pub struct Stdout;

impl Stdout {
	pub const SYSTEM_ID: WriteSystemId = WriteSystemId::constant(1);

	pub const fn to_const_u32() -> u32 {
		Self::SYSTEM_ID.to_const_u32()
	}
}

impl Write for Stdout {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		write(Self::SYSTEM_ID, s.as_bytes()).map(|_| ()).map_err(|_| fmt::Error)
	}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = core::write!($crate::Stdout, $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    () => ({
        $crate::print!("\n");
    });
    ($($arg:tt)*) => ({
        $crate::print!($($arg)*);
        $crate::print!("\n");
    });
}
