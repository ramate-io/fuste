pub mod homo;

/// Simple fixed-size slab allocator with bitmap tracking
#[derive(Debug)]
pub enum SlabError {
	OutOfMemory,
	InvalidPointer,
}

pub struct Slab<'a, const BLOCK_SIZE: usize, const NUM_BLOCKS: usize> {
	memory: &'a mut [[u8; BLOCK_SIZE]; NUM_BLOCKS],
	bitmap: u32, // supports up to 32 blocks
}

impl<'a, const BLOCK_SIZE: usize, const NUM_BLOCKS: usize> Slab<'a, BLOCK_SIZE, NUM_BLOCKS> {
	/// Create a new constant slab
	pub const fn new_const(memory: &'a mut [[u8; BLOCK_SIZE]; NUM_BLOCKS]) -> Self {
		Self { memory, bitmap: 0 }
	}

	/// Create a new slab
	pub fn new(memory: &'a mut [[u8; BLOCK_SIZE]; NUM_BLOCKS]) -> Self {
		Self { memory, bitmap: 0 }
	}

	/// Allocate a block. Returns a mutable reference to the fixed-size block.
	pub fn alloc(&mut self) -> Result<&mut [u8; BLOCK_SIZE], SlabError> {
		if self.bitmap == (1 << NUM_BLOCKS) - 1 {
			return Err(SlabError::OutOfMemory);
		}

		// Find the first free block
		let free_index = (!self.bitmap).trailing_zeros() as usize;
		if free_index >= NUM_BLOCKS {
			return Err(SlabError::OutOfMemory);
		}

		// Mark block as used
		self.bitmap |= 1 << free_index;

		Ok(&mut self.memory[free_index])
	}

	/// Free a previously allocated block
	pub fn free(&mut self, ptr: *mut [u8; BLOCK_SIZE]) -> Result<(), SlabError> {
		let base = self.memory.as_ptr() as usize;
		let offset = ptr as usize - base;

		if offset % BLOCK_SIZE != 0 || offset >= BLOCK_SIZE * NUM_BLOCKS {
			return Err(SlabError::InvalidPointer);
		}

		let index = offset / BLOCK_SIZE;
		self.bitmap &= !(1 << index);
		Ok(())
	}

	/// Returns the number of free blocks remaining
	pub fn free_count(&self) -> usize {
		NUM_BLOCKS - self.bitmap.count_ones() as usize
	}

	/// Checks whether a block is currently allocated
	pub fn is_allocated(&self, index: usize) -> bool {
		if index >= NUM_BLOCKS {
			return false;
		}
		(self.bitmap & (1 << index)) != 0
	}
}
