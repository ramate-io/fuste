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
	#[error("Symbol name matching entrypoint \"{0}\" not found in ELF file")]
	EntrypointNotFound(String),
}

pub struct Elf32Loader {
	entrypoint_symbol_name: String,
}

impl Default for Elf32Loader {
	fn default() -> Self {
		Self::new("_start".to_string())
	}
}

impl Elf32Loader {
	pub fn new(entrypoint_symbol_name: String) -> Self {
		Self { entrypoint_symbol_name }
	}

	pub fn load_elf<const MEMORY_SIZE: usize>(
		&self,
		machine: &mut Machine<MEMORY_SIZE>,
		path: impl AsRef<Path>,
	) -> Result<(), ElfLoaderError> {
		// Read the ELF file into memory
		let buffer = fs::read(path.as_ref())?;
		let elf = Elf::parse(buffer.as_slice())?;

		// Normal symbol table
		let mut flag_entrypoint_found = false;
		for sym in elf.syms.iter() {
			let name = elf
				.strtab
				.get_at(sym.st_name)
				.ok_or(ElfLoaderError::InvalidSymbolName(sym.st_name.to_string()))?;

			if name == "_start" {
				flag_entrypoint_found = true;
				break;
			}
		}

		if !flag_entrypoint_found {
			return Err(ElfLoaderError::EntrypointNotFound(self.entrypoint_symbol_name.clone()));
		}

		for ph in &elf.program_headers {
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
		machine.registers_mut().program_counter_set(entry);

		Ok(())
	}
}
