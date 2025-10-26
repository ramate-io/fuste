#![no_std]

pub use fuste_channel::{self as channels, ChannelError, ChannelStatus, ChannelSystemId};
pub use fuste_ecall::{self as ecalls, Ecall, EcallError, EcallStatus};
pub use fuste_exit::{self as exits, exit, ExitError, ExitStatus};
pub use fuste_write::{
	self as writers, write, WriteError, WriteStatus, WriteStatusCode, WriteSystemId,
};
pub mod io;
pub use io::Stdout;
