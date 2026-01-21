use clap::Parser;
use fuste_ecall_dispatcher::{EcallDispatcher, NoopDispatcher};
use fuste_exit::ExitStatus;
use fuste_exit_system::ExitSystem;
use fuste_interrupt_handler::{InterruptHandler, NoopEbreakDispatcher};
use fuste_lilbug::LilBugComputer;
use fuste_lilbug::LilBugSystem;
use fuste_riscv_core::{
	instructions::Rv32iInstruction,
	machine::{Machine, MachineError, MachineSystem},
	plugins::rv32i_computer::Rv32iComputer,
};
use fuste_riscv_elf::{Elf32Loader, ElfLoaderError};
use fuste_std_output_system::StdOutputSystem;
use fuste_tick_handler::TickHandler;
use std::ops::ControlFlow;
use std::path::PathBuf;

pub struct EcallMachine {
	pub inner: InterruptHandler<
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
	>,
}

impl MachineSystem<BOX_MEMORY_SIZE> for EcallMachine {
	#[inline(always)]
	fn tick(
		&mut self,
		machine: &mut Machine<BOX_MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		self.inner.tick(machine)
	}
}

impl LilBugComputer<BOX_MEMORY_SIZE> for EcallMachine {
	fn exit_status(&self) -> ExitStatus {
		self.inner.ecall_dispatcher.exit_dispatcher.exit_status()
	}
}

pub struct NoEcallMachine {
	pub inner: Rv32iComputer,
}

impl MachineSystem<BOX_MEMORY_SIZE> for NoEcallMachine {
	#[inline(always)]
	fn tick(
		&mut self,
		machine: &mut Machine<BOX_MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		self.inner.tick(machine)
	}
}

impl LilBugComputer<BOX_MEMORY_SIZE> for NoEcallMachine {
	fn exit_status(&self) -> ExitStatus {
		ExitStatus::Unsupported
	}
}

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

	pub fn ticks<Computer: MachineSystem<BOX_MEMORY_SIZE>>(
		&self,
		computer: Computer,
	) -> Result<TickHandler<BOX_MEMORY_SIZE, Computer>, ElfError> {
		let tick_handler = TickHandler {
			inner: computer,
			current_tick: 0,
			max_ticks: self.ticks.unwrap_or(u32::MAX),
		};

		Ok(tick_handler)
	}

	pub fn maybe_run_ticks<Computer: MachineSystem<BOX_MEMORY_SIZE>>(
		&self,
		machine: &mut Machine<BOX_MEMORY_SIZE>,
		mut computer: Computer,
	) -> Result<(), ElfError> {
		if self.ticks.is_some() {
			let mut tick_handler = self.ticks(computer)?;
			machine.run(&mut tick_handler)?;
		} else {
			machine.run(&mut computer)?;
		}

		Ok(())
	}

	pub fn lilbug<Computer: LilBugComputer<BOX_MEMORY_SIZE>>(
		&self,
		computer: Computer,
	) -> Result<LilBugSystem<BOX_MEMORY_SIZE, Computer>, ElfError> {
		let lilbug_system = LilBugSystem {
			computer,
			log_program_counter: self.log_program_counter,
			log_instructions: self.log_instructions,
			log_registers: self.log_registers,
			log_registers_at_end: self.log_registers_at_end,
			log_exit_status: self.log_exit_status,
		};

		Ok(lilbug_system)
	}

	pub fn maybe_run_lilbug<Computer: LilBugComputer<BOX_MEMORY_SIZE>>(
		&self,
		machine: &mut Machine<BOX_MEMORY_SIZE>,
		computer: Computer,
	) -> Result<(), ElfError> {
		if self.is_debug() {
			let lilbug_system = self.lilbug(computer)?;
			self.maybe_run_ticks(machine, lilbug_system)?;
		} else {
			self.maybe_run_ticks(machine, computer)?;
		}

		Ok(())
	}

	pub fn run_ecall_machine(
		&self,
		machine: &mut Machine<BOX_MEMORY_SIZE>,
	) -> Result<(), ElfError> {
		let inner = InterruptHandler::<
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
				open_channel_dispatcher: NoopDispatcher {},
				check_channel_dispatcher: NoopDispatcher {},
			},
			ebreak_dispatcher: NoopEbreakDispatcher {},
		};

		let ecall_machine = EcallMachine { inner };

		self.maybe_run_lilbug(machine, ecall_machine)?;

		Ok(())
	}

	pub fn run_noop_ecall_machine(
		&self,
		machine: &mut Machine<BOX_MEMORY_SIZE>,
	) -> Result<(), ElfError> {
		let noop_ecall_machine = NoEcallMachine { inner: Rv32iComputer };

		self.maybe_run_lilbug(machine, noop_ecall_machine)?;

		Ok(())
	}

	pub fn maybe_run_ecall_machine(
		&self,
		machine: &mut Machine<BOX_MEMORY_SIZE>,
	) -> Result<(), ElfError> {
		// Note we use inner construction because we don't want to
		// wrap in an enum and have lots of inner matching
		// on the branches for every tick.
		if self.ecalls {
			self.run_ecall_machine(machine)?;
		} else {
			self.run_noop_ecall_machine(machine)?;
		}

		Ok(())
	}

	pub async fn execute(&self) -> Result<(), ElfError> {
		// Initialize the machine and loader
		let loader = Elf32Loader::new(self.entrypoint_symbol_name.clone());
		let mut machine = Machine::<BOX_MEMORY_SIZE>::new();

		// Load the ELF file into the machine
		loader.load_elf(&mut machine, &self.path)?;

		// Note we use inner construction because we don't want to
		// wrap in an enum and have lots of inner matching
		// on the branches for every tick.
		//
		// The internals of this function perform the task of
		// composition.
		//
		// The inner loop is monomorphized.
		self.maybe_run_ecall_machine(&mut machine)?;

		Ok(())
	}
}
