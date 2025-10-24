#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
fn _start() -> ! {
	unsafe {
		// set stack pointer
		asm!("la sp, _stack_end", options(nomem, nostack, preserves_flags));
	}
	_main();
}

extern "C" {
	static _stack_end: u32;
}

pub fn exit() -> ! {
	unsafe {
		asm!("ebreak", options(nomem, nostack, preserves_flags));
	}
	loop {}
}

#[no_mangle]
pub extern "C" fn _main() -> ! {
	let _ = main();
	exit();
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
	}

	Ok(())
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
