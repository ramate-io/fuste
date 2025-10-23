use clap::Parser;
use fuste_riscv_core::{
	machine::{Machine, MachineError},
	plugins::rv32i_computer::Rv32iComputer,
};
use fuste_riscv_elf::{Elf32Loader, ElfLoaderError};
use std::path::PathBuf;

const BOX_MEMORY_SIZE: usize = 1024 * 1024 * 2; // 8MB

#[derive(Debug, thiserror::Error)]
pub enum ElfError {
	#[error("Encountered an error while loading the ELF file: {0}")]
	LoaderError(#[from] ElfLoaderError),
	#[error("Encountered an error while running the machine: {0}")]
	MachineError(#[from] MachineError),
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Elf {
	/// The path to the ELF file to run
	#[clap(long)]
	pub path: PathBuf,
}

impl Elf {
	pub async fn execute(&self) -> Result<(), ElfError> {
		// Initialize the machine and loader
		println!("Loading ELF file: {}", self.path.display());
		let loader = Elf32Loader::new();
		let mut machine = Machine::<BOX_MEMORY_SIZE>::new();

		// Load the ELF file into the machine
		println!("Loading ELF file into machine...");
		loader.load_elf(&mut machine, &self.path)?;

		// Initialize the plugin and run the machine
		println!("Running machine...");
		let mut plugin = Rv32iComputer;
		machine.run(&mut plugin).map_err(ElfError::MachineError)?;

		Ok(())
	}
}
