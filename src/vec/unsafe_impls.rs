use core::{ alloc::Layout, arch::x86_64, intrinsics, ptr::NonNull };
use crate::{ Vec, structs::InstructionSet, traits::{ ToBits, OrExplode } };

impl<T: ToBits> Vec<T> {
    pub(crate) fn alloc_buffer(&self, capacity: usize) -> *mut bool {
        unsafe_or_explode!(
            {
                let bit_capacity = capacity * self.bit_width;
                let layout = Layout::from_size_align(
                    bit_capacity.max(self.alignment),
                    self.alignment
                ).or_explode("Invalid layout");

                self.allocate_zeroed(layout).or_explode("Allocation failed").as_ptr() as *mut bool
            },
            "Allocation exploded"
        )
    }

    pub(crate) fn dealloc_buffer(&mut self) {
        unsafe_or_explode!(
            {
                if self.bit_capacity > 0 {
                    let layout = (
                        unsafe {
                            Layout::from_size_align(
                                self.bit_capacity.max(self.alignment),
                                self.alignment
                            )
                        }
                    ).or_explode("Invalid layout");
                    self.allocator.deallocate(NonNull::new_unchecked(self.data as *mut u8), layout);
                }
            },
            "Deallocation exploded"
        )
    }

    #[inline(always)]
    pub(crate) fn copy_simd(&mut self, src: *const bool, count: usize) {
        unsafe_or_explode!(
            {
                match self.instruction_set {
                    InstructionSet::AVX512 => {
                        for i in (0..count).step_by(64) {
                            let chunk = x86_64::_mm512_loadu_si512(src.add(i) as *const _);
                            x86_64::_mm512_stream_si512(
                                self.data.add(self.len + i) as *mut _,
                                chunk
                            );
                        }
                    }
                    InstructionSet::AVX2 => {
                        for i in (0..count).step_by(32) {
                            let chunk = x86_64::_mm256_loadu_si256(src.add(i) as *const _);
                            x86_64::_mm256_stream_si256(
                                self.data.add(self.len + i) as *mut _,
                                chunk
                            );
                        }
                    }
                    InstructionSet::SSE => {
                        for i in (0..count).step_by(16) {
                            let chunk = x86_64::_mm_loadu_si128(src.add(i) as *const _);
                            x86_64::_mm_stream_si128(self.data.add(self.len + i) as *mut _, chunk);
                        }
                    }
                    InstructionSet::None => {
                        core::ptr::copy_nonoverlapping(src, self.data.add(self.len), count);
                    }
                }
            },
            "SIMD copy exploded"
        )
    }

    #[inline(always)]
    pub(crate) fn prefetch(&self, ptr: *const bool, offset: usize) {
        unsafe_or_explode!(
            {
                intrinsics::prefetch_write_data(ptr.add(offset + 64) as *const u8, 2);
            },
            "Prefetch exploded"
        )
    }

    // Raw pointer operations
    #[inline(always)]
    pub(crate) fn get_unchecked(&self, index: usize) -> &T {
        unsafe_or_explode!(
            {
                &*(self.data.add(index * self.bit_width) as *const T)
            },
            "Get unchecked exploded"
        )
    }

    #[inline(always)]
    pub(crate) fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe_or_explode!(
            {
                &mut *(self.data.add(index * self.bit_width) as *mut T)
            },
            "Get unchecked mut exploded"
        )
    }

    // Add SIMD vectorized memory operations
    #[inline(always)]
    pub(crate) fn simd_memcpy(&mut self, src: *const bool, count: usize) {
        unsafe_or_explode!(
            {
                #[cfg(target_arch = "x86_64")]
                {
                    if is_x86_feature_detected!("avx512f") {
                        for i in (0..count).step_by(64) {
                            let chunk = x86_64::_mm512_loadu_si512(src.add(i) as *const _);
                            x86_64::_mm512_stream_si512(
                                self.data.add(self.len + i) as *mut _,
                                chunk
                            );
                        }
                    } else if is_x86_feature_detected!("avx2") {
                        for i in (0..count).step_by(32) {
                            let chunk = x86_64::_mm256_loadu_si256(src.add(i) as *const _);
                            x86_64::_mm256_stream_si256(
                                self.data.add(self.len + i) as *mut _,
                                chunk
                            );
                        }
                    }
                }
            },
            "SIMD memcpy exploded"
        )
    }

    // Optimized memory zeroing
    #[inline(always)]
    pub(crate) fn simd_memzero(&mut self, count: usize) {
        unsafe_or_explode!(
            {
                #[cfg(target_arch = "x86_64")]
                {
                    if is_x86_feature_detected!("avx512f") {
                        let zero = x86_64::_mm512_setzero_si512();
                        for i in (0..count).step_by(64) {
                            x86_64::_mm512_stream_si512(self.data.add(i) as *mut _, zero);
                        }
                    } else if is_x86_feature_detected!("avx2") {
                        let zero = x86_64::_mm256_setzero_si256();
                        for i in (0..count).step_by(32) {
                            x86_64::_mm256_stream_si256(self.data.add(i) as *mut _, zero);
                        }
                    }
                }
            },
            "SIMD memzero exploded"
        )
    }

    // SIMD comparison operations
    #[inline(always)]
    pub(crate) fn simd_memcmp(&self, other: &Self, count: usize) -> bool {
        unsafe_or_explode!(
            {
                let mut offset = 0;
                #[cfg(target_arch = "x86_64")]
                {
                    if is_x86_feature_detected!("avx512f") {
                        while offset + 64 <= count {
                            let a = x86_64::_mm512_loadu_si512(self.data.add(offset) as *const _);
                            let b = x86_64::_mm512_loadu_si512(other.data.add(offset) as *const _);
                            if x86_64::_mm512_cmpeq_epi64_mask(a, b) != 0xffff {
                                return false;
                            }
                            offset += 64;
                        }
                    }
                }
                true
            },
            "SIMD memcmp exploded"
        )
    }

    // Raw pointer manipulation
    #[inline(always)]
    pub(crate) fn get_element_ptr(&self, index: usize) -> *const T {
        unsafe_or_explode!(
            {
                self.data.add(index * self.bit_width) as *const T
            },
            "Get element ptr exploded"
        )
    }

    #[inline(always)]
    pub(crate) fn get_element_ptr_mut(&mut self, index: usize) -> *mut T {
        unsafe_or_explode!(
            {
                self.data.add(index * self.bit_width) as *mut T
            },
            "Get element ptr mut exploded"
        )
    }

    // Advanced memory prefetch
    #[inline(always)]
    pub(crate) fn prefetch_write_range(&self, start: usize, count: usize) {
        unsafe_or_explode!(
            {
                let mut offset = start;
                while offset < start + count {
                    intrinsics::prefetch_write_data(self.data.add(offset) as *const u8, 3);
                    offset += 64;
                }
            },
            "Prefetch write range exploded"
        )
    }

    #[inline(always)]
    pub(crate) fn prefetch_read_range(&self, start: usize, count: usize) {
        unsafe_or_explode!(
            {
                let mut offset = start;
                while offset < start + count {
                    intrinsics::prefetch_read_data(self.data.add(offset) as *const u8, 3);
                    offset += 64;
                }
            },
            "Prefetch read range exploded"
        )
    }

    // Optimized memory reallocation
    pub(crate) fn realloc(&mut self, new_cap: usize) -> bool {
        unsafe_or_explode!(
            {
                let size = new_cap * self.bit_width;
                let new_layout = Layout::from_size_align(size, self.alignment).or_explode(
                    "Invalid layout in realloc"
                );

                match self.allocator.allocate_zeroed(new_layout) {
                    Ok(ptr) => {
                        let new_ptr = ptr.as_ptr() as *mut bool;
                        if self.len > 0 {
                            core::ptr::copy_nonoverlapping(self.data, new_ptr, self.len);
                            self.allocator.deallocate(
                                NonNull::new_unchecked(self.data as *mut u8),
                                Layout::from_size_align_unchecked(self.bit_capacity, self.alignment)
                            );
                        }
                        self.data = new_ptr;
                        self.bit_capacity = new_cap * self.bit_width;
                        true
                    }
                    Err(_) => false,
                }
            },
            "Reallocation exploded"
        )
    }
}

// Add Splice iterator implementation
pub struct Splice<I> {
    vec: *mut Vec<I::Item>,
    iter: I,
    start: usize,
    end: usize,
}

impl<I> Iterator for Splice<I> where I: Iterator, I::Item: ToBits {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            unsafe {
                let vec = &mut *self.vec;
                let result = Some(vec[self.start].clone());
                self.start += 1;
                result
            }
        } else {
            self.iter.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        (self.end - self.start + lower, upper.map(|u| u + (self.end - self.start)))
    }
}
