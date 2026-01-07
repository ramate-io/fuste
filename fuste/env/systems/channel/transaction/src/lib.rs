#![no_std]

use fuste_channel::{ops::block_on_channel, ChannelSystemId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfRequestChannelId(u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SfRequestError {
	CouldNotDeserialize(u32),
}

pub trait ChannelRequestBytes<const N: usize>: Sized {
	/// Tries to fill a buffer of the max bytes size.
	fn try_to_bytes(&self) -> Result<[u8; N], SfRequestError>;

	/// Tries to deserialize a buffer into a request.
	fn try_from_bytes(bytes: &[u8]) -> Result<Self, SfRequestError>;
}
