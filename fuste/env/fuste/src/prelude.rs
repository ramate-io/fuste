// The macros will be expanded at the call site where these are available

#[macro_export]
macro_rules! entry {
	(
		$(#[$attrs:meta])*
		$vis:vis fn $name:ident() -> $ret:ty $body:block
	) => {
		// Generate the startup code
		#[no_mangle]
		fn _start() -> ! {
			unsafe {
				// set stack pointer
				::core::arch::asm!(
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
		pub extern "C" fn _main() -> ! {
			let _ = $name();
			$crate::exit($crate::ExitStatus::Success);
		}

		#[panic_handler]
		fn panic(_info: &::core::panic::PanicInfo) -> ! {
			$crate::exit($crate::ExitStatus::Error);
		}

		// Preserve the original function
		$(#[$attrs])*
		$vis fn $name() -> $ret $body
	};
}
