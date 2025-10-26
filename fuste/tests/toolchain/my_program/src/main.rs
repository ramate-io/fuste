#![no_std]
#![no_main]

use core::arch::asm;
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
pub fn write<const COUNT: usize>(buffer: &[u8; COUNT]) -> Result<(), ()> {
	unsafe {
		// set the pointer to the buffer
		core::arch::asm!(
			"mv a0, {0}",      // a0 = pointer to buffer
			"li a7, 64",       // syscall number (64 = write)
			"ecall",
			in(reg) buffer,
		);

		// read the a1 pointer as a u32 status
		let status = core::arch::asm!(
			"mv a1, {0}",      // a1 = pointer to status
			"lw a1, 0(a1)",
			in(reg) status,
		);
	}

	if status != 0 {
		return Err(());
	}

	Ok(())
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
	}

	Ok(())
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	exit(1);
}
