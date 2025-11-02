#![no_std]
#![allow(unexpected_cfgs)]

use core::fmt::{self, Display};
use fuste_ecall::Ecall;

/// The system ID is a 16-bit value that identifies the system to write to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteSystemId(u32);

impl WriteSystemId {
	pub const fn constant(value: u32) -> Self {
		Self(value)
	}

	pub fn new(value: u32) -> Self {
		Self(value)
	}

	pub fn to_u32(self) -> u32 {
		self.0
	}

	pub const fn to_const_u32(self) -> u32 {
		self.0
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteStatusCode {
	SystemError = -4,
	InvalidSystem = -3,
	Failure = -2,
	Ignored = -1,
	Success = 0,
}

impl Display for WriteStatusCode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Write status code: {}", self.clone().to_i32())
	}
}

impl WriteStatusCode {
	pub fn to_i32(self) -> i32 {
		self as i32
	}

	pub fn try_from_i32(value: i32) -> Result<Self, WriteError> {
		match value {
			-4 => Ok(WriteStatusCode::SystemError),
			-3 => Ok(WriteStatusCode::InvalidSystem),
			-2 => Ok(WriteStatusCode::Failure),
			-1 => Ok(WriteStatusCode::Ignored),
			0 => Ok(WriteStatusCode::Success),
			_ => Err(WriteError::InvalidStatusCode(value)),
		}
	}

	pub fn is_success(self) -> bool {
		self == WriteStatusCode::Success
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteSystemStatus(i32);

impl Display for WriteSystemStatus {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Write system status: {}", self.clone().to_i32())
	}
}

impl WriteSystemStatus {
	pub fn new(value: i32) -> Self {
		Self(value)
	}

	pub fn to_i32(self) -> i32 {
		self.0
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteStatus {
	code: WriteStatusCode,
	system_status: WriteSystemStatus,
}

impl WriteStatus {
	pub fn new(code: WriteStatusCode, system_status: WriteSystemStatus) -> Self {
		Self { code, system_status }
	}

	pub fn try_from_i32s(code: i32, system_status: i32) -> Result<Self, WriteError> {
		Ok(Self {
			code: WriteStatusCode::try_from_i32(code)?,
			system_status: WriteSystemStatus::new(system_status),
		})
	}

	pub fn is_success(self) -> bool {
		self.code.is_success()
	}

	pub fn ok(self) -> Result<WriteStatus, WriteError> {
		match self.code {
			WriteStatusCode::Success => Ok(self),
			WriteStatusCode::SystemError => Err(WriteError::SystemError(self)),
			WriteStatusCode::InvalidSystem => Err(WriteError::InvalidSystem(self)),
			WriteStatusCode::Failure => Err(WriteError::Failure(self)),
			WriteStatusCode::Ignored => Err(WriteError::Ignored(self)),
		}
	}
}

impl Display for WriteStatus {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Write status: code={}, system_status={}", self.code, self.system_status)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteError {
	SystemError(WriteStatus),
	InvalidSystem(WriteStatus),
	Failure(WriteStatus),
	Ignored(WriteStatus),
	InvalidStatusCode(i32),
	NotImplemented,
}

impl Display for WriteError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			WriteError::SystemError(status) => write!(f, "System error: {}", status),
			WriteError::InvalidSystem(status) => write!(f, "Invalid system: {}", status),
			WriteError::Failure(status) => write!(f, "Failure: {}", status),
			WriteError::Ignored(status) => write!(f, "Ignored: {}", status),
			WriteError::InvalidStatusCode(code) => write!(f, "Invalid status code: {}", code),
			WriteError::NotImplemented => write!(f, "Not implemented"),
		}
	}
}

#[inline(never)]
pub fn write(system_id: WriteSystemId, _buffer: &[u8]) -> Result<WriteStatus, WriteError> {
	let _ecall = Ecall::Write.to_u32();
	let _system_id = system_id.to_u32();
	let _status: i32;
	let _system_status: i32;
	let _status_ignored = WriteStatusCode::Ignored.to_i32();

	#[cfg(target_family = "fuste")]
	{
		unsafe {
			core::arch::asm!(
				"ecall",
				in("a7") _ecall,                   // syscall number for write
				in("a0") _system_id,            // file descriptor
				in("a1") _buffer.as_ptr(),      // pointer to buffer
				in("a2") _buffer.len(),         // length
				in("a3") _status_ignored,       // if this isn't reset, the system must have ignored the call
				lateout("a3") _status,           // the status of the operation
				lateout("a4") _system_status,    // the system status of the operation
			);

			let write_status = WriteStatus::try_from_i32s(_status, _system_status)?;
			write_status.ok()
		}
	}

	#[cfg(not(target_family = "fuste"))]
	{
		Err(WriteError::NotImplemented)
	}
}
