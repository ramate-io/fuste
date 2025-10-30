# Fuste

Fuste is a virtual machine stack designed for integration with [Robles](https://github.com/ramate-io/robles), [Ramate's](https://github.com/ramate-io/ramate) implementation of [BFA](https://github.com/ramate-io/bfa) protocols. 

## Getting started 
1. Review the programs in the [`tests/toolchain`](/fuste/tests/toolchain/) workspace before you begin writing your own. 
2. Ensure you have built [`fubox`](/fuste/riscv-box/) and that is available on your `PATH`. 
3. Configure a workspace with the desire [`env/fuste`](/fuste/env/fuste/) crates. 
4. Configure the toolchain similar to [`tests/toolchain`](/fuste/tests/toolchain/riscv32i-ramate-fuste-elf.json). You can change the memory layout in the linker script if you like. 
5. Write your program:

```rust 
#![no_std]
#![no_main]
use fuste::println;

pub fn add(a: u32, b: u32) -> u32 {
	a + b
}

fuste::entry! {
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
}
```
6. Run!

```shell
cargo run --target riscv32i-ramate-fuste-elf.json -p my-fuste-program
```

## Contributing

| Task | Description |
|------|-------------|
| [Upcoming Events](https://github.com/ramate-io/fuste/issues?q=is%3Aissue%20state%3Aopen%20label%3Apriority%3Ahigh%2Cpriority%3Amedium%20label%3Aevent) | High-priority `event` issues with planned completion dates. |
| [Release Candidates](https://github.com/ramate-io/fuste/issues?q=is%3Aissue%20state%3Aopen%20label%3Arelease-candidate) | Feature-complete versions linked to events. |
| [Features & Bugs](https://github.com/ramate-io/fuste/issues?q=is%3Aissue%20state%3Aopen%20label%3Afeature%2Cbug%20label%3Apriority%3Aurgent%2Cpriority%3Ahigh) | High-priority `feature` and `bug` issues. |
