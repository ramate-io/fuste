#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

extern "C" {
	static _stack_end: u32;
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	unsafe {
		// set stack pointer
		asm!("la sp, _stack_end", options(nomem, nostack, preserves_flags));
	}
	main();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
	// Replace with your entrypoint logic
	let mut j = 0;
	for i in 0..10 {
		assert_eq!(i, i);
		j += i;
	}

	panic!("program finished");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
