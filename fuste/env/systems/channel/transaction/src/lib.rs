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
pub struct TransactionSchemeId([u8; Self::ID_LENGTH]);

impl TransactionSchemeId {
	const ID_LENGTH: usize = 32;

	pub fn to_bytes(&self) -> [u8; Self::ID_LENGTH] {
		self.0
	}

	pub fn from_bytes(bytes: &[u8]) -> Result<Self, SerialChannelError> {
		if bytes.len() != 32 {
			return Err(SerialChannelError::SerializedBufferTooSmall(32));
		}
		let mut id = [0; 32];
		id.copy_from_slice(bytes);
		Ok(Self(id))
	}

	pub fn into_inner(self) -> [u8; Self::ID_LENGTH] {
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
	const CHANNEL_SYSTEM_ID: ChannelSystemId = ChannelSystemId::constant(0xad03);

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
	request: Request,
) -> Result<Response, SerialChannelError> {
	let transaction_request_data = TransactionData::new(request);

	let response =
		serial_channel_request::<N, M, TransactionData<Request>, TransactionData<Response>>(
			TransactionData::<Request>::CHANNEL_SYSTEM_ID,
			&transaction_request_data,
		)?;

	Ok(response.data)
}
