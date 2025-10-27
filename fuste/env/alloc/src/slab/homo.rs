use super::{Slab, SlabError};

/// A cache of slabs which are all of the same dimension.
pub struct HomoSlabCache<
	'a,
	const BLOCK_SIZE: usize,
	const NUM_BLOCKS: usize,
	const NUM_SLABS: usize,
> {
	slabs: [Slab<'a, BLOCK_SIZE, NUM_BLOCKS>; NUM_SLABS],
}

impl<'a, const BLOCK_SIZE: usize, const NUM_BLOCKS: usize, const NUM_SLABS: usize>
	HomoSlabCache<'a, BLOCK_SIZE, NUM_BLOCKS, NUM_SLABS>
{
	/// Create a new constant HomoSlabCache
	pub const fn new_const(slabs: [Slab<'a, BLOCK_SIZE, NUM_BLOCKS>; NUM_SLABS]) -> Self {
		Self { slabs }
	}

	/// Construct a new HomoSlabCache from an array of slabs
	pub fn new(slabs: [Slab<'a, BLOCK_SIZE, NUM_BLOCKS>; NUM_SLABS]) -> Self {
		Self { slabs }
	}

	/// Allocate a block from the first slab with free space
	pub fn alloc(&mut self) -> Result<&mut [u8; BLOCK_SIZE], SlabError> {
		for slab in &mut self.slabs {
			if let Ok(block) = slab.alloc() {
				return Ok(block);
			}
		}
		Err(SlabError::OutOfMemory)
	}

	/// Free a block by searching all slabs
	pub fn free(&mut self, ptr: *mut [u8; BLOCK_SIZE]) -> Result<(), SlabError> {
		for slab in &mut self.slabs {
			if slab.free(ptr).is_ok() {
				return Ok(());
			}
		}
		Err(SlabError::InvalidPointer)
	}

	/// Count total free blocks
	pub fn free_count(&self) -> usize {
		self.slabs.iter().map(|s| s.free_count()).sum()
	}
}
