use clap::Parser;
use fuste_riscv_core::{
	instructions::Rv32iInstruction,
	machine::{Machine, MachineError, MachinePlugin},
	plugins::rv32i_computer::Rv32iComputer,
};
use fuste_riscv_elf::{Elf32Loader, ElfLoaderError};
use std::path::PathBuf;

const BOX_MEMORY_SIZE: usize = 1024 * 1024 * 2; // 1MB

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
	/// The number of ticks to run the machine for
	#[clap(long)]
	pub ticks: Option<u32>,
}

pub struct DebugPlugin(Rv32iComputer);

impl MachinePlugin<BOX_MEMORY_SIZE> for DebugPlugin {
	fn tick(&mut self, machine: &mut Machine<BOX_MEMORY_SIZE>) -> Result<(), MachineError> {
		let address = machine.registers().program_counter();
		let instruction = machine.memory().read_word(address).map_err(MachineError::MemoryError)?;
		let decoded_instruction = Rv32iInstruction::<BOX_MEMORY_SIZE>::from_word(instruction)
			.map_err(|_e| MachineError::PluginError("Failed to decode instruction for debugger"))?;
		println!("0x{:X}: 0b{:b} -> {}", address, instruction, decoded_instruction);

		self.0.tick(machine)?;
		Ok(())
	}
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
		let mut plugin = DebugPlugin(Rv32iComputer);
		match self.ticks {
			Some(ticks) => {
				for i in 0..ticks {
					println!("Tick {}:", i);
					plugin.tick(&mut machine)?;
				}
			}
			None => {
				machine.run(&mut plugin).map_err(ElfError::MachineError)?;
			}
		}

		Ok(())
	}
}
