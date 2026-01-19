use core::ops::ControlFlow;
use fuste_ecall_dispatcher::{CheckChannelSystemDispatcher, OpenChannelSystemDispatcher};
use fuste_riscv_core::machine::{Machine, MachineError, MachineSystem};
use fuste_std_output::Stdout;
use fuste_transaction::base::{
	id::Id,
	response::BaseTransaction,
	signer::{BaseSigner, SystemBufferAddress},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserSignerRequest<const N: usize, const P: usize> {
	pub index: Option<usize>,
	pub address_bytes: Option<[u8; N]>,
	pub public_key_bytes: Option<[u8; P]>,
}

impl<const N: usize, const P: usize> UserSignerRequest<N, P> {
	pub fn address_bytes(&self) -> Option<[u8; N]> {
		// if there are address bytes, return them
		if let Some(address_bytes) = self.address_bytes {
			Some(address_bytes.clone())
		} else if let Some(public_key_bytes) = self.public_key_bytes {
			// if there are public key bytes, return them, cut to the address bytes length
			let mut address_bytes = [0; N];
			address_bytes.copy_from_slice(&public_key_bytes[..N]);
			Some(address_bytes)
		} else {
			None
		}
	}

	pub fn public_key_bytes(&self) -> Option<[u8; P]> {
		// if there are public key bytes, return them
		if let Some(public_key_bytes) = self.public_key_bytes {
			Some(public_key_bytes.clone())
		} else if let Some(address_bytes) = self.address_bytes {
			// if there are address bytes, return them, cut to the public key bytes length
			let mut public_key_bytes = [0; P];
			public_key_bytes.copy_from_slice(&address_bytes[..P]);
			Some(public_key_bytes)
		} else {
			None
		}
	}

	pub fn user_signer_bytes(&self) -> Option<([u8; N], [u8; P])> {
		if let Some(address_bytes) = self.address_bytes() {
			if let Some(public_key_bytes) = self.public_key_bytes() {
				Some((address_bytes, public_key_bytes))
			} else {
				None
			}
		} else {
			None
		}
	}
}

pub struct TransactionMetadataSystemBuilder<
	const MEMORY_SIZE: usize,
	const N: usize,
	const P: usize,
	const K: usize,
	const I: usize,
> {
	id: Id<I>,
	hart: ([u8; N], [u8; P]),
	user_signers: Vec<Option<([u8; N], [u8; P])>>,
}

impl<const MEMORY_SIZE: usize, const N: usize, const P: usize, const K: usize, const I: usize>
	Default for TransactionMetadataSystemBuilder<MEMORY_SIZE, N, P, K, I>
{
	fn default() -> Self {
		Self { id: Self::DEFAULT_ID, hart: Self::DEFAULT_HART, user_signers: Vec::new() }
	}
}

impl<const MEMORY_SIZE: usize, const N: usize, const P: usize, const K: usize, const I: usize>
	TransactionMetadataSystemBuilder<MEMORY_SIZE, N, P, K, I>
{
	const DEFAULT_SIGNER: Option<BaseSigner<N, P>> = None;
	const DEFAULT_ID: Id<I> = Id::const_new();
	const DEFAULT_HART: ([u8; N], [u8; P]) = ([0; N], [0; P]);

	pub fn with_id(mut self, id: Id<I>) -> Self {
		self.id = id;
		self
	}

	pub fn with_hart(mut self, hart: ([u8; N], [u8; P])) -> Self {
		self.hart = hart;
		self
	}

	pub fn with_user_signer(mut self, user_signer: UserSignerRequest<N, P>) -> Self {
		let user_signer_bytes = user_signer.user_signer_bytes();
		if let Some(index) = user_signer.index {
			self.user_signers[index] = user_signer_bytes;
		} else {
			self.user_signers.push(user_signer_bytes);
		}
		self
	}

	pub fn build(self) -> TransactionMetadataSystem<MEMORY_SIZE, N, P, K, I> {
		let mut signers_buf: [Option<BaseSigner<N, P>>; K] = [Self::DEFAULT_SIGNER; K];

		let hart_signer = BaseSigner::new(self.hart.0, self.hart.1, SystemBufferAddress::HART_SELF);
		signers_buf[0] = Some(hart_signer);

		for (i, signer) in self.user_signers.iter().enumerate() {
			if let Some((address_bytes, public_key_bytes)) = signer {
				signers_buf[i + 1] = Some(BaseSigner::new(
					*address_bytes,
					*public_key_bytes,
					SystemBufferAddress::new((i + 1) as u32),
				));
			}
		}

		TransactionMetadataSystem { response_data: BaseTransaction::new(signers_buf, self.id) }
	}
}

pub struct TransactionMetadataSystem<
	const MEMORY_SIZE: usize,
	const N: usize,
	const P: usize,
	const K: usize,
	const I: usize,
> {
	response_data: BaseTransaction<N, P, K, I>,
}

impl<const MEMORY_SIZE: usize, const N: usize, const P: usize, const K: usize, const I: usize>
	TransactionMetadataSystem<MEMORY_SIZE, N, P, K, I>
{
	pub fn response_data(&self) -> &BaseTransaction<N, P, K, I> {
		&self.response_data
	}
}

impl<const MEMORY_SIZE: usize, const N: usize, const P: usize, const K: usize, const I: usize>
	MachineSystem<MEMORY_SIZE> for TransactionMetadataSystem<MEMORY_SIZE, N, P, K, I>
{
	fn tick(
		&mut self,
		machine: &mut Machine<MEMORY_SIZE>,
	) -> Result<ControlFlow<()>, MachineError> {
		let write_fd = machine.csrs().registers().get(10);
		let write_buffer_address = machine.csrs().registers().get(11);
		let write_buffer_length = machine.csrs().registers().get(12);
		let write_buffer = machine
			.memory()
			.read_bytes(write_buffer_address, write_buffer_length)
			.map_err(MachineError::MemoryError)?;

		if write_fd == Stdout::to_const_u32() {
			// print the write buffer to stdout
			print!("{}", String::from_utf8_lossy(write_buffer));
			// write 0 to the result register a3
			machine.csrs_mut().registers_mut().set(13, 0);
			machine.csrs_mut().registers_mut().program_counter_mut().increment();
			machine.commit_csrs();
		}

		Ok(ControlFlow::Continue(()))
	}
}

impl<const MEMORY_SIZE: usize, const N: usize, const P: usize, const K: usize, const I: usize>
	CheckChannelSystemDispatcher<MEMORY_SIZE> for TransactionMetadataSystem<MEMORY_SIZE, N, P, K, I>
{
}

impl<const MEMORY_SIZE: usize, const N: usize, const P: usize, const K: usize, const I: usize>
	OpenChannelSystemDispatcher<MEMORY_SIZE> for TransactionMetadataSystem<MEMORY_SIZE, N, P, K, I>
{
}
