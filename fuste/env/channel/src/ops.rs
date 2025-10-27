use super::{read, write, ChannelError, ChannelStatus, ChannelSystemId};

/// Writes to a channel once and blocks until the operation is complete.
pub fn block_on_channel(
	system_id: ChannelSystemId,
	buffer: &mut [u8],
) -> Result<ChannelStatus, ChannelError> {
	let status = write(system_id.clone(), buffer)?;

	// Success means completion
	if status.is_success() {
		Ok(status.clone())
	} else {
		loop {
			let status = read(system_id.clone(), buffer)?;
			if status.is_success() {
				return Ok(status);
			}
		}
	}
}

/// Blocks on a channel once and returns the slice of the buffer that was written by the system.
pub fn block_on_channel_request(
	system_id: ChannelSystemId,
	buffer: &mut [u8],
) -> Result<&mut [u8], ChannelError> {
	let status = block_on_channel(system_id.clone(), buffer)?;
	let written_len = status.size as usize;

	// SAFETY: the system guarantees that it wrote at most `buffer.len()` bytes
	// TODO: we can push this up higher and use a typesafe pattern to indicate the channgel status is safe.
	if written_len > buffer.len() {
		return Err(ChannelError::BufferTooSmall(status));
	}

	Ok(&mut buffer[..status.size as usize])
}

/// Blocks on a channel an runs a callback on the slice of the buffer
/// for every tick until processing is complete.
pub fn block_on_channel_stream_request(
	system_id: ChannelSystemId,
	buffer: &mut [u8],
	callback: impl Fn(&mut [u8]) -> Result<(), ChannelError>,
) -> Result<(), ChannelError> {
	let max_buffer_len = buffer.len();
	let mut status = write(system_id.clone(), buffer)?;
	let mut written_len = status.size as usize;

	loop {
		// SAFETY: the system guarantees that it wrote at most `buffer.len()` bytes
		// TODO: we can push this up higher and use a typesafe pattern to indicate the channgel status is safe.
		if written_len > max_buffer_len {
			return Err(ChannelError::BufferTooSmall(status));
		}

		if !status.is_holding() {
			callback(&mut buffer[..written_len])?;
		}

		if status.is_success() {
			return Ok(());
		}

		status = read(system_id.clone(), buffer)?;
		written_len = status.size as usize;
	}
}
