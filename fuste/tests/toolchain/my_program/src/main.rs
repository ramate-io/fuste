#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
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
