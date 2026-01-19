pub mod id;
pub mod response;
pub mod signer;

use crate::transaction_data;
use crate::{
	request::TransactionDataRequest, response::TransactionDataResponse, TransactionScheme,
	TransactionSchemeId,
};
use core::marker::PhantomData;
use fuste_serial_channel::{Deserialize, SerialChannelError, Serialize};
use id::Id;
use response::BaseTransaction;
use signer::BaseSigner;
use signer::SystemBufferAddress;

/// The base transaction scheme that should be installed with any transaction-based system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base<Signer, Id, Response: TransactionDataResponse<Signer, Id, Self>> {
	__signer_marker: PhantomData<Signer>,
	__id_marker: PhantomData<Id>,
	__response_marker: PhantomData<Response>,
}

impl<Signer, Id, Response: TransactionDataResponse<Signer, Id, Self>> Base<Signer, Id, Response> {
	pub fn new() -> Self {
		Self {
			__signer_marker: PhantomData,
			__id_marker: PhantomData,
			__response_marker: PhantomData,
		}
	}
}

impl<Signer, Id, Response: TransactionDataResponse<Signer, Id, Self>> TransactionScheme
	for Base<Signer, Id, Response>
{
	fn scheme_id() -> TransactionSchemeId {
		Response::scheme_id()
	}
}

impl<Signer, Id, Response: TransactionDataResponse<Signer, Id, Self>> Serialize
	for Base<Signer, Id, Response>
{
	fn try_write_to_buffer(&self, _buffer: &mut [u8]) -> Result<usize, SerialChannelError> {
		Ok(0)
	}
}

impl<Signer, Id, Response: TransactionDataResponse<Signer, Id, Self>> Deserialize
	for Base<Signer, Id, Response>
{
	fn try_from_bytes_with_remaining_buffer(
		buffer: &[u8],
	) -> Result<(&[u8], Self), SerialChannelError> {
		Ok((buffer, Base::new()))
	}
}

impl<Signer, Id, Response: TransactionDataResponse<Signer, Id, Self>>
	TransactionDataRequest<Signer, Id, Response> for Base<Signer, Id, Response>
{
}

pub struct Assert<const OK: bool>;
pub trait IsTrue {}
impl IsTrue for Assert<true> {}

impl<const N: usize, const P: usize, const K: usize, const I: usize>
	Base<BaseSigner<N, P>, Id<I>, BaseTransaction<N, P, K, I>>
{
	pub fn base() -> Self {
		Base::new()
	}

	pub fn get_with_wsize<const WSIZE: usize>(
	) -> Result<BaseTransaction<N, P, K, I>, SerialChannelError> {
		if WSIZE < I + K * (N + P + SystemBufferAddress::BYTES_LENGTH) {
			return Err(SerialChannelError::SerializedBufferTooSmall(
				(I + K * (N + P + SystemBufferAddress::BYTES_LENGTH)) as u32,
			));
		}

		// TODO: make this static somehow without using experimental features.
		transaction_data::<
			{ TransactionSchemeId::ID_LENGTH },
			WSIZE,
			BaseSigner<N, P>,
			Id<I>,
			Self,
			BaseTransaction<N, P, K, I>,
		>(Self::base())
	}

	/// Gets the base transaction with a canonically small write buffer size.
	pub fn get_small() -> Result<BaseTransaction<N, P, K, I>, SerialChannelError> {
		Self::get_with_wsize::<{ 1024 * 32 }>()
	}
}

pub fn transaction_with_wsize<
	const WSIZE: usize,
	const N: usize,
	const P: usize,
	const K: usize,
	const I: usize,
>() -> Result<BaseTransaction<N, P, K, I>, SerialChannelError> {
	Base::<BaseSigner<N, P>, Id<I>, BaseTransaction<N, P, K, I>>::get_with_wsize::<WSIZE>()
}

pub fn transaction_small<const N: usize, const P: usize, const K: usize, const I: usize>(
) -> Result<BaseTransaction<N, P, K, I>, SerialChannelError> {
	Base::<BaseSigner<N, P>, Id<I>, BaseTransaction<N, P, K, I>>::get_small()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_base_get_compiles() {
		let _base =
			Base::<BaseSigner<32, 32>, Id<32>, BaseTransaction<32, 32, 32, 32>>::get_with_wsize::<
				{ 1024 * 1024 },
			>();
		let _base = transaction_with_wsize::<{ 1024 * 1024 }, 32, 32, 32, 32>();
	}
}
