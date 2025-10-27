use crate::slab::{homo::HomoSlabCache, Slab, SlabError};
use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::ptr::NonNull;

/// Small helper result type used below
type AllocPtr = *mut u8;

/// The concrete Allocator holding multiple homogeneous slab caches.
/// Generic parameters mirror the number of blocks and slabs per class.
pub struct Allocator<'a, const NUM_32_BYTE_BLOCKS: usize, const NUM_32_BYTE_SLABS: usize> {
	slab32: HomoSlabCache<'a, 32, NUM_32_BYTE_BLOCKS, 1>,
}

impl<'a, const NUM_32_BYTE_BLOCKS: usize, const NUM_32_BYTE_SLABS: usize>
	Allocator<'a, NUM_32_BYTE_BLOCKS, NUM_32_BYTE_SLABS>
{
	/// Build an allocator from backing memory arrays.
	///
	/// Each memory argument is a 3D array: [NUM_SLABS][NUM_BLOCKS][BLOCK_SIZE].
	pub const fn new_from_memory(
		memory_32: &'a mut [[[u8; 32]; NUM_32_BYTE_BLOCKS]; NUM_32_BYTE_SLABS],
	) -> Self {
		// Use array_init to construct slab arrays for each class
		let slabs32_arr = [Slab::new_const(&mut memory_32[0])];
		let slab32 = HomoSlabCache::new_const(slabs32_arr);

		Self { slab32 }
	}

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

		Err(SlabError::InvalidPointer)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const BLOCKS: usize = 4;
	const SLABS: usize = 1;

	#[test]
	fn simple_alloc_free() {
		static mut MEMORY: [[[u8; 32]; BLOCKS]; SLABS] = [[[0; 32]; BLOCKS]; SLABS];

		let mut allocator = unsafe {
			#[allow(static_mut_refs)]
			Allocator::new_from_memory(&mut MEMORY)
		};

		let layout = Layout::from_size_align(16, 8).unwrap();
		let ptr = allocator.alloc_layout(layout).unwrap();
		unsafe { *ptr = 42 };

		allocator.free_ptr(ptr).unwrap();
		assert_eq!(allocator.slab32.free_count(), BLOCKS);
	}
}

pub struct MyGlobalAllocator<const NUM_32_BYTE_BLOCKS: usize, const NUM_32_BYTE_SLABS: usize> {
	allocator: &'static UnsafeCell<Allocator<'static, NUM_32_BYTE_BLOCKS, NUM_32_BYTE_SLABS>>,
}

// `GlobalAlloc` is safe to implement for single-threaded UnsafeCell use
unsafe impl<const NUM_32_BYTE_BLOCKS: usize, const NUM_32_BYTE_SLABS: usize> GlobalAlloc
	for MyGlobalAllocator<NUM_32_BYTE_BLOCKS, NUM_32_BYTE_SLABS>
{
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let allocator = &mut *self.allocator.get();
		match allocator.alloc_layout(layout) {
			Ok(ptr) => ptr,
			Err(_) => core::ptr::null_mut(),
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
		let allocator = &mut *self.allocator.get();
		let _ = allocator.free_ptr(ptr);
	}
}

unsafe impl<const NUM_32_BYTE_BLOCKS: usize, const NUM_32_BYTE_SLABS: usize> Sync
	for MyGlobalAllocator<NUM_32_BYTE_BLOCKS, NUM_32_BYTE_SLABS>
{
}

const BLOCKS: usize = 32;
const SLABS: usize = 4;

static mut MEMORY: [[[u8; 32]; BLOCKS]; SLABS] = [[[0; 32]; BLOCKS]; SLABS];
static mut ALLOCATOR_INSTANCE: UnsafeCell<Allocator<'static, BLOCKS, SLABS>> =
	UnsafeCell::new(unsafe {
		#[allow(static_mut_refs)]
		Allocator::new_from_memory(&mut MEMORY)
	});

// Wrap in a type implementing GlobalAlloc
#[global_allocator]
static GLOBAL_ALLOCATOR: MyGlobalAllocator<BLOCKS, SLABS> = MyGlobalAllocator {
	#[allow(static_mut_refs)]
	allocator: unsafe { &ALLOCATOR_INSTANCE },
};
