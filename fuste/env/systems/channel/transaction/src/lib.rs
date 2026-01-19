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

impl Serialize for TransactionSchemeId {
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		if buffer.len() < self.0.len() {
			return Err(SerialChannelError::SerializedBufferTooSmall(self.0.len() as u32));
		}

		buffer[..self.0.len()].copy_from_slice(&self.0);
		Ok(self.0.len())
	}
}

impl Deserialize for TransactionSchemeId {
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		if buffer.len() != Self::ID_LENGTH {
			return Err(SerialChannelError::SerializedBufferTooSmall(Self::ID_LENGTH as u32));
		}
		let scheme_id = Self(buffer.try_into().unwrap());
		Ok((&buffer[Self::ID_LENGTH..], scheme_id))
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
	fn try_write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		let scheme_id_len = self.scheme_id.try_write_to_buffer(buffer)?;
		let data_len = self.data.try_write_to_buffer(&mut buffer[scheme_id_len..])?;
		Ok(scheme_id_len + data_len)
	}
}

impl<R: TransactionScheme> Deserialize for TransactionData<R> {
	fn try_from_bytes_with_remaining_buffer(
		bytes: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		let (remaining_buffer, scheme_id) =
			TransactionSchemeId::try_from_bytes_with_remaining_buffer(bytes)?;
		let (remaining_buffer, data) = R::try_from_bytes_with_remaining_buffer(remaining_buffer)?;

		Ok((remaining_buffer, Self { scheme_id, data }))
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
