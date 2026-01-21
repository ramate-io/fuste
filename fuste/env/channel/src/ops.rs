use super::{check, open, ChannelError, ChannelStatus, ChannelSystemId};

/// Writes to a channel once and blocks until the operation is complete.
pub fn block_on_channel(
	system_id: ChannelSystemId,
	read_buffer: &[u8],
	read_write_buffer: &mut [u8],
) -> Result<ChannelStatus, ChannelError> {
	let status = open(system_id.clone(), read_buffer, read_write_buffer)?;

	// Success means completion
	if status.is_success() {
		Ok(status.clone())
	} else {
		loop {
			let status = check(system_id.clone(), read_buffer, read_write_buffer)?;
			if status.is_success() {
				return Ok(status);
			}
		}
	}
}

/// Blocks on a channel once and returns the slice of the buffer that was written by the system.
pub fn block_on_channel_request<'r, 'w>(
	system_id: ChannelSystemId,
	read_buffer: &'r [u8],
	read_write_buffer: &'w mut [u8],
) -> Result<&'w mut [u8], ChannelError> {
	let status = block_on_channel(system_id.clone(), read_buffer, read_write_buffer)?;
	let written_len = status.size as usize;

	// SAFETY: the system guarantees that it wrote at most `buffer.len()` bytes
	// TODO: we can push this up higher and use a typesafe pattern to indicate the channgel status is safe.
	if written_len > read_write_buffer.len() {
		return Err(ChannelError::BufferTooSmall(status));
	}

	Ok(&mut read_write_buffer[..status.size as usize])
}

/// Blocks on a channel an runs a callback on the slice of the buffer
/// for every tick until processing is complete.
pub fn block_on_channel_stream_request<'r, 'w>(
	system_id: ChannelSystemId,
	read_buffer: &'r [u8],
	read_write_buffer: &'w mut [u8],
	callback: impl Fn(&mut [u8]) -> Result<(), ChannelError>,
) -> Result<(), ChannelError> {
	let max_buffer_len = read_write_buffer.len();
	let mut status = open(system_id.clone(), read_buffer, read_write_buffer)?;
	let mut written_len = status.size as usize;

	loop {
		// SAFETY: the system guarantees that it wrote at most `buffer.len()` bytes
		// TODO: we can push this up higher and use a typesafe pattern to indicate the channgel status is safe.
		if written_len > max_buffer_len {
			return Err(ChannelError::BufferTooSmall(status));
		}

		if !status.is_holding() {
			callback(&mut read_write_buffer[..written_len])?;
		}

		// Success should always mean a meaningless write buffer.
		// The last holding operations should always have the last meaningful write buffer.
		if status.is_success() {
			return Ok(());
		}

		// The check tells the system to turn over the next operation
		status = check(system_id.clone(), read_buffer, read_write_buffer)?;
		written_len = status.size as usize;
	}
}
