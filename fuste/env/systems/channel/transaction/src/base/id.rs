#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Id<const I: usize>([u8; I]);

impl<const I: usize> Id<I> {
	pub fn new(bytes: [u8; I]) -> Self {
		Self(bytes)
	}

	pub fn copy_from_slice(slice: &[u8]) -> Self {
		let mut bytes = [0; I];
		bytes.copy_from_slice(slice);
		Self(bytes)
	}
}
