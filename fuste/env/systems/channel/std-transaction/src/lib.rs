#![no_std]

pub mod all;
pub mod id;
pub mod signer;

use all::All;
use fuste_channel::ChannelSystemId;
use fuste_serial_channel::{serial_channel_request, SerialChannelError, SerialType};

pub trait TransactionScheme: SerialType {
	const CHANNEL_SYSTEM_ID: ChannelSystemId;
}
pub trait TransactionData: SerialType {}

/// Gets the transaction data for the given request and response.
///
/// # Arguments
///
/// * `request` - The request to get the transaction data for.
/// * `response` - The response to get the transaction data for.
///
/// # Returns
///
/// The transaction data for the given request and response.
pub fn transaction_data<
	const RSIZE: usize,
	const WSIZE: usize,
	Scheme: TransactionScheme,
	Data: TransactionData,
>(
	scheme: Scheme,
) -> Result<Data, SerialChannelError> {
	serial_channel_request::<RSIZE, WSIZE, Scheme, Data>(Scheme::CHANNEL_SYSTEM_ID, &scheme)
}

pub fn transaction_data_all<
	const RSIZE: usize,
	const WSIZE: usize,
	Scheme: TransactionScheme + All,
	Data: TransactionData,
>() -> impl Iterator<Item = Result<Data, SerialChannelError>> {
	Scheme::all().map(|scheme| transaction_data::<RSIZE, WSIZE, Scheme, Data>(scheme))
}

pub fn transaction_data_all_from<
	const RSIZE: usize,
	const WSIZE: usize,
	Scheme: TransactionScheme + All + Sized,
	Data: TransactionData,
>(
	scheme: &Scheme,
) -> impl Iterator<Item = Result<Data, SerialChannelError>> + '_ {
	scheme
		.all_from()
		.map(|scheme| transaction_data::<RSIZE, WSIZE, Scheme, Data>(scheme))
}
