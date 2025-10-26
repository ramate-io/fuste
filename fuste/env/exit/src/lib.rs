#![no_std]
#![allow(unexpected_cfgs)]

use core::fmt::{self, Display};
use fuste_ecall::Ecall;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExitError {
	InvalidExitStatus(u32),
}

impl Display for ExitError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ExitError::InvalidExitStatus(value) => write!(f, "Invalid exit status: {}", value),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExitStatus {
	Success = 0,
	Error = 1,
	Terminated = 2,
}

impl ExitStatus {
	pub fn to_u32(self) -> u32 {
		self as u32
	}

	pub fn try_from_u32(value: u32) -> Result<Self, ExitError> {
		match value {
			0 => Ok(ExitStatus::Success),
			1 => Ok(ExitStatus::Error),
			2 => Ok(ExitStatus::Terminated),
			_ => Err(ExitError::InvalidExitStatus(value)),
		}
	}
}

#[inline(never)]
pub fn exit(status: ExitStatus) -> ! {
	let _ecall = Ecall::Exit.to_u8();
	let _status = status.to_u32();

	#[cfg(target_family = "fuste")]
	{
		unsafe {
			core::arch::asm!(
				"mv a0, {0}",      // a0 = pointer to ExitStatus
				"mv a7, {1}",       // syscall number (93 = exit)
				"ecall",
				in(reg) _status,
				in(reg) _ecall,
				options(noreturn)
			);
		}
	}

	#[cfg(not(target_family = "fuste"))]
	{
		loop {
			// we could make this call to a machine field that
			// is only compiled and available in non fuste targets.
			// that would potentially make testing fun.
		}
	}
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn test_exit_status_try_from_u32() {
		assert_eq!(ExitStatus::try_from_u32(0), Ok(ExitStatus::Success));
		assert_eq!(ExitStatus::try_from_u32(1), Ok(ExitStatus::Error));
		assert_eq!(ExitStatus::try_from_u32(2), Ok(ExitStatus::Terminated));
		assert_eq!(ExitStatus::try_from_u32(3), Err(ExitError::InvalidExitStatus(3)));
	}

	#[test]
	fn test_exit_status_to_u32() {
		assert_eq!(ExitStatus::Success.to_u32(), 0);
		assert_eq!(ExitStatus::Error.to_u32(), 1);
		assert_eq!(ExitStatus::Terminated.to_u32(), 2);
	}
}
