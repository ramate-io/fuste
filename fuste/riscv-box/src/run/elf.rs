use clap::Parser;
use fuste_riscv_core::{
	instructions::{EcallInterrupt, ExecutableInstructionError, Rv32iInstruction},
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
	/// Whether to log the program counter
	#[clap(long)]
	pub log_program_counter: bool,
	/// Whether to log the instructions
	#[clap(long)]
	pub log_instructions: bool,
	/// Whether to log the registers
	#[clap(long)]
	pub log_registers: bool,
	/// Whether to log the registers at the end of the execution
	#[clap(long)]
	pub log_registers_at_end: bool,
}

pub struct DebugPlugin {
	computer: Rv32iComputer,
	log_program_counter: bool,
	log_instructions: bool,
	log_registers: bool,
}

impl MachinePlugin<BOX_MEMORY_SIZE> for DebugPlugin {
	fn tick(&mut self, machine: &mut Machine<BOX_MEMORY_SIZE>) -> Result<(), MachineError> {
		if self.log_program_counter {
			println!("program counter: 0x{:X}", machine.registers().program_counter());
		}
		if self.log_registers {
			println!("registers: {:?}", machine.registers());
		}

		let address = machine.registers().program_counter();
		let instruction = machine.memory().read_word(address).map_err(MachineError::MemoryError)?;
		if self.log_instructions {
			let decoded_instruction = Rv32iInstruction::<BOX_MEMORY_SIZE>::from_word(instruction)
				.map_err(|_e| {
				MachineError::PluginError("Failed to decode instruction for debugger")
			})?;
			println!("0x{address:08X}: {:40} <- 0b{:032b}", decoded_instruction, instruction);
		}
		self.computer.tick(machine)?;
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
		let mut plugin = DebugPlugin {
			computer: Rv32iComputer,
			log_program_counter: self.log_program_counter,
			log_instructions: self.log_instructions,
			log_registers: self.log_registers,
		};
		match self.ticks {
			Some(ticks) => {
				for i in 0..ticks {
					print!("Tick {}: ", i);
					match plugin.tick(&mut machine) {
						Ok(()) => (),
						Err(MachineError::InstructionError(
							ExecutableInstructionError::EcallInterrupt(error),
						)) => {
							handle_ecall_interrupt(error, &mut machine)?;
						}
						Err(e) => return Err(ElfError::MachineError(e)),
					}
				}
			}
			None => match machine.run(&mut plugin) {
				Ok(()) => (),
				Err(MachineError::InstructionError(
					ExecutableInstructionError::EcallInterrupt(error),
				)) => {
					handle_ecall_interrupt(error, &mut machine)?;
				}
				Err(e) => return Err(ElfError::MachineError(e)),
			},
		}

		if self.log_registers_at_end {
			println!("registers at end: {:?}", machine.registers());
		}

		Ok(())
	}
}

pub fn handle_ecall_interrupt(
	error: EcallInterrupt,
	machine: &mut Machine<BOX_MEMORY_SIZE>,
) -> Result<(), ElfError> {
	let syscall_number = machine.csrs().registers().get(17);
	if syscall_number == 93 {
		let syscall_status_address = machine.csrs().registers().get(10);
		let syscall_status = machine
			.memory()
			.read_word(syscall_status_address)
			.map_err(MachineError::MemoryError)?;
		println!("Program exited with status: {}", syscall_status);
		if syscall_status == 0 {
			Ok(())
		} else {
			Err(ElfError::MachineError(MachineError::InstructionError(
				ExecutableInstructionError::EcallInterrupt(error),
			)))
		}
	} else {
		Err(ElfError::MachineError(MachineError::InstructionError(
			ExecutableInstructionError::EcallInterrupt(error),
		)))
	}
}
