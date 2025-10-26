#![no_std]

use core::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcallError {
	InvalidEcall(u8),
	InvalidEcallStatus(u32),
}

impl Display for EcallError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			EcallError::InvalidEcall(value) => write!(f, "Invalid ecall: {}", value),
			EcallError::InvalidEcallStatus(value) => write!(f, "Invalid ecall status: {}", value),
		}
	}
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ecall {
	Exit = 93,
	Write = 64,
	WriteChannel = 33,
	ReadChannel = 34,
}

impl Ecall {
	pub fn to_u8(self) -> u8 {
		self as u8
	}

	pub fn try_from_u8(value: u8) -> Result<Self, EcallError> {
		match value {
			93 => Ok(Ecall::Exit),
			64 => Ok(Ecall::Write),
			33 => Ok(Ecall::WriteChannel),
			34 => Ok(Ecall::ReadChannel),
			_ => Err(EcallError::InvalidEcall(value)),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcallStatus {
	Success = 0,
	Error = 1,
	NotImplemented = 2,
}

impl EcallStatus {
	pub fn to_u32(self) -> u32 {
		self as u32
	}

	pub fn try_from_u32(value: u32) -> Result<Self, EcallError> {
		match value {
			0 => Ok(EcallStatus::Success),
			1 => Ok(EcallStatus::Error),
			2 => Ok(EcallStatus::NotImplemented),
			_ => Err(EcallError::InvalidEcallStatus(value)),
		}
	}
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn test_ecall_try_from_u32() {
		assert_eq!(Ecall::try_from_u8(93), Ok(Ecall::Exit));
		assert_eq!(Ecall::try_from_u8(64), Ok(Ecall::Write));
		assert_eq!(Ecall::try_from_u8(33), Ok(Ecall::WriteChannel));
		assert_eq!(Ecall::try_from_u8(34), Ok(Ecall::ReadChannel));
		assert_eq!(Ecall::try_from_u8(35), Err(EcallError::InvalidEcall(35)));
	}

	#[test]
	fn test_ecall_to_u8() {
		assert_eq!(Ecall::Exit.to_u8(), 93);
		assert_eq!(Ecall::Write.to_u8(), 64);
		assert_eq!(Ecall::WriteChannel.to_u8(), 33);
		assert_eq!(Ecall::ReadChannel.to_u8(), 34);
	}

	#[test]
	fn test_ecall_status_try_from_u32() {
		assert_eq!(EcallStatus::try_from_u32(0), Ok(EcallStatus::Success));
		assert_eq!(EcallStatus::try_from_u32(1), Ok(EcallStatus::Error));
		assert_eq!(EcallStatus::try_from_u32(2), Ok(EcallStatus::NotImplemented));
		assert_eq!(EcallStatus::try_from_u32(3), Err(EcallError::InvalidEcallStatus(3)));
	}

	#[test]
	fn test_ecall_status() {
		assert_eq!(EcallStatus::Success.to_u32(), 0);
		assert_eq!(EcallStatus::Error.to_u32(), 1);
		assert_eq!(EcallStatus::NotImplemented.to_u32(), 2);
	}
}
