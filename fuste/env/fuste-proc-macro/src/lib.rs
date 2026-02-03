use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Attribute macro: #[entry]
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
	// Parse the function the user wrote
	let input_fn = parse_macro_input!(item as ItemFn);

	let fn_name = &input_fn.sig.ident;
	let fn_vis = &input_fn.vis;
	let fn_attrs = &input_fn.attrs;
	let fn_block = &input_fn.block;
	let fn_ret = &input_fn.sig.output;

	// Generate expanded code
	let expanded = quote! {

		// Startup entry point
		#[no_mangle]
		fn _start() -> ! {
			unsafe {
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

		// Main trampoline
		#[no_mangle]
		#[inline(never)]
		pub extern "C" fn _main() -> ! {
			let _ = #fn_name();
			fuste::exit(fuste::ExitStatus::Success);
		}

		// Panic handler
		#[panic_handler]
		fn panic(_info: &::core::panic::PanicInfo) -> ! {
			fuste::exit(fuste::ExitStatus::Error);
		}

		// Preserve original user function
		#(#fn_attrs)*
		#fn_vis fn #fn_name() #fn_ret
		#fn_block
	};

	TokenStream::from(expanded)
}
