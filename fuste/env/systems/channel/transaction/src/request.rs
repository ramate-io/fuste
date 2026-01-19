use crate::{response::TransactionDataResponse, TransactionScheme};

/// A marker trait for transaction data requests.
pub trait TransactionDataRequest<Signer, Id, R: TransactionDataResponse<Signer, Id, Self>>:
	TransactionScheme
{
}
