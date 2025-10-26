#![no_std]

use core::fmt::{self, Display};

#[derive(Debug)]
pub enum EcallError {
	InvalidEcall(u32),
}

impl Display for EcallError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			EcallError::InvalidEcall(value) => write!(f, "Invalid ecall: {}", value),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ecall {
	Exit = 93,
	Write = 64,
	WriteChannel = 33,
	ReadChannel = 34,
}

impl Ecall {
	pub fn to_u32(self) -> u32 {
		self as u32
	}

	pub fn try_from_u32(value: u32) -> Result<Self, EcallError> {
		match value {
			93 => Ok(Ecall::Exit),
			64 => Ok(Ecall::Write),
			33 => Ok(Ecall::WriteChannel),
			34 => Ok(Ecall::ReadChannel),
			_ => Err(EcallError::InvalidEcall(value)),
		}
	}
}

#[derive(Debug, Clone)]
pub enum EcallStatus {
	Success = 0,
	Error = 1,
	NotImplemented = 2,
}
