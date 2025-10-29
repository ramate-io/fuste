use crate::Gallocator;
use core::cell::UnsafeCell;
use fuste_alloc::allocator::Allocator;
use fuste_alloc::slab::homo::HomoSlabCache;
use fuste_alloc::slab::Slab;

// 32 byte blocks and slabs
static NUM_32_BYTE_BLOCKS: usize = 32;
static NUM_32_BYTE_SLABS: usize = 32;
static mut MEMORY_32: [[[u8; 32]; NUM_32_BYTE_BLOCKS]; NUM_32_BYTE_SLABS] =
	[[[0; 32]; NUM_32_BYTE_BLOCKS]; NUM_32_BYTE_SLABS];

// 64 byte blocks and slabs
static NUM_64_BYTE_BLOCKS: usize = 0;
static NUM_64_BYTE_SLABS: usize = 0;
static mut _MEMORY_64: [[[u8; 64]; NUM_64_BYTE_BLOCKS]; NUM_64_BYTE_SLABS] =
	[[[0; 64]; NUM_64_BYTE_BLOCKS]; NUM_64_BYTE_SLABS];

// 128 byte blocks and slabs
static NUM_128_BYTE_BLOCKS: usize = 32;
static NUM_128_BYTE_SLABS: usize = 16;
static mut MEMORY_128: [[[u8; 128]; NUM_128_BYTE_BLOCKS]; NUM_128_BYTE_SLABS] =
	[[[0; 128]; NUM_128_BYTE_BLOCKS]; NUM_128_BYTE_SLABS];

// 256 byte blocks and slabs
static NUM_256_BYTE_BLOCKS: usize = 32;
static NUM_256_BYTE_SLABS: usize = 8;
static mut MEMORY_256: [[[u8; 256]; NUM_256_BYTE_BLOCKS]; NUM_256_BYTE_SLABS] =
	[[[0; 256]; NUM_256_BYTE_BLOCKS]; NUM_256_BYTE_SLABS];

// 512 byte blocks and slabs
static NUM_512_BYTE_BLOCKS: usize = 32;
static NUM_512_BYTE_SLABS: usize = 4;
static mut MEMORY_512: [[[u8; 512]; NUM_512_BYTE_BLOCKS]; NUM_512_BYTE_SLABS] =
	[[[0; 512]; NUM_512_BYTE_BLOCKS]; NUM_512_BYTE_SLABS];

// 1024 byte blocks and slabs
static NUM_1024_BYTE_BLOCKS: usize = 16;
static NUM_1024_BYTE_SLABS: usize = 3;
static mut MEMORY_1024: [[[u8; 1024]; NUM_1024_BYTE_BLOCKS]; NUM_1024_BYTE_SLABS] =
	[[[0; 1024]; NUM_1024_BYTE_BLOCKS]; NUM_1024_BYTE_SLABS];

// 2048 byte blocks and slabs
static NUM_2048_BYTE_BLOCKS: usize = 16;
static NUM_2048_BYTE_SLABS: usize = 1;
static mut MEMORY_2048: [[[u8; 2048]; NUM_2048_BYTE_BLOCKS]; NUM_2048_BYTE_SLABS] =
	[[[0; 2048]; NUM_2048_BYTE_BLOCKS]; NUM_2048_BYTE_SLABS];

// 4096 byte blocks and slabs
static NUM_4096_BYTE_BLOCKS: usize = 8;
static NUM_4096_BYTE_SLABS: usize = 1;
static mut MEMORY_4096: [[[u8; 4096]; NUM_4096_BYTE_BLOCKS]; NUM_4096_BYTE_SLABS] =
	[[[0; 4096]; NUM_4096_BYTE_BLOCKS]; NUM_4096_BYTE_SLABS];

// 8192 byte blocks and slabs
static NUM_8192_BYTE_BLOCKS: usize = 2;
static NUM_8192_BYTE_SLABS: usize = 1;
static mut MEMORY_8192: [[[u8; 8192]; NUM_8192_BYTE_BLOCKS]; NUM_8192_BYTE_SLABS] =
	[[[0; 8192]; NUM_8192_BYTE_BLOCKS]; NUM_8192_BYTE_SLABS];

static mut ALLOCATOR_INSTANCE: UnsafeCell<
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
> = UnsafeCell::new(Allocator {
	slab32: HomoSlabCache::<'static, 32, 32, 32>::new_const([
		Slab::new_const(unsafe { &mut MEMORY_32[0] }),
		Slab::new_const(unsafe { &mut MEMORY_32[1] }),
		Slab::new_const(unsafe { &mut MEMORY_32[2] }),
		Slab::new_const(unsafe { &mut MEMORY_32[3] }),
		Slab::new_const(unsafe { &mut MEMORY_32[4] }),
		Slab::new_const(unsafe { &mut MEMORY_32[5] }),
		Slab::new_const(unsafe { &mut MEMORY_32[6] }),
		Slab::new_const(unsafe { &mut MEMORY_32[7] }),
		Slab::new_const(unsafe { &mut MEMORY_32[8] }),
		Slab::new_const(unsafe { &mut MEMORY_32[9] }),
		Slab::new_const(unsafe { &mut MEMORY_32[10] }),
		Slab::new_const(unsafe { &mut MEMORY_32[11] }),
		Slab::new_const(unsafe { &mut MEMORY_32[12] }),
		Slab::new_const(unsafe { &mut MEMORY_32[13] }),
		Slab::new_const(unsafe { &mut MEMORY_32[14] }),
		Slab::new_const(unsafe { &mut MEMORY_32[15] }),
		Slab::new_const(unsafe { &mut MEMORY_32[16] }),
		Slab::new_const(unsafe { &mut MEMORY_32[17] }),
		Slab::new_const(unsafe { &mut MEMORY_32[18] }),
		Slab::new_const(unsafe { &mut MEMORY_32[19] }),
		Slab::new_const(unsafe { &mut MEMORY_32[20] }),
		Slab::new_const(unsafe { &mut MEMORY_32[21] }),
		Slab::new_const(unsafe { &mut MEMORY_32[22] }),
		Slab::new_const(unsafe { &mut MEMORY_32[23] }),
		Slab::new_const(unsafe { &mut MEMORY_32[24] }),
		Slab::new_const(unsafe { &mut MEMORY_32[25] }),
		Slab::new_const(unsafe { &mut MEMORY_32[26] }),
		Slab::new_const(unsafe { &mut MEMORY_32[27] }),
		Slab::new_const(unsafe { &mut MEMORY_32[28] }),
		Slab::new_const(unsafe { &mut MEMORY_32[29] }),
		Slab::new_const(unsafe { &mut MEMORY_32[30] }),
		Slab::new_const(unsafe { &mut MEMORY_32[31] }),
	]),
	slab64: HomoSlabCache::<'static, 64, NUM_64_BYTE_BLOCKS, NUM_64_BYTE_SLABS>::new_const([]),
	slab128: HomoSlabCache::<'static, 128, NUM_128_BYTE_BLOCKS, NUM_128_BYTE_SLABS>::new_const([
		Slab::new_const(unsafe { &mut MEMORY_128[0] }),
		Slab::new_const(unsafe { &mut MEMORY_128[1] }),
		Slab::new_const(unsafe { &mut MEMORY_128[2] }),
		Slab::new_const(unsafe { &mut MEMORY_128[3] }),
		Slab::new_const(unsafe { &mut MEMORY_128[4] }),
		Slab::new_const(unsafe { &mut MEMORY_128[5] }),
		Slab::new_const(unsafe { &mut MEMORY_128[6] }),
		Slab::new_const(unsafe { &mut MEMORY_128[7] }),
		Slab::new_const(unsafe { &mut MEMORY_128[8] }),
		Slab::new_const(unsafe { &mut MEMORY_128[9] }),
		Slab::new_const(unsafe { &mut MEMORY_128[10] }),
		Slab::new_const(unsafe { &mut MEMORY_128[11] }),
		Slab::new_const(unsafe { &mut MEMORY_128[12] }),
		Slab::new_const(unsafe { &mut MEMORY_128[13] }),
		Slab::new_const(unsafe { &mut MEMORY_128[14] }),
		Slab::new_const(unsafe { &mut MEMORY_128[15] }),
	]),
	slab256: HomoSlabCache::<'static, 256, NUM_256_BYTE_BLOCKS, NUM_256_BYTE_SLABS>::new_const([
		Slab::new_const(unsafe { &mut MEMORY_256[0] }),
		Slab::new_const(unsafe { &mut MEMORY_256[1] }),
		Slab::new_const(unsafe { &mut MEMORY_256[2] }),
		Slab::new_const(unsafe { &mut MEMORY_256[3] }),
		Slab::new_const(unsafe { &mut MEMORY_256[4] }),
		Slab::new_const(unsafe { &mut MEMORY_256[5] }),
		Slab::new_const(unsafe { &mut MEMORY_256[6] }),
		Slab::new_const(unsafe { &mut MEMORY_256[7] }),
	]),
	slab512: HomoSlabCache::<'static, 512, NUM_512_BYTE_BLOCKS, NUM_512_BYTE_SLABS>::new_const([
		Slab::new_const(unsafe { &mut MEMORY_512[0] }),
		Slab::new_const(unsafe { &mut MEMORY_512[1] }),
		Slab::new_const(unsafe { &mut MEMORY_512[2] }),
		Slab::new_const(unsafe { &mut MEMORY_512[3] }),
	]),
	slab1024: HomoSlabCache::<'static, 1024, NUM_1024_BYTE_BLOCKS, NUM_1024_BYTE_SLABS>::new_const(
		[
			Slab::new_const(unsafe { &mut MEMORY_1024[0] }),
			Slab::new_const(unsafe { &mut MEMORY_1024[1] }),
			Slab::new_const(unsafe { &mut MEMORY_1024[2] }),
		],
	),
	slab2048: HomoSlabCache::<'static, 2048, NUM_2048_BYTE_BLOCKS, NUM_2048_BYTE_SLABS>::new_const(
		[Slab::new_const(unsafe { &mut MEMORY_2048[0] })],
	),
	slab4096: HomoSlabCache::<'static, 4096, NUM_4096_BYTE_BLOCKS, NUM_4096_BYTE_SLABS>::new_const(
		[Slab::new_const(unsafe { &mut MEMORY_4096[0] })],
	),
	slab8192: HomoSlabCache::<'static, 8192, NUM_8192_BYTE_BLOCKS, NUM_8192_BYTE_SLABS>::new_const(
		[Slab::new_const(unsafe { &mut MEMORY_8192[0] })],
	),
});

#[global_allocator]
pub static GLOBAL_ALLOCATOR: Gallocator<
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
> = Gallocator {
	allocator: unsafe {
		#[allow(static_mut_refs)]
		&ALLOCATOR_INSTANCE
	},
};
