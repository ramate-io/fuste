use crate::{write, WriteSystemId};
use core::fmt::{self, Write};
pub use fuste_channel::ops::{block_on_channel, block_on_channel_request};

pub struct Stdout;

impl Stdout {
	pub const SYSTEM_ID: WriteSystemId = WriteSystemId::constant(1);
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
