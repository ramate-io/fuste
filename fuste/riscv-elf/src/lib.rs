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

pub struct ElfLoader;

impl ElfLoader {
	pub fn load_elf<const MEMORY_SIZE: usize>(
		&self,
		machine: &mut Machine<MEMORY_SIZE>,
		path: impl AsRef<Path>,
	) -> Result<(), ElfLoaderError> {
		// Read the ELF file
		let buffer = fs::read(path.as_ref())?;
		let elf = Elf::parse(buffer.as_slice())?;

		for ph in &elf.program_headers {
			if ph.p_type == goblin::elf::program_header::PT_LOAD {
				let start = ph.p_paddr as usize;
				let data = &buffer[ph.file_range()];
				machine.memory_mut().write_bytes(start as u32, data)?;
				// zero-fill padding up to ph.p_memsz
			}
		}

		Ok(())
	}
}
