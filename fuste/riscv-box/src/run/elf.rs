use clap::Parser;
use fuste_ecall_dispatcher::{EcallDispatcher, NoopDispatcher};
use fuste_exit_system::ExitSystem;
use fuste_interrupt_handler::{InterruptHandler, NoopEbreakDispatcher};
use fuste_riscv_core::{
	instructions::{EcallInterrupt, ExecutableInstructionError, Rv32iInstruction},
	machine::{Machine, MachineError, MachineSystem},
	plugins::rv32i_computer::Rv32iComputer,
};
use fuste_riscv_elf::{Elf32Loader, ElfLoaderError};
use fuste_std_output_system::StdOutputSystem;
use std::ops::ControlFlow;
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
	/// The name of the entrypoint symbol to load
	#[clap(long, default_value = "_start")]
	pub entrypoint_symbol_name: String,
	/// Whether to support ecalls
	#[clap(long, default_value = "true")]
	pub ecalls: bool,
	/// Whether to support std-output
	#[clap(long, default_value = "true")]
	pub std_output: bool,
	/// Whether to log the exit status
	#[clap(long)]
	pub log_exit_status: bool,
}

pub struct DebugSystem {
	computer: Rv32iComputer,
	log_program_counter: bool,
	log_instructions: bool,
	log_registers: bool,
}

impl MachineSystem<BOX_MEMORY_SIZE> for DebugSystem {
	fn tick(
		&mut self,
		machine: &mut Machine<BOX_MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
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
				MachineError::SystemError("Failed to decode instruction for debugger")
			})?;
			println!("0x{address:08X}: {:40} <- 0b{:032b}", decoded_instruction, instruction);
		}
		self.computer.tick(machine)
	}
}

impl Elf {
	pub fn is_debug(&self) -> bool {
		self.log_program_counter
			|| self.log_instructions
			|| self.log_registers
			|| self.log_registers_at_end
			|| self.log_exit_status
	}

	pub async fn execute(&self) -> Result<(), ElfError> {
		// Initialize the machine and loader
		let loader = Elf32Loader::new(self.entrypoint_symbol_name.clone());
		let mut machine = Machine::<BOX_MEMORY_SIZE>::new();

		// Load the ELF file into the machine
		loader.load_elf(&mut machine, &self.path)?;

		if self.ecalls {
			let ecall_machine = InterruptHandler::<
				BOX_MEMORY_SIZE,
				Rv32iComputer,
				EcallDispatcher<
					BOX_MEMORY_SIZE,
					ExitSystem<BOX_MEMORY_SIZE>,
					Option<StdOutputSystem<BOX_MEMORY_SIZE>>,
					NoopDispatcher<BOX_MEMORY_SIZE>,
					NoopDispatcher<BOX_MEMORY_SIZE>,
				>,
				NoopEbreakDispatcher<BOX_MEMORY_SIZE>,
			> {
				inner: Rv32iComputer,
				ecall_dispatcher: EcallDispatcher {
					exit_dispatcher: ExitSystem::new(),
					write_dispatcher: if self.std_output {
						Some(StdOutputSystem::<BOX_MEMORY_SIZE>)
					} else {
						None
					},
					write_channel_dispatcher: NoopDispatcher {},
					read_channel_dispatcher: NoopDispatcher {},
				},
				ebreak_dispatcher: NoopEbreakDispatcher {},
			};
		}

		Ok(())
	}
}

pub fn handle_ecall_interrupt(
	error: EcallInterrupt,
	machine: &mut Machine<BOX_MEMORY_SIZE>,
) -> Result<ControlFlow<()>, ElfError> {
	let syscall_number = machine.csrs().registers().get(17);
	if syscall_number == 93 {
		let syscall_status_address = machine.csrs().registers().get(10);
		let syscall_status = machine
			.memory()
			.read_word(syscall_status_address)
			.map_err(MachineError::MemoryError)?;
		println!("Program exited with status: {}", syscall_status);
		if syscall_status == 0 {
			Ok(ControlFlow::Break(()))
		} else {
			Err(ElfError::MachineError(MachineError::InstructionError(
				ExecutableInstructionError::EcallInterrupt(error),
			)))
		}
	} else if syscall_number == 64 {
		let write_fd = machine.csrs().registers().get(10);
		let write_buffer_address = machine.csrs().registers().get(11);
		let write_buffer_length = machine.csrs().registers().get(12);
		let write_buffer = machine
			.memory()
			.read_bytes(write_buffer_address, write_buffer_length)
			.map_err(MachineError::MemoryError)?;

		if write_fd == 1 {
			// print the write buffer to stdout
			print!("{}", String::from_utf8_lossy(write_buffer));
			// write 0 to the result register a3
			machine.csrs_mut().registers_mut().set(13, 0);
			machine.csrs_mut().registers_mut().program_counter_mut().increment();
			machine.commit_csrs();
		} else {
			return Err(ElfError::MachineError(MachineError::InstructionError(
				ExecutableInstructionError::EcallInterrupt(error),
			)));
		}

		Ok(ControlFlow::Continue(()))
	} else {
		Err(ElfError::MachineError(MachineError::InstructionError(
			ExecutableInstructionError::EcallInterrupt(error),
		)))
	}
}
