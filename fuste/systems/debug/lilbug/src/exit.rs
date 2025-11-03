use crate::LilBugComputer;
use fuste_exit::ExitStatus;
use fuste_exit_system::ExitSystem;

impl<const MEMORY_SIZE: usize> LilBugComputer<MEMORY_SIZE> for ExitSystem<MEMORY_SIZE> {
	fn exit_status(&self) -> ExitStatus {
		self.syscall_status.clone()
	}
}
