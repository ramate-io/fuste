#![no_std]

use fuste_serial_channel::{Deserialize, SerialChannelError, SerialType, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionSchemeId(u32);

pub trait TransactionScheme: SerialType {
	fn scheme_id() -> TransactionSchemeId;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionMetadataRequest<R: TransactionScheme> {
	pub scheme_id: TransactionSchemeId,
	pub request: R,
}

impl<R: TransactionScheme> TransactionMetadataRequest<R> {
	pub fn new(request: R) -> Self {
		Self { scheme_id: R::scheme_id(), request }
	}
}

impl<R: TransactionScheme> Serialize for TransactionMetadataRequest<R> {
	fn try_to_bytes<const N: usize>(&self) -> Result<(usize, [u8; N]), SerialChannelError> {
		let scheme_id_bytes: [u8; 4] = self.scheme_id.0.to_le_bytes();
		let (request_len, request_bytes) = self.request.try_to_bytes::<N>()?;

		let mut bytes = [0; N];

		if scheme_id_bytes.len() + request_len > N {
			return Err(SerialChannelError::SerializedBufferTooSmall(
				(scheme_id_bytes.len() + request_len) as u32,
			));
		}

		// Copy the scheme id and request bytes into the buffer.
		bytes[..scheme_id_bytes.len()].copy_from_slice(&scheme_id_bytes);
		bytes[scheme_id_bytes.len()..scheme_id_bytes.len() + request_len]
			.copy_from_slice(&request_bytes);

		Ok((scheme_id_bytes.len() + request_len, bytes))
	}
}

impl<R: TransactionScheme> Deserialize for TransactionMetadataRequest<R> {
	fn try_from_bytes(bytes: &[u8]) -> Result<Self, SerialChannelError> {
		// First 4 bytes should be the scheme id.
		if bytes.len() < 4 {
			return Err(SerialChannelError::SerializedBufferTooSmall(4));
		}

		let scheme_id =
			TransactionSchemeId(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));
		let request = R::try_from_bytes(&bytes[4..])?;

		if scheme_id != R::scheme_id() {
			return Err(SerialChannelError::TypeMismatch((scheme_id.0, R::scheme_id().0)));
		}

		Ok(Self { scheme_id, request })
	}
}
