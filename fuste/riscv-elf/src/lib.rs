use fuste_riscv_core::machine::memory::MemoryError;
use fuste_riscv_core::machine::Machine;
use goblin::elf::Elf;
use std::fs;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum ElfLoaderError {
	#[error("Failed to load ELF file: {0}")]
	Io(#[from] std::io::Error),
	#[error("Failed to parse ELF file: {0}")]
	Parse(#[from] goblin::error::Error),
	#[error("Elf too large to load into memory. Memory size is {memory_size} bytes, but elf is {elf_size} bytes.")]
	ElfTooLarge { memory_size: usize, elf_size: usize },
	#[error("Failed to write bytes to memory: {0}")]
	MemoryError(#[from] MemoryError),
	#[error("Invalid symbol name: {0}")]
	InvalidSymbolName(String),
}

pub struct Elf32Loader;

impl Elf32Loader {
	pub fn new() -> Self {
		Self
	}

	pub fn load_elf<const MEMORY_SIZE: usize>(
		&self,
		machine: &mut Machine<MEMORY_SIZE>,
		path: impl AsRef<Path>,
	) -> Result<(), ElfLoaderError> {
		// Read the ELF file into memory
		let buffer = fs::read(path.as_ref())?;
		let elf = Elf::parse(buffer.as_slice())?;

		println!("Symbols in ELF:");

		// Normal symbol table
		for sym in elf.syms.iter() {
			let name = elf
				.strtab
				.get_at(sym.st_name)
				.ok_or(ElfLoaderError::InvalidSymbolName(sym.st_name.to_string()))?;
			// println!("0x{:08X} {} {:?} size {}", sym.st_value, name, sym.st_type(), sym.st_size);

			if name == "_start" {
				let start = sym.st_value as usize;
				let end = start + sym.st_size as usize;
				println!("_start bytes: {:02X?}", &buffer[start..end]);
			}
		}

		// Dynamic symbols (if present)
		for sym in elf.dynsyms.iter() {
			let name = elf
				.dynstrtab
				.get_at(sym.st_name)
				.ok_or(ElfLoaderError::InvalidSymbolName(sym.st_name.to_string()))?;
			println!("0x{:08X} {} {:?} size {}", sym.st_value, name, sym.st_type(), sym.st_size);
		}

		for ph in &elf.program_headers {
			println!("Loading program header: {:?}", ph);
			if ph.p_type != goblin::elf::program_header::PT_LOAD {
				continue;
			}

			let start = ph.p_paddr as usize;
			let file_size = ph.p_filesz as usize;
			let mem_size = ph.p_memsz as usize;

			// Write the file portion
			let data = &buffer[ph.file_range()];
			machine.memory_mut().write_bytes(start as u32, data)?;

			// Zero-fill remaining memory if necessary
			if mem_size > file_size {
				let padding_size = mem_size - file_size;
				let zero_padding = vec![0u8; padding_size];
				machine.memory_mut().write_bytes((start + file_size) as u32, &zero_padding)?;
			}
		}

		// set the program counter
		let entry = elf.entry as u32;
		println!("Setting program counter to 0x{:X}", entry);
		machine.registers_mut().program_counter_set(entry);

		Ok(())
	}
}
