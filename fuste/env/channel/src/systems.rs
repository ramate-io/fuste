use crate::{ChannelError, ChannelStatus};

pub trait ChannelSystem {
	fn handle_open(
		&self,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError>;

	fn handle_check(
		&self,
		read_buffer: &[u8],
		write_buffer: &mut [u8],
	) -> Result<ChannelStatus, ChannelError>;
}
