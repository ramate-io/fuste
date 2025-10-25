pub mod elf;

use clap::Subcommand;

#[derive(Debug, thiserror::Error)]
pub enum RunError {
	#[error("Encountered an error while loading and running an ELF program: {0}")]
	ElfError(#[from] elf::ElfError),
}

#[derive(Subcommand)]
pub enum Run {
	/// Load and run a RISC-V ELF file
	Elf(elf::Elf),
}

impl Run {
	pub async fn execute(&self) -> Result<(), RunError> {
		match self {
			Run::Elf(elf) => elf.execute().await.map_err(RunError::ElfError),
		}
	}
}
