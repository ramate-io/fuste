pub mod signer;

use crate::{TransactionScheme, TransactionSchemeId};
use fuste_serial_channel::{Deserialize, SerialChannelError, Serialize};

/// The base transaction scheme that should be installed with any transaction-based system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base<const N: usize, const P: usize>;

impl<const N: usize, const P: usize> TransactionScheme for Base<N, P> {
	fn scheme_id() -> TransactionSchemeId {
		TransactionSchemeId((N as u32, P as u32))
	}
}

impl<const N: usize, const P: usize> Serialize for Base<N, P> {
	fn try_to_bytes<const M: usize>(&self) -> Result<(usize, [u8; M]), SerialChannelError> {
		Ok((0, [0; M]))
	}
}

impl<const N: usize, const P: usize> Deserialize for Base<N, P> {
	fn try_from_bytes(_bytes: &[u8]) -> Result<Self, SerialChannelError> {
		Ok(Base)
	}
}
