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
	fn try_to_bytes<const M: usize>(&self) -> Result<(usize, [u8; M]), SerialChannelError> {
		Ok((0, [0; M]))
	}
}

impl<Signer, Id, Response: TransactionDataResponse<Signer, Id, Self>> Deserialize
	for Base<Signer, Id, Response>
{
	fn try_from_bytes(_bytes: &[u8]) -> Result<Self, SerialChannelError> {
		Ok(Base::new())
	}
}

impl<Signer, Id, Response: TransactionDataResponse<Signer, Id, Self>>
	TransactionDataRequest<Signer, Id, Response> for Base<Signer, Id, Response>
{
}

impl<const N: usize, const P: usize, const K: usize, const I: usize>
	Base<BaseSigner<N, P>, Id<I>, BaseTransaction<N, P, K, I>>
{
	pub fn base() -> Self {
		Base::new()
	}

	pub fn get() -> Result<BaseTransaction<N, P, K, I>, SerialChannelError> {
		transaction_data::<
			{ TransactionSchemeId::ID_LENGTH },
			N,
			BaseSigner<N, P>,
			Id<I>,
			Self,
			BaseTransaction<N, P, K, I>,
		>(Self::base())
	}
}

pub fn get_base_transaction<const N: usize, const P: usize, const K: usize, const I: usize>(
) -> Result<BaseTransaction<N, P, K, I>, SerialChannelError> {
	Base::<BaseSigner<N, P>, Id<I>, BaseTransaction<N, P, K, I>>::get()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_base_get_compiles() {
		let _base = Base::<BaseSigner<32, 32>, Id<32>, BaseTransaction<32, 32, 32, 32>>::get();
		let _base = get_base_transaction::<32, 32, 32, 32>();
	}
}
