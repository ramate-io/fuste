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

		Ok(())
	}
}
