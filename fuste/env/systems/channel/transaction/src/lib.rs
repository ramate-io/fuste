#![no_std]

use core::marker::PhantomData;
use fuste_channel::ChannelSystemId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionMetadataError {
	CouldNotDeserialize(u32),
}

/// The scheme ID is a 32-bit value that identifies the scheme to use for the transaction.
///
/// This allows for supporting multiple transaction schemes written on the same channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionSchemeId(u32);

impl TransactionSchemeId {
	pub const fn constant(value: u32) -> Self {
		Self(value)
	}

	pub fn new(value: u32) -> Self {
		Self(value)
	}

	pub fn to_u32(self) -> u32 {
		self.0
	}

	pub const fn to_const_u32(self) -> u32 {
		self.0
	}
}

pub trait TransactionMetadata<Signer: Sized>: Sized {
	fn try_from_bytes(bytes: &[u8]) -> Result<Self, TransactionMetadataError>;

	fn signers(&self) -> &[Signer];

	fn scheme_id(&self) -> TransactionSchemeId;
}

pub struct Transaction<M: TransactionMetadata<Signer>, Signer: Sized> {
	metadata: M,
	__signer_marker: PhantomData<Signer>,
}

impl<M: TransactionMetadata<Signer>, Signer: Sized> Transaction<M, Signer> {
	pub const SYSTEM_ID: ChannelSystemId = ChannelSystemId::constant(32);

	pub const fn to_const_u32() -> u32 {
		Self::SYSTEM_ID.to_const_u32()
	}

	pub fn new(metadata: M) -> Self {
		Self { metadata, __signer_marker: PhantomData }
	}

	pub fn metadata(&self) -> &M {
		&self.metadata
	}

	pub fn signers(&self) -> &[Signer] {
		self.metadata.signers()
	}
}
