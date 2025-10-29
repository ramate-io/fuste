use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::cell::UnsafeCell;
use fuste_alloc::allocator::Allocator;

pub struct Gallocator<
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
> {
	pub(crate) allocator: &'static UnsafeCell<
		Allocator<
			'static,
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
		>,
	>,
}

// `GlobalAlloc` is safe to implement for single-threaded UnsafeCell use
unsafe impl<
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
	> GlobalAlloc
	for Gallocator<
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

unsafe impl<
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
	> Sync
	for Gallocator<
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
}
