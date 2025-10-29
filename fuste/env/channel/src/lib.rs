#![no_std]
#![allow(unexpected_cfgs)]

pub mod ops;
use core::fmt::{self, Display};
use fuste_ecall::Ecall;

/// The system ID is a 32-bit value that identifies the system to write to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelSystemId(u32);

impl ChannelSystemId {
	pub const fn constant(value: u32) -> Self {
		Self(value)
	}

	pub fn new(value: u32) -> Self {
		Self(value)
	}

	pub fn to_u32(self) -> u32 {
		self.0
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelStatusCode {
	/// The system returned an error.
	SystemError = -4,
	/// The system is invalid.
	InvalidSystem = -3,
	/// The operation failed.
	Failure = -2,
	/// The operation was ignored.
	Ignored = -1,
	/// The operation completed successfully.
	Success = 0,
	/// The system yielded back to the caller.
	Yielded = 1,
	/// The system is holding the channel, no new data yet.
	Holding = 2,
}

impl Display for ChannelStatusCode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Write status code: {}", self.clone().to_i32())
	}
}

impl ChannelStatusCode {
	pub fn to_i32(self) -> i32 {
		self as i32
	}

	pub fn try_from_i32(value: i32) -> Result<Self, ChannelError> {
		match value {
			-4 => Ok(ChannelStatusCode::SystemError),
			-3 => Ok(ChannelStatusCode::InvalidSystem),
			-2 => Ok(ChannelStatusCode::Failure),
			-1 => Ok(ChannelStatusCode::Ignored),
			0 => Ok(ChannelStatusCode::Success),
			1 => Ok(ChannelStatusCode::Yielded),
			2 => Ok(ChannelStatusCode::Holding),
			_ => Err(ChannelError::InvalidStatusCode(value)),
		}
	}

	pub fn is_success(&self) -> bool {
		*self == ChannelStatusCode::Success
	}

	pub fn has_yielded(&self) -> bool {
		*self == ChannelStatusCode::Yielded
	}

	pub fn is_holding(&self) -> bool {
		*self == ChannelStatusCode::Holding
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelSystemStatus(i32);

impl Display for ChannelSystemStatus {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Write system status: {}", self.clone().to_i32())
	}
}

impl ChannelSystemStatus {
	pub fn new(value: i32) -> Self {
		Self(value)
	}

	pub fn to_i32(self) -> i32 {
		self.0
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelStatus {
	size: u32,
	code: ChannelStatusCode,
	system_status: ChannelSystemStatus,
}

impl ChannelStatus {
	pub fn new(size: u32, code: ChannelStatusCode, system_status: ChannelSystemStatus) -> Self {
		Self { size, code, system_status }
	}

	pub fn try_from_words(size: u32, code: i32, system_status: i32) -> Result<Self, ChannelError> {
		Ok(Self {
			size,
			code: ChannelStatusCode::try_from_i32(code)?,
			system_status: ChannelSystemStatus::new(system_status),
		})
	}

	pub fn is_success(&self) -> bool {
		self.code.is_success()
	}

	pub fn has_yielded(&self) -> bool {
		self.code.has_yielded()
	}

	pub fn is_holding(&self) -> bool {
		self.code.is_holding()
	}

	pub fn ok(self) -> Result<ChannelStatus, ChannelError> {
		match self.code {
			ChannelStatusCode::Success => Ok(self),
			ChannelStatusCode::Holding => Ok(self),
			ChannelStatusCode::Yielded => Ok(self),
			ChannelStatusCode::SystemError => Err(ChannelError::SystemError(self)),
			ChannelStatusCode::InvalidSystem => Err(ChannelError::InvalidSystem(self)),
			ChannelStatusCode::Failure => Err(ChannelError::Failure(self)),
			ChannelStatusCode::Ignored => Err(ChannelError::Ignored(self)),
		}
	}
}

impl Display for ChannelStatus {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Write status: code={}, system_status={}", self.code, self.system_status)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelError {
	BufferTooSmall(ChannelStatus),
	SystemError(ChannelStatus),
	InvalidSystem(ChannelStatus),
	Failure(ChannelStatus),
	Ignored(ChannelStatus),
	InvalidStatusCode(i32),
	NotImplemented,
}

impl Display for ChannelError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ChannelError::SystemError(status) => write!(f, "System error: {}", status),
			ChannelError::InvalidSystem(status) => write!(f, "Invalid system: {}", status),
			ChannelError::Failure(status) => write!(f, "Failure: {}", status),
			ChannelError::Ignored(status) => write!(f, "Ignored: {}", status),
			ChannelError::InvalidStatusCode(code) => write!(f, "Invalid status code: {}", code),
			ChannelError::NotImplemented => write!(f, "Not implemented"),
			ChannelError::BufferTooSmall(status) => write!(f, "Buffer too small: {}", status),
		}
	}
}

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelOp {
	Write = Ecall::WriteChannel as u32,
	Read = Ecall::ReadChannel as u32,
}

#[inline(never)]
pub fn channel_op(
	system_id: ChannelSystemId,
	op: ChannelOp,
	_buffer: &mut [u8],
) -> Result<ChannelStatus, ChannelError> {
	let _ecall = op as u32;
	let _system_id = system_id.to_u32();
	let _size: u32;
	let _status: i32;
	let _system_status: i32;
	let _status_ignored = ChannelStatusCode::Ignored.to_i32();

	#[cfg(target_family = "fuste")]
	{
		unsafe {
			core::arch::asm!(
				"ecall",
				in("a7") _ecall,                // syscall number for channel operation
				in("a0") _system_id,            // the channel system id
				in("a1") _buffer.as_ptr(),      // pointer to buffer
				in("a2") _buffer.len(),         // length
				in("a3") _status_ignored,       // if this isn't reset, the system must have ignored the call
				lateout("a3") _status,          // return value (bytes written or -errno)
				lateout("a4") _size,            // the size of the buffer written
				lateout("a5") _system_status,   // the system status of the operation
			);
		}

		let channel_status = ChannelStatus::try_from_words(_size, _status, _system_status)?;
		channel_status.ok()
	}

	#[cfg(not(target_family = "fuste"))]
	{
		Err(ChannelError::NotImplemented)
	}
}

/// Writes to a channel and returns the immediate status of the operation.
pub fn write(system_id: ChannelSystemId, buffer: &mut [u8]) -> Result<ChannelStatus, ChannelError> {
	channel_op(system_id, ChannelOp::Write, buffer)
}

/// Reads from a channel providing the immediate status of the operation.
pub fn read(system_id: ChannelSystemId, buffer: &mut [u8]) -> Result<ChannelStatus, ChannelError> {
	channel_op(system_id, ChannelOp::Read, buffer)
}
