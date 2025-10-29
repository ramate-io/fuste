pub mod gallocator;
pub mod notepad;

use crate::slab::{homo::HomoSlabCache, SlabError};
use core::alloc::Layout;
use core::ptr::NonNull;

pub trait AllocatorLayout {
	// 32 byte blocks and slabs
	const NUM_32_BYTE_BLOCKS: usize;
	const NUM_32_BYTE_SLABS: usize;
	// 64 byte blocks and slabs
	const NUM_64_BYTE_BLOCKS: usize;
	const NUM_64_BYTE_SLABS: usize;
	// 128 byte blocks and slabs
	const NUM_128_BYTE_BLOCKS: usize;
	const NUM_128_BYTE_SLABS: usize;
	// 256 byte blocks and slabs
	const NUM_256_BYTE_BLOCKS: usize;
	const NUM_256_BYTE_SLABS: usize;
	// 512 byte blocks and slabs
	const NUM_512_BYTE_BLOCKS: usize;
	const NUM_512_BYTE_SLABS: usize;
	// 1024 byte blocks and slabs
	const NUM_1024_BYTE_BLOCKS: usize;
	const NUM_1024_BYTE_SLABS: usize;
	// 2048 byte blocks and slabs
	const NUM_2048_BYTE_BLOCKS: usize;
	const NUM_2048_BYTE_SLABS: usize;
	// 4096 byte blocks and slabs
	const NUM_4096_BYTE_BLOCKS: usize;
	const NUM_4096_BYTE_SLABS: usize;
	// 8192 byte blocks and slabs
	const NUM_8192_BYTE_BLOCKS: usize;
	const NUM_8192_BYTE_SLABS: usize;
}

/// Small helper result type used below
type AllocPtr = *mut u8;

/// The concrete Allocator holding multiple homogeneous slab caches.
/// Generic parameters mirror the number of blocks and slabs per class.
pub struct Allocator<
	'a,
	// 32 byte blocks and slabs
	const NUM_32_BYTE_BLOCKS: usize,
	const NUM_32_BYTE_SLABS: usize,
	// 64 byte blocks and slabs
	const NUM_64_BYTE_BLOCKS: usize,
	const NUM_64_BYTE_SLABS: usize,
	// 128 byte blocks and slabs
	const NUM_128_BYTE_BLOCKS: usize,
	const NUM_128_BYTE_SLABS: usize,
	// 256 byte blocks and slabs
	const NUM_256_BYTE_BLOCKS: usize,
	const NUM_256_BYTE_SLABS: usize,
	// 512 byte blocks and slabs
	const NUM_512_BYTE_BLOCKS: usize,
	const NUM_512_BYTE_SLABS: usize,
	// 1024 byte blocks and slabs
	const NUM_1024_BYTE_BLOCKS: usize,
	const NUM_1024_BYTE_SLABS: usize,
	// 2048 byte blocks and slabs
	const NUM_2048_BYTE_BLOCKS: usize,
	const NUM_2048_BYTE_SLABS: usize,
	// 4096 byte blocks and slabs
	const NUM_4096_BYTE_BLOCKS: usize,
	const NUM_4096_BYTE_SLABS: usize,
	// 8192 byte blocks and slabs
	const NUM_8192_BYTE_BLOCKS: usize,
	const NUM_8192_BYTE_SLABS: usize,
> {
	pub slab32: HomoSlabCache<'a, 32, NUM_32_BYTE_BLOCKS, NUM_32_BYTE_SLABS>,
	pub slab64: HomoSlabCache<'a, 64, NUM_64_BYTE_BLOCKS, NUM_64_BYTE_SLABS>,
	pub slab128: HomoSlabCache<'a, 128, NUM_128_BYTE_BLOCKS, NUM_128_BYTE_SLABS>,
	pub slab256: HomoSlabCache<'a, 256, NUM_256_BYTE_BLOCKS, NUM_256_BYTE_SLABS>,
	pub slab512: HomoSlabCache<'a, 512, NUM_512_BYTE_BLOCKS, NUM_512_BYTE_SLABS>,
	pub slab1024: HomoSlabCache<'a, 1024, NUM_1024_BYTE_BLOCKS, NUM_1024_BYTE_SLABS>,
	pub slab2048: HomoSlabCache<'a, 2048, NUM_2048_BYTE_BLOCKS, NUM_2048_BYTE_SLABS>,
	pub slab4096: HomoSlabCache<'a, 4096, NUM_4096_BYTE_BLOCKS, NUM_4096_BYTE_SLABS>,
	pub slab8192: HomoSlabCache<'a, 8192, NUM_8192_BYTE_BLOCKS, NUM_8192_BYTE_SLABS>,
}

impl<
		'a,
		const NUM_32_BYTE_BLOCKS: usize,
		const NUM_32_BYTE_SLABS: usize,
		const NUM_64_BYTE_BLOCKS: usize,
		const NUM_64_BYTE_SLABS: usize,
		const NUM_128_BYTE_BLOCKS: usize,
		const NUM_128_BYTE_SLABS: usize,
		const NUM_256_BYTE_BLOCKS: usize,
		const NUM_256_BYTE_SLABS: usize,
		const NUM_512_BYTE_BLOCKS: usize,
		const NUM_512_BYTE_SLABS: usize,
		const NUM_1024_BYTE_BLOCKS: usize,
		const NUM_1024_BYTE_SLABS: usize,
		const NUM_2048_BYTE_BLOCKS: usize,
		const NUM_2048_BYTE_SLABS: usize,
		const NUM_4096_BYTE_BLOCKS: usize,
		const NUM_4096_BYTE_SLABS: usize,
		const NUM_8192_BYTE_BLOCKS: usize,
		const NUM_8192_BYTE_SLABS: usize,
	>
	Allocator<
		'a,
		NUM_32_BYTE_BLOCKS,
		NUM_32_BYTE_SLABS,
		NUM_64_BYTE_BLOCKS,
		NUM_64_BYTE_SLABS,
		NUM_128_BYTE_BLOCKS,
		NUM_128_BYTE_SLABS,
		NUM_256_BYTE_BLOCKS,
		NUM_256_BYTE_SLABS,
		NUM_512_BYTE_BLOCKS,
		NUM_512_BYTE_SLABS,
		NUM_1024_BYTE_BLOCKS,
		NUM_1024_BYTE_SLABS,
		NUM_2048_BYTE_BLOCKS,
		NUM_2048_BYTE_SLABS,
		NUM_4096_BYTE_BLOCKS,
		NUM_4096_BYTE_SLABS,
		NUM_8192_BYTE_BLOCKS,
		NUM_8192_BYTE_SLABS,
	>
{
	/// Allocate a block for the given layout. Returns pointer or Err.
	pub fn alloc_layout(&mut self, layout: Layout) -> Result<AllocPtr, SlabError> {
		// handle zero-sized
		if layout.size() == 0 {
			return Ok(NonNull::<u8>::dangling().as_ptr());
		}

		// Try classes in ascending order of block size. Must satisfy size and alignment.
		let size = layout.size();
		let align = layout.align();

		// helper macro to test and try allocation
		macro_rules! try_class {
			($cache:expr, $block_size:expr) => {
				if $block_size >= size && $block_size >= align {
					if let Ok(block) = $cache.alloc() {
						return Ok(block.as_mut_ptr() as AllocPtr);
					}
				}
			};
		}

		try_class!(self.slab32, 32);
		try_class!(self.slab128, 128);
		try_class!(self.slab256, 256);
		try_class!(self.slab512, 512);
		try_class!(self.slab1024, 1024);
		try_class!(self.slab2048, 2048);
		try_class!(self.slab4096, 4096);
		try_class!(self.slab8192, 8192);

		// Nothing fit
		Err(SlabError::OutOfMemory)
	}

	/// Free a pointer by searching slab caches for a match.
	/// The provided layout is ignored for ownership detection; we search caches.
	pub fn free_ptr(&mut self, ptr: AllocPtr) -> Result<(), SlabError> {
		if ptr.is_null() {
			return Ok(());
		}

		// Try each cache in same order. Cast pointer to the appropriate array pointer.
		macro_rules! try_free_class {
			($cache:expr, $block_size:expr) => {
				// Cast pointer to pointer-to-array; slab.free will validate ownership.
				let arr_ptr = ptr as *mut [u8; $block_size];
				if $cache.free(arr_ptr).is_ok() {
					return Ok(());
				}
			};
		}

		try_free_class!(self.slab32, 32);
		try_free_class!(self.slab128, 128);
		try_free_class!(self.slab256, 256);
		try_free_class!(self.slab512, 512);
		try_free_class!(self.slab1024, 1024);
		try_free_class!(self.slab2048, 2048);
		try_free_class!(self.slab4096, 4096);
		try_free_class!(self.slab8192, 8192);

		Err(SlabError::InvalidPointer)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::slab::Slab;

	#[test]
	fn simple_alloc_free() {
		static mut MEMORY_32: [[[u8; 32]; 4]; 1] = [[[0; 32]; 4]; 1];
		let slabs32_arr = [Slab::new_const(unsafe { &mut MEMORY_32[0] })];

		let slabs128_arr: [Slab<'static, 128, 4>; 0] = [];
		let slabs64_arr: [Slab<'static, 64, 4>; 0] = [];
		let slabs256_arr: [Slab<'static, 256, 4>; 0] = [];
		let slabs512_arr: [Slab<'static, 512, 4>; 0] = [];
		let slabs1024_arr: [Slab<'static, 1024, 4>; 0] = [];
		let slabs2048_arr: [Slab<'static, 2048, 4>; 0] = [];
		let slabs4096_arr: [Slab<'static, 4096, 4>; 0] = [];
		let slabs8192_arr: [Slab<'static, 8192, 4>; 0] = [];

		let mut allocator = Allocator {
			slab32: HomoSlabCache::new_const(slabs32_arr),
			slab64: HomoSlabCache::new_const(slabs64_arr),
			slab128: HomoSlabCache::new_const(slabs128_arr),
			slab256: HomoSlabCache::new_const(slabs256_arr),
			slab512: HomoSlabCache::new_const(slabs512_arr),
			slab1024: HomoSlabCache::new_const(slabs1024_arr),
			slab2048: HomoSlabCache::new_const(slabs2048_arr),
			slab4096: HomoSlabCache::new_const(slabs4096_arr),
			slab8192: HomoSlabCache::new_const(slabs8192_arr),
		};

		let layout = Layout::from_size_align(16, 8).unwrap();
		let ptr = allocator.alloc_layout(layout).unwrap();
		unsafe { *ptr = 42 };

		allocator.free_ptr(ptr).unwrap();
		assert_eq!(allocator.slab32.free_count(), 4);
	}
}
