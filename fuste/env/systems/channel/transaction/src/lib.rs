#![no_std]

pub mod base;
pub mod request;
pub mod response;

use fuste_channel::ChannelSystemId;
use fuste_serial_channel::{
	serial_channel_request, Deserialize, SerialChannelError, SerialType, Serialize,
};
use request::TransactionDataRequest;
use response::TransactionDataResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionSchemeId((u32, u32));

impl TransactionSchemeId {
	pub fn to_bytes(&self) -> [u8; 8] {
		let (first, second) = self.0;
		let mut bytes = [0; 8];
		bytes[..4].copy_from_slice(&first.to_le_bytes());
		bytes[4..].copy_from_slice(&second.to_le_bytes());
		bytes
	}

	pub fn from_bytes(bytes: &[u8]) -> Result<Self, SerialChannelError> {
		if bytes.len() != 8 {
			return Err(SerialChannelError::SerializedBufferTooSmall(8));
		}
		let first = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
		let second = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
		Ok(Self((first, second)))
	}

	pub fn into_inner(self) -> (u32, u32) {
		self.0
	}
}

pub trait TransactionScheme: SerialType {
	fn scheme_id() -> TransactionSchemeId;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionData<R: TransactionScheme> {
	pub scheme_id: TransactionSchemeId,
	pub data: R,
}

impl<R: TransactionScheme> TransactionData<R> {
	pub fn new(data: R) -> Self {
		Self { scheme_id: R::scheme_id(), data }
	}
}

impl<R: TransactionScheme> Serialize for TransactionData<R> {
	fn try_to_bytes<const N: usize>(&self) -> Result<(usize, [u8; N]), SerialChannelError> {
		let scheme_id_bytes = self.scheme_id.to_bytes();
		let (data_len, data_bytes) = self.data.try_to_bytes::<N>()?;

		let mut bytes = [0; N];

		if scheme_id_bytes.len() + scheme_id_bytes.len() + data_len > N {
			return Err(SerialChannelError::SerializedBufferTooSmall(
				(scheme_id_bytes.len() + scheme_id_bytes.len() + data_len) as u32,
			));
		}

		// Copy the scheme id and request bytes into the buffer.
		bytes[..scheme_id_bytes.len()].copy_from_slice(&scheme_id_bytes);
		bytes[scheme_id_bytes.len()..scheme_id_bytes.len() + data_len].copy_from_slice(&data_bytes);

		Ok((scheme_id_bytes.len() + data_len, bytes))
	}
}

impl<R: TransactionScheme> Deserialize for TransactionData<R> {
	fn try_from_bytes(bytes: &[u8]) -> Result<Self, SerialChannelError> {
		// First 4 bytes should be the scheme id.
		if bytes.len() < 4 {
			return Err(SerialChannelError::SerializedBufferTooSmall(4));
		}

		let scheme_id = TransactionSchemeId::from_bytes(&bytes[..4])?;
		let data = R::try_from_bytes(&bytes[4..])?;

		if scheme_id != R::scheme_id() {
			return Err(SerialChannelError::SchemeMismatch(
				scheme_id.into_inner(),
				R::scheme_id().into_inner(),
			));
		}

		Ok(Self { scheme_id, data })
	}
}

pub fn transaction_data<
	const N: usize,
	const M: usize,
	Signer,
	Id,
	Request: TransactionDataRequest<Signer, Id, Response>,
	Response: TransactionDataResponse<Signer, Id, Request>,
>(
	system_id: ChannelSystemId,
	request: Request,
) -> Result<Response, SerialChannelError> {
	let transaction_request_data = TransactionData::new(request);

	let response =
		serial_channel_request::<N, M, TransactionData<Request>, TransactionData<Response>>(
			system_id,
			&transaction_request_data,
		)?;

	Ok(response.data)
}
