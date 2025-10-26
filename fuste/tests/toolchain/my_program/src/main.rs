#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::{self, Write};
use core::panic::PanicInfo;

#[no_mangle]
fn _start() -> ! {
	unsafe {
		// set stack pointer
		asm!(
			"la sp, {stack}",
			stack = sym _stack_end
		);
	}
	_main();
}

extern "C" {
	static _stack_end: u32;
}

#[no_mangle]
#[inline(never)]
pub fn exit(status: u32) -> ! {
	unsafe {
		core::arch::asm!(
			"mv a0, {0}",      // a0 = pointer to ExitStatus
			"li a7, 93",       // syscall number (93 = exit)
			"ecall",
			in(reg) status,
			options(noreturn)
		);
	}
}

#[no_mangle]
#[inline(never)]
pub fn write(fd: u32, buffer: &[u8]) -> Result<i32, i32> {
	let ret: i32;
	unsafe {
		core::arch::asm!(
			"ecall",
			in("a7") 64,                   // syscall number for write
			in("a0") fd,                   // file descriptor
			in("a1") buffer.as_ptr(),      // pointer to buffer
			in("a2") buffer.len(),         // length
			lateout("a3") ret,             // return value (bytes written or -errno)
		);
	}

	if ret < 0 {
		Err(ret)
	} else {
		Ok(ret)
	}
}

pub struct Stdout;

impl Write for Stdout {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		write(1, s.as_bytes()).map(|_| ()).map_err(|_| fmt::Error)
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

#[no_mangle]
#[inline(never)]
pub extern "C" fn _main() -> ! {
	let _ = main();
	exit(0);
}

pub fn add(a: u32, b: u32) -> u32 {
	a + b
}

fn main() -> Result<(), ()> {
	let mut j = 0;
	for i in 0..10 {
		assert_eq!(i, i);
		j += i;
		j = add(j, i);
		println!("Hello, world!");
		println!("i: {}, j: {}", i, j);
	}

	Ok(())
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	exit(1);
}
