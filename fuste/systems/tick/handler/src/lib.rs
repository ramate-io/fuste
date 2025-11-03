#![no_std]

use core::ops::ControlFlow;
use fuste_riscv_core::machine::{Machine, MachineError, MachineSystem};

/// The [TickHandler] plugin handles ticking the inner machine.
pub struct TickHandler<const MEMORY_SIZE: usize, Inner: MachineSystem<MEMORY_SIZE>> {
	pub inner: Inner,
	pub current_tick: u32,
	pub max_ticks: u32,
}

impl<const MEMORY_SIZE: usize, Inner: MachineSystem<MEMORY_SIZE>> MachineSystem<MEMORY_SIZE>
	for TickHandler<MEMORY_SIZE, Inner>
{
	/// Ticks up to the max ticks or until the inner machine returns a break.
	#[inline(always)]
	fn tick(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		let result = self.inner.tick(machine)?;

		self.current_tick += 1;
		if self.current_tick >= self.max_ticks {
			return Ok(ControlFlow::Break(()));
		} else {
			return Ok(result);
		}
	}
}
