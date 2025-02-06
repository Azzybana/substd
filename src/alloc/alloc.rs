//! Don't use this, unless you enjoy panic in production.
//! Sometimes you need it to go brrr, and you can restart it if it crashes.
//! It has enough sanity checking baked in to at least tell you what exploded.
//! If you feed it valid numbers, it won't explode, so go fix your code.
//! It's not meant to produce debug info or care about safety checks otherwise.
//! Speed, Safety, Resource usage... pick 2.

use core::{ ptr::null_mut, mem::size_of };

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{ __cpuid, __cpuid_count };
#[cfg(target_arch = "x86")]
use core::arch::x86::{ __cpuid, __cpuid_count };
use libc::{ mmap, PROT_READ, PROT_WRITE, MAP_PRIVATE, MAP_ANONYMOUS };
use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::winnt::{ MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE };

// Macros for SIMD detection
#[macro_export]
macro_rules! any_simd_support {
    () => {
        #[cfg(
            any(
                target_feature = "avx512f",
                target_feature = "avx",
                target_feature = "avx2",
                target_feature = "sse4.2",
                target_feature = "sse4.1",
                target_feature = "ssse3",
                target_feature = "sse3",
                target_feature = "sse2",
                target_feature = "sse"
            )
        )]
        #[repr(C, align(8))] // Basic 8-byte alignment
    };
}

#[macro_export]
macro_rules! avx512_support {
    () => {
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        #[repr(C, align(64))] // AVX-512 alignment
    };
}

macro_rules! any_avx_support {
    () => {
        #[cfg(
            all(
                any(target_arch = "x86_64", target_arch = "x86"),
                all(
                        any(target_feature = "avx", target_feature = "avx2"), 
                        not(target_feature = "avx512f")
                )
            )
        )]
        #[repr(C, align(32))] // AVX alignment (256)
    };
}

macro_rules! avx2_support {
    () => {
        #[cfg(
            all(
                any(target_arch = "x86_64", target_arch = "x86"),
                all(
                        target_feature = "avx2", 
                        not(target_feature = "avx512f")
                )
            )
        )]
        #[repr(C, align(32))] // AVX alignment
    };
}

macro_rules! avx_support {
    () => {
        #[cfg(
            all(
                any(target_arch = "x86_64", target_arch = "x86"),
                all(
                        target_feature = "avx", 
                        not(target_feature = "avx512f", target_feature = "avx2")
                )
            )
        )]
        #[repr(C, align(32))] // AVX alignment
    };
}

macro_rules! any_sse_support {
    () => {
        #[cfg(
            all(
                any(target_arch = "x86_64", target_arch = "x86"),
                all(
                        target_feature = "sse", 
                        not(target_feature = "avx512f", 
                            target_feature = "avx2",
                            target_feature = "avx",)
                )
            )
        )]
        #[repr(C, align(16))] // AVX alignment
    };
}

macro_rules! sse42_support {
    () => {
        #[cfg(
            all(
                any(target_arch = "x86_64", target_arch = "x86"),
                all(
                        target_feature = "sse4.2", 
                        not(target_feature = "avx512f", 
                            target_feature = "avx2",
                            target_feature = "avx",)
                )
            )
        )]
        #[repr(C, align(16))] // AVX alignment
    };
}

macro_rules! sse41_support {
    () => {
        #[cfg(
            all(
                any(target_arch = "x86_64", target_arch = "x86"),
                all(
                        target_feature = "sse4.1", 
                        not(target_feature = "avx512f", 
                            target_feature = "avx2",
                            target_feature = "avx",
                            target_feature = "sse4.2",)
                )
            )
        )]
        #[repr(C, align(16))] // AVX alignment
    };
}

macro_rules! sse3_support {
    () => {
        #[cfg(
            all(
                any(target_arch = "x86_64", target_arch = "x86"),
                all(
                        target_feature = "sse3", 
                        not(target_feature = "avx512f", 
                            target_feature = "avx2",
                            target_feature = "avx",
                            target_feature = "sse4.2",
                            target_feature = "sse4.1",
                            target_feature = "sse3",)
                )
            )
        )]
        #[repr(C, align(16))] // AVX alignment
    };
}

macro_rules! sse2_support {
    () => {
        #[cfg(
            all(
                any(target_arch = "x86_64", target_arch = "x86"),
                all(
                        target_feature = "sse2", 
                        not(target_feature = "avx512f", 
                            target_feature = "avx2",
                            target_feature = "avx",
                            target_feature = "sse4.2",
                            target_feature = "sse4.1",
                            target_feature = "sse3",)
                )
            )
        )]
        #[repr(C, align(16))] // AVX alignment
    };
}
macro_rules! sse_support {
    () => {
        #[cfg(
            all(
                any(target_arch = "x86_64", target_arch = "x86"),
                all(
                        target_feature = "sse", 
                        not(target_feature = "avx512f", 
                            target_feature = "avx2",
                            target_feature = "avx",
                            target_feature = "sse4.2",
                            target_feature = "sse4.1",
                            target_feature = "sse3",
                            target_feature = "sse2")
                )
            )
        )]
        #[repr(C, align(16))] // AVX alignment
    };
}

#[macro_export]
macro_rules! no_simd_support {
    () => {
        #[cfg(
            not(
                any(
                    target_feature = "avx512f",
                    target_feature = "avx",
                    target_feature = "avx2",
                    target_feature = "sse",
                    target_feature = "sse2",
                    target_feature = "sse3",
                    target_feature = "sse4.1",
                    target_feature = "sse4.2"
                )
            )
        )]
        #[repr(C, align(8))]
    };
}

avx512_support!();
#[derive(Copy, Clone)]
pub struct CacheInfo {
    // Hot 32-byte block (AVX-256 aligned)
    pub cache_size: u32, // +0  (4-byte aligned)
    pub prefetch_size: u16, // +4  (2-byte aligned)
    pub line_size: u8, // +6
    _padding1: u8, // +7  (maintain 8-byte alignment)

    // Cold 32-byte block
    pub cache_sets: u16, // +8  (2-byte aligned)
    pub associativity: u8, // +10
    pub cache_level: u8, // +11
    pub shared_cores: u8, // +12
    _padding2: [u8; 19], // +13-31 (pad to 32 bytes)

    // Ensure 64-byte total size
    _padding3: [u8; 32], // +32-63 (pad to 64 bytes)
}

any_avx_support!();
#[derive(Copy, Clone)]
pub struct CacheInfo {
    // Hot 16-byte block
    pub cache_size: u32, // +0  (4-byte aligned)
    pub prefetch_size: u16, // +4  (2-byte aligned)
    pub line_size: u8, // +6
    pub cache_level: u8, // +7
    pub cache_sets: u16, // +8  (2-byte aligned)
    pub associativity: u8, // +10
    pub shared_cores: u8, // +11
    _padding1: [u8; 4], // +12-15 (align to 16)

    // Cold 16-byte block
    _padding2: [u8; 16], // +16-31 (pad to 32 bytes)
}

any_sse_support!();
#[derive(Copy, Clone)]
pub struct CacheInfo {
    // Hot 8-byte block (First XMM half)
    pub cache_size: u32, // +0  (4-byte aligned)
    pub prefetch_size: u16, // +4  (2-byte aligned)
    pub line_size: u8, // +6
    pub cache_level: u8, // +7

    // Cold 8-byte block (Second XMM half)
    pub cache_sets: u16, // +8  (2-byte aligned)
    pub associativity: u8, // +10
    pub shared_cores: u8, // +11
    _padding: [u8; 4], // +12-15 (align to 16)
}

no_simd_support!();
#[repr(C, align(8))] // Basic 8-byte alignment
#[derive(Copy, Clone)]
pub struct CacheInfo {
    // Hot 4-byte block
    pub cache_size: u32, // +0  (4-byte aligned)

    // Medium 2-byte block
    pub prefetch_size: u16, // +4  (2-byte aligned)
    pub cache_sets: u16, // +6  (2-byte aligned)

    // Cold 1-byte fields + padding
    pub line_size: u8, // +8
    pub cache_level: u8, // +9
    pub associativity: u8, // +10
    pub shared_cores: u8, // +11
    _padding: [u8; 4], // +12-15 (align to 8)
}

impl CacheInfo {
    #[inline]
    pub fn new() -> Self {
        Self.detect()
    }

    #[inline]
    pub unsafe fn detect() -> Self {
        any_simd_support!();
        unsafe_or_explode! {
            // Get CPU info
            let cpu_info = __cpuid(1);
            let cache = __cpuid(0x0000_0004);

            // Feature detection
            let has_avx512f = (cpu_info.ecx & (1 << 28)) != 0;
            let has_avx2 = (cpu_info.ebx & (1 << 5)) != 0;
            let has_avx = (cpu_info.ecx & (1 << 28)) != 0;
            let has_sse4_2 = (cpu_info.ecx & (1 << 20)) != 0;
            let has_sse4_1 = (cpu_info.ecx & (1 << 19)) != 0;
            let has_ssse3 = (cpu_info.ecx & (1 << 9)) != 0;
            let has_sse3 = (cpu_info.ecx & (1 << 0)) != 0;
            let has_sse2 = (cpu_info.edx & (1 << 26)) != 0;
            let has_sse = (cpu_info.edx & (1 << 25)) != 0;

            // Cache info extraction
            let line_size = (cache.eax & 0xff) as u8;
            let cache_level = ((cache.eax >> 5) & 0x7) as u8;
            let associativity = ((cache.ebx >> 22) & 0x3ff) as u8;
            let shared_cores = ((cache.eax >> 14) & 0xfff) as u8;
            let cache_sets = (cache.ecx + 1) as u16;

            // Determine prefetch size based on available features
            let prefetch_size = if has_avx512f {
                (line_size as u16) * 8 // 512-bit operations
            } else if has_avx2 || has_avx {
                (line_size as u16) * 4 // 256-bit operations
            } else if has_sse4_2 || has_sse4_1 {
                (line_size as u16) * 2 // 128-bit operations with newer SSE
            } else if has_ssse3 || has_sse3 || has_sse2 || has_sse {
                line_size as u16 // Basic 128-bit operations
            } else {
                line_size as u16 // Fallback
            };

            Self {
                line_size,
                cache_level,
                associativity,
                prefetch_size,
                cache_size: ((cache.ebx >> 22) & 0x3ff) *
                ((cache.ebx >> 12) & 0x3ff) *
                (line_size as u32) *
                (cache_sets as u32),
                cache_sets,
                shared_cores,
                _padding: [0; 4],
            };
        }

        // Fallback for non-x86 architectures
        no_simd_support!();
        const fn fallback() -> Self {
            Self {
                line_size: 64, // Common default
                cache_level: 1,
                associativity: 8,
                prefetch_size: 64,
                cache_size: 32768, // 32KB default L1
                cache_sets: 64,
                shared_cores: 1,
                _padding: [0; 4],
            }
        }
    }
}

// Yummy MemoryMap slices
#[repr(transparent)]
pub struct MemoryMap {
    data: [u8; u8],
}

impl MemoryMap {
    pub fn new() -> Self {
        avx512_support!();
        Self { data: [0; 64] }; // AVX-512: 512 bits
        any_avx_support!();
        Self { data: [0; 32] }; // AVX/AVX2: 256 bits
        any_sse_support!();
        Self { data: [0; 16] }; // SSE variants: 128 bits
        no_simd_support!();
        Self { data: [0; 8] }; // Basic: 64 bits
    }
}

pub struct Allocator {
    // Current chunk - pointers must match architecture
    current_chunk: *mut u8,
    current_offset: u32, // 4GB per chunk is reasonable

    // Cache specifics - modern CPUs typically < 1MB L1
    cache_info: CacheInfo,
    l1_cache_size: u16, // 64KB is common for L1
    prefetch_size: u8, // 256 bytes max is typical

    // Adding a MemoryMap via slices, since I like pi, lol
    mem_map: MemoryMap,

    // Chunk management
    chunk_size: u32, // 4GB max chunk
    free_chunks: *mut u8,
    largest_free_chunk: *mut u8,

    // Masks/alignments - architecture dependent
    align_mask: usize, // Must match pointer size
    chunk_mask: usize, // Must match pointer size

    // Limits
    max_bytes: usize, // 4GB total is reasonable
    free_bytes: usize, // Track against max_bytes
    used_bytes: usize, // Track against max_bytes
}

impl Allocator {
    #[inline]
    pub const fn new() -> Self {
        let cache_info = CacheInfo::detect();
        Self {
            // Current chunk - pointers must match architecture
            current_chunk: null_mut(),
            current_offset: 0,

            // Cache specifics - modern CPUs typically < 1MB L1
            cache_info,
            l1_cache_size: (cache_info.l1_size >> 10) as u16, // Convert to KB
            prefetch_size: cache_info.prefetch_size as u8,

            // Adding a MemoryMap via slices, since I like pi, lol
            mem_map: MemoryMap::new(),

            // Chunk management
            chunk_size: 1 << 20, // 1MB chunks
            free_chunks: null_mut(),
            largest_free_chunk: null_mut(),

            // Masks/alignments - architecture dependent
            align_mask: sizeof::<usize>() - 1,
            chunk_mask: !((1 << 20) - 1), // Invert chunk size - 1

            // Limits
            max_bytes: usize::MAX,
            free_bytes: usize::MAX,
            used_bytes: 0,
        }
    }

    // https://github.com/rust-lang/rust/blob/master/library/alloc/src/alloc.rs#L23
    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        if size > self.max_bytes - self.used_bytes {
            return null_mut(); // Out of memory
        }

        // Align size to cache line and chunk boundaries
        let aligned_size = (size + self.align_mask) & !self.align_mask;
        let aligned_chunk = (aligned_size + self.chunk_mask) & !self.chunk_mask;

        // Check if current chunk is sufficient
        if self.current_offset + aligned_chunk > self.chunk_size {
            // Allocate new chunk if needed
            self.current_chunk = unsafe {
                core::alloc::alloc_zeroed(
                    core::alloc::Layout::from_size_align(self.chunk_size as usize, 1).unwrap()
                )
            };
            if self.current_chunk.is_null() {
                return null_mut(); // Allocation failed
            }
            self.current_offset = 0;
        }

        // Allocate memory from current chunkcan
        let ptr = unsafe { self.current_chunk.add(self.current_offset as usize) };
        self.current_offset += aligned_chunk as u32;
        self.used_bytes += aligned_size;

        ptr
    }

    pub fn grow(&mut self, size: usize) -> *mut u8 {
        if size > self.max_bytes - self.used_bytes {
            return null_mut(); // Out of memory
        }

        // Align size to cache line and chunk boundaries
        let aligned_size = (size + self.align_mask) & !self.align_mask;
        let aligned_chunk = (aligned_size + self.chunk_mask) & !self.chunk_mask;

        // Check if current chunk is sufficient
        if self.current_offset + aligned_chunk > self.chunk_size {
            // Allocate new chunk if needed
            self.current_chunk = unsafe {
                core::alloc::alloc_zeroed(
                    core::alloc::Layout::from_size_align(self.chunk_size as usize, 1).unwrap()
                )
            };
            self.current_offset = 0;
        }

        // Allocate memory from current chunk
        let ptr = unsafe { self.current_chunk.add(self.current_offset as usize) };
        self.current_offset += aligned_chunk as u32;
        self.used_bytes += aligned_size;

        ptr
    }

    pub fn dealloc(&mut self, ptr: *mut u8) {
        unsafe {
            core::alloc::dealloc(
                ptr,
                core::alloc::Layout::from_size_align(self.chunk_size as usize, 1).unwrap()
            );
        }
    }

    pub fn shrink(&mut self, size: usize) -> *mut u8 {
        if size > self.used_bytes {
            return null_mut(); // Cannot shrink beyond used bytes
        }

        // Align size to cache line and chunk boundaries
        let aligned_size = (size + self.align_mask) & !self.align_mask;
        let aligned_chunk = (aligned_size + self.chunk_mask) & !self.chunk_mask;

        // Check if current chunk is sufficient
        if self.current_offset < aligned_chunk {
            // Deallocate current chunk if needed
            unsafe {
                core::alloc::dealloc(
                    self.current_chunk,
                    core::alloc::Layout::from_size_align(self.chunk_size as usize, 1).unwrap()
                );
            }
            self.current_chunk = null_mut();
            self.current_offset = 0;
        }

        // Deallocate memory from current chunk
        let ptr = unsafe { self.current_chunk.add(self.current_offset as usize) };
        self.current_offset -= aligned_chunk as u32;
        self.used_bytes -= aligned_size;

        ptr
    }

    pub fn realloc(&mut self, ptr: *mut u8, new_size: usize) -> *mut u8 {
        if new_size > self.max_bytes - self.used_bytes {
            return null_mut(); // Out of memory
        }

        // Align size to cache line and chunk boundaries
        let aligned_size = (new_size + self.align_mask) & !self.align_mask;
        let aligned_chunk = (aligned_size + self.chunk_mask) & !self.chunk_mask;

        // Check if current chunk is sufficient
        if self.current_offset + aligned_chunk > self.chunk_size {
            // Allocate new chunk if needed
            self.current_chunk = unsafe {
                core::alloc::alloc_zeroed(
                    core::alloc::Layout::from_size_align(self.chunk_size as usize, 1).unwrap()
                )
            };
            self.current_offset = 0;
        }

        // Reallocate memory from current chunk
        let new_ptr = unsafe { self.current_chunk.add(self.current_offset as usize) };
        self.current_offset += aligned_chunk as u32;
        self.used_bytes += aligned_size;

        unsafe {
            core::ptr::copy_nonoverlapping(ptr, new_ptr, aligned_size);
            core::alloc::dealloc(
                ptr,
                core::alloc::Layout::from_size_align(self.chunk_size as usize, 1).unwrap()
            );
        }

        new_ptr
    }
    pub fn defrag(&mut self) {
        if self.current_chunk.is_null() {
            return false;
        }

        unsafe {
            unsafe {
                // What seems to work best, is to reserve a cache line,
                // then scan and compact, then update the allocator state.
                // I'm using a left-shift based on space.

                // Phase 1: Reserve cache space
                let cache_reserve =
                    ((self.prefetch_size as usize) + self.align_mask) & !self.align_mask;
                let mut write_ptr = self.current_chunk.add(cache_reserve);
                let end_ptr = self.current_chunk.add(self.used_bytes as usize);
                let mut read_ptr = write_ptr;

                // Phase 2 & 3: Scan and compact
                while read_ptr < end_ptr {
                    // Find next non-zero block
                    let mut block_start = read_ptr;
                    while block_start < end_ptr && *block_start == 0 {
                        block_start = block_start.add(1);
                    }

                    if block_start >= end_ptr {
                        break;
                    }

                    // Find block end
                    let mut block_end = block_start;
                    while block_end < end_ptr && *block_end != 0 {
                        block_end = block_end.add(1);
                    }

                    // Calculate aligned size
                    let block_size = block_end.offset_from(block_start) as usize;
                    let aligned_size = (block_size + self.align_mask) & !self.align_mask;

                    // Move block if needed
                    if write_ptr != block_start {
                        std::ptr::copy(block_start, write_ptr, block_size);
                        // Zero out old location
                        std::ptr::write_bytes(block_start, 0, block_size);
                    }

                    // Update write pointer with alignment
                    write_ptr = write_ptr.add(aligned_size);
                    read_ptr = block_end;
                }

                // Phase 4: Update allocator state
                let new_used = write_ptr.offset_from(self.current_chunk) as u32;
                if new_used < self.used_bytes {
                    self.used_bytes = new_used;
                    self.free_bytes = self.max_bytes - self.used_bytes;
                    true
                } else {
                    false
                }
            }
        }
    }
    pub fn prune(&mut self) -> bool {
        if self.current_chunk.is_null() {
            return false;
        }

        unsafe {
            // Start from end of used memory
            let mut end = self.current_chunk.add(self.used_bytes as usize);
            let start = self.current_chunk;

            // Scan backward for last non-zero byte
            while end > start && *end.sub(1) == 0 {
                end = end.sub(1);
            }

            // Calculate new size with alignments
            let base_size = end.offset_from(start) as usize;
            let cache_pad = self.prefetch_size as usize;
            let aligned_size = (base_size + cache_pad + self.align_mask) & !self.align_mask;

            // Update allocator state if smaller
            if aligned_size < (self.used_bytes as usize) {
                self.used_bytes = aligned_size as u32;
                self.free_bytes = self.max_bytes - self.used_bytes;
                true
            } else {
                false
            }
        }
    }
    pub fn trim(&mut self) -> bool {
        if self.current_chunk.is_null() {
            return false;
        }

        unsafe {
            // Find first used byte
            let mut start = self.current_chunk;
            let mut end = self.current_chunk.add(self.used_bytes as usize);

            // Scan forward for first non-zero byte
            while start < end && *start == 0 {
                start = start.add(1);
            }

            // Scan backward for last non-zero byte
            while end > start && *end.sub(1) == 0 {
                end = end.sub(1);
            }

            // Calculate new size needed (with alignment)
            let used_size = end.offset_from(start) as usize;
            let aligned_size = (used_size + self.align_mask) & !self.align_mask;

            // Update allocator state
            self.max_bytes = aligned_size as u32;
            self.used_bytes = used_size as u32;
            self.free_bytes = self.max_bytes - self.used_bytes;

            // Reallocate memory if needed
            let new_chunk = self.reallocate(self.current_chunk, aligned_size);
            if !new_chunk.is_null() {
                self.current_chunk = new_chunk;
                true
            } else {
                false
            }
        }
    }
    pub fn clear(&mut self) {
        // Quick and Dirty, writing 0's takes time.
        self.current_chunk = null_mut();
        self.current_offset = 0;
        self.free_chunks = null_mut();
        self.largest_free_chunk = null_mut();
        self.free_bytes = self.max_bytes;
        self.used_bytes = 0;
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        self.clear();
    }
}
