use fuste_serial_channel::{Deserialize, SerialChannelError, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemBufferAddress(u32);

impl SystemBufferAddress {
	/// Canonically, the system_buffer_address 0 will refer
	/// to the signer associated with the hart itself.
	///
	/// This is commonly used when managing authentication when logic within the transaction
	/// should decide internal state, for example, updates to a transaction module.
	pub const HART_SELF: Self = Self(0);

	pub fn to_le_bytes(&self) -> [u8; 4] {
		self.0.to_le_bytes()
	}
}

/// The base signer type that should be used for any transaction-based system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BaseSigner<const N: usize, const P: usize> {
	address_bytes: [u8; N],
	public_key_bytes: [u8; P],
	// The system index in the system buffer where the signer is stored.
	// This is used for validating the signer in the context of a given hart.
	system_buffer_address: SystemBufferAddress,
}

impl<const N: usize, const P: usize> BaseSigner<N, P> {
	/// Create a new signer.
	///
	/// Note that it is fine to expose this, because security is enforced by the systems themselves.
	/// In the end, the developer can program whatever they want in the ELF binary,
	/// and it is the job of the VM and systems to isolate it.
	/// However, developers will not want to accidentally use this, so it is guarded by a feature.
	#[cfg(feature = "system")]
	pub fn new(
		address_bytes: [u8; N],
		public_key_bytes: [u8; P],
		system_buffer_address: SystemBufferAddress,
	) -> Self {
		Self { address_bytes, public_key_bytes, system_buffer_address }
	}

	/// Creates a signer with the canonical hart self address,
	/// but not necessarily with the right public key and address bytes.
	#[cfg(feature = "system")]
	pub fn partial_hart_self() -> Self {
		Self {
			address_bytes: [0; N],
			public_key_bytes: [0; P],
			system_buffer_address: SystemBufferAddress::HART_SELF,
		}
	}

	#[cfg(feature = "system")]
	pub fn with_system_buffer_address(
		mut self,
		system_buffer_address: SystemBufferAddress,
	) -> Self {
		self.system_buffer_address = system_buffer_address;
		self
	}

	#[cfg(feature = "system")]
	pub fn with_address_bytes(mut self, address_bytes: [u8; N]) -> Self {
		self.address_bytes = address_bytes;
		self
	}

	#[cfg(feature = "system")]
	pub fn with_public_key_bytes(mut self, public_key_bytes: [u8; P]) -> Self {
		self.public_key_bytes = public_key_bytes;
		self
	}
}

impl<const N: usize, const P: usize> Serialize for BaseSigner<N, P> {
	fn try_to_bytes<const M: usize>(&self) -> Result<(usize, [u8; M]), SerialChannelError> {
		let mut bytes = [0; M];
		bytes[..4].copy_from_slice(&self.system_buffer_address.to_le_bytes());
		bytes[4..N + 4].copy_from_slice(&self.address_bytes);
		bytes[N + 4..N + P + 4].copy_from_slice(&self.public_key_bytes);
		Ok((N + P + 4, bytes))
	}
}

impl<const N: usize, const P: usize> Deserialize for BaseSigner<N, P> {
	fn try_from_bytes(bytes: &[u8]) -> Result<Self, SerialChannelError> {
		// First 4 bytes should be the system buffer address.
		if bytes.len() < 4 {
			return Err(SerialChannelError::SerializedBufferTooSmall(4));
		}
		let system_buffer_address = u32::from_le_bytes(bytes[..4].try_into().unwrap());

		// The next N bytes should be the address bytes.
		if bytes.len() < N {
			return Err(SerialChannelError::SerializedBufferTooSmall(N as u32));
		}
		let mut copy_address_buffer = [0; N];
		copy_address_buffer.copy_from_slice(&bytes[4..N + 4]);
		let address_bytes = copy_address_buffer;

		// Next P bytes should be the public key bytes.
		if bytes.len() < N + P + 4 {
			return Err(SerialChannelError::SerializedBufferTooSmall(N as u32 + P as u32 + 4));
		}
		let mut copy_public_key_buffer = [0; P];
		copy_public_key_buffer.copy_from_slice(&bytes[N + 4..N + P + 4]);
		let public_key_bytes = copy_public_key_buffer;

		Ok(BaseSigner {
			address_bytes,
			public_key_bytes,
			system_buffer_address: SystemBufferAddress(system_buffer_address),
		})
	}
}
