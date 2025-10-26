use super::registers::Registers;

/// Note: these are not RISC-V CSRs, but an adaption for the
/// current state of the virtual machine which does not implement privileges
/// directly--instead resorting more typically to a plugin design pattern.
///
/// We may eventually extend this to be a LIFO queue of CSRs.
#[derive(Debug, Clone)]
pub struct Csrs {
	/// The program counter at the point of the trap.
	epc: u32,
	/// The trap cause.
	cause: u32,
	/// The value of the registers at the time of the trap.
	///
	/// This snapshotting sets up for a multi-hart architecture.
	/// Each hart can have its own registers, but access the same memory.
	/// Eventually, doing this will require reserving memory for the stacks of different harts.
	///
	/// +------------------------+  <- top of RAM
	/// | Hart 0 stack           | Reserved and entry set up by the linker script.
	/// +------------------------+
	/// | Hart 1 stack           | Reserved by linker script. Enterted by trap.
	/// +------------------------+
	/// | Hart 2 stack           |
	/// +------------------------+
	/// | Shared heap / data     |
	/// +------------------------+
	/// | .bss / .data / .text   |
	/// +------------------------+
	/// | ROM / MMIO / reserved  |
	/// +------------------------+
	registers: Registers,
}

impl Csrs {
	pub fn new() -> Self {
		Self { epc: 0, cause: 0, registers: Registers::new() }
	}
}

impl Csrs {
	pub fn epc(&self) -> u32 {
		self.epc
	}

	pub fn cause(&self) -> u32 {
		self.cause
	}

	pub fn registers(&self) -> &Registers {
		&self.registers
	}

	pub fn registers_mut(&mut self) -> &mut Registers {
		&mut self.registers
	}

	pub fn epc_set(&mut self, value: u32) {
		self.epc = value;
	}

	pub fn cause_set(&mut self, value: u32) {
		self.cause = value;
	}

	pub fn registers_set(&mut self, value: Registers) {
		self.registers = value;
	}
}
