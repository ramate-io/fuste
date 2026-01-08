use crate::{response::TransactionDataResponse, TransactionScheme};

pub trait TransactionDataRequest<Signer, Id, R: TransactionDataResponse<Signer, Id, Self>>:
	TransactionScheme
{
}
