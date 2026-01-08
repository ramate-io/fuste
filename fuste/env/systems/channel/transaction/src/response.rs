use crate::{request::TransactionDataRequest, TransactionScheme};

pub trait TransactionDataResponse<Signer, Id, Request: TransactionDataRequest<Signer, Id, Self>>:
	TransactionScheme
{
	/// Returns the signers for the transaction up to a maximum of N signers.
	fn signers(&self) -> &[Option<Signer>];

	/// Returns the id for the transaction.
	fn id(&self) -> &Id;
}
