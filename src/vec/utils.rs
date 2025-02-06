use crate::traits::BareSimd;

/// Efficient bit offset calculation.
///
macro_rules! bit_offset {
    ($self:ident, $index:expr) => {
        if ($self.bit_width & ($self.bit_width - 1)) == 0 {
            $index << $self.bit_width.trailing_zeros()
        } else {
            $index * $self.bit_width
        }
    };
}

/// Safe pointer arithmetic for bit vectors.
///
macro_rules! bit_ptr {
    ($self:ident, $index:expr) => {
        $self.data.add(bit_offset!($self, $index))
    };
}

/// Safe slice creation from raw parts.
///
macro_rules! bit_slice {
    ($ptr:expr, $len:expr) => {
        unsafe_or_explode!({
            slice::from_raw_parts($ptr, $len)
        }, "bit_slice exploded")
    };
}

/// Safe mutable slice creation.
///
macro_rules! bit_slice_mut {
    ($ptr:expr, $len:expr) => {
        unsafe_or_explode!({
            slice::from_raw_parts_mut($ptr, $len)
        }, "bit_slice_mut exploded")
    };
}

/// SIMD-optimized memory movement
macro_rules! simd_memcpy {
    ($dst:expr, $src:expr, $size:expr) => {
        if is_x86_feature_detected!("avx512f") {
            for i in (0..$size).step_by(64) {
                unsafe {
                    let chunk = _mm512_loadu_si512($src.add(i) as *const _);
                    _mm512_stream_si512($dst.add(i) as *mut _, chunk);
                }
            }
            _mm_sfence();
        } else if is_x86_feature_detected!("avx2") {
            for i in (0..$size).step_by(32) {
                unsafe {
                    let chunk = _mm256_loadu_si256($src.add(i) as *const _);
                    _mm256_stream_si256($dst.add(i) as *mut _, chunk);
                }
            }
        }
    };
}

/// Vectorized comparison operations
macro_rules! simd_compare {
    ($a:expr, $b:expr, $len:expr) => {
        if is_x86_feature_detected!("avx512f") {
            unsafe {
                let mut result = 0;
                for i in (0..$len).step_by(64) {
                    let va = _mm512_loadu_si512($a.add(i) as *const _);
                    let vb = _mm512_loadu_si512($b.add(i) as *const _);
                    result |= _mm512_cmpeq_epi64_mask(va, vb) as i32;
                }
                result
            }
        } else {
            // Fallback comparison
            0
        }
    };
}

/// Cache control utilities
macro_rules! prefetch_read {
    ($ptr:expr) => {
        if is_x86_feature_detected!("sse") {
            unsafe {
                _mm_prefetch($ptr as *const i8, _MM_HINT_T0);
            }
        }
    };
}

macro_rules! prefetch_write {
    ($ptr:expr) => {
        if is_x86_feature_detected!("sse") {
            unsafe {
                _mm_prefetch($ptr as *const i8, _MM_HINT_T0);
                _mm_mfence();
            }
        }
    };
}

/// Bit manipulation utilities
macro_rules! count_trailing_zeros {
    ($x:expr) => {
        if is_x86_feature_detected!("bmi1") {
            unsafe {
                _tzcnt_u64($x)
            }
        } else {
            $x.trailing_zeros()
        }
    };
}

macro_rules! count_leading_zeros {
    ($x:expr) => {
        if is_x86_feature_detected!("lzcnt") {
            unsafe {
                _lzcnt_u64($x)
            }
        } else {
            $x.leading_zeros()
        }
    };
}

/// Memory fence operations
macro_rules! memory_fence {
    () => {
        unsafe {
            _mm_mfence();
        }
    };
}

macro_rules! store_fence {
    () => {
        unsafe {
            _mm_sfence();
        }
    };
}

macro_rules! load_fence {
    () => {
        unsafe {
            _mm_lfence();
        }
    };
}

/// Vector broadcast operations
macro_rules! broadcast_element {
    ($x:expr, $ty:ty) => {
        if is_x86_feature_detected!("avx512f") {
            unsafe {
                match core::mem::size_of::<$ty>() {
                    8 => _mm512_set1_epi64($x as i64),
                    4 => _mm512_set1_epi32($x as i32),
                    2 => _mm512_set1_epi16($x as i16),
                    1 => _mm512_set1_epi8($x as i8),
                    _ => _mm512_setzero_si512(),
                }
            }
        }
    };
}

/// Optimized element copying between vectors.
///
macro_rules! copy_elements {
    ($src:expr, $dst:expr, $count:expr) => {
        ptr::copy($src, $dst, $count * $self.bit_width);
    };
}

/// Safe non-overlapping element copying.
///
macro_rules! copy_nonoverlapping_elements {
    ($src:expr, $dst:expr, $count:expr) => {
        ptr::copy_nonoverlapping($src, $dst, $count * $self.bit_width);
    };
}

// Utility for checking if CPU supports required SIMD features
#[inline]
pub fn check_simd_support() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        is_x86_feature_detected!("sse") &&
            is_x86_feature_detected!("sse2") &&
            is_x86_feature_detected!("avx")
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

#[macro_export]
macro_rules! unsafe_or_explode {
    ($expr:expr, $msg:expr) => {
        match unsafe { $expr } {
            value => value
        }
    };
}

#[macro_export]
macro_rules! detect_simd_support {
    () => {{
        #[cfg(target_feature = "avx512f")] {
            8 // AVX-512
        }
        #[cfg(all(not(target_feature = "avx512f"), target_feature = "avx2"))] {
            7 // AVX2
        }
        #[cfg(all(
            not(target_feature = "avx512f"),
            not(target_feature = "avx2"),
            target_feature = "avx"
        ))] {
            6 // AVX
        }
        #[cfg(all(
            not(target_feature = "avx512f"),
            not(target_feature = "avx2"),
            not(target_feature = "avx"),
            target_feature = "sse4.2"
        ))] {
            5 // SSE4.2
        }
        #[cfg(all(
            not(target_feature = "avx512f"),
            not(target_feature = "avx2"),
            not(target_feature = "avx"),
            not(target_feature = "sse4.2"),
            target_feature = "sse4.1"
        ))] {
            4 // SSE4.1
        }
        #[cfg(all(
            not(target_feature = "avx512f"),
            not(target_feature = "avx2"),
            not(target_feature = "avx"),
            not(target_feature = "sse4.2"),
            not(target_feature = "sse4.1"),
            target_feature = "sse3"
        ))] {
            3 // SSE3
        }
        #[cfg(all(
            not(target_feature = "avx512f"),
            not(target_feature = "avx2"),
            not(target_feature = "avx"),
            not(target_feature = "sse4.2"),
            not(target_feature = "sse4.1"),
            not(target_feature = "sse3"),
            target_feature = "sse2"
        ))] {
            2 // SSE2
        }
        #[cfg(all(
            not(target_feature = "avx512f"),
            not(target_feature = "avx2"),
            not(target_feature = "avx"),
            not(target_feature = "sse4.2"),
            not(target_feature = "sse4.1"),
            not(target_feature = "sse3"),
            not(target_feature = "sse2"),
            target_feature = "sse"
        ))] {
            1 // SSE
        }
        #[cfg(all(
            not(target_feature = "avx512f"),
            not(target_feature = "avx2"),
            not(target_feature = "avx"),
            not(target_feature = "sse4.2"),
            not(target_feature = "sse4.1"),
            not(target_feature = "sse3"),
            not(target_feature = "sse2"),
            not(target_feature = "sse")
        ))] {
            0 // Fallback
        }
    }} as u8
}

// Usage example:
let instruction_set = detect_simd_support!();


// Generic SIMD vector creation helpers
#[inline]
pub fn simd_splat<T: BareSimd>(value: T::Element) -> T {
    T::splat(value)
}

#[inline]
pub fn simd_zero<T: BareSimd>() -> T {
    T::zero()
}

// Generic SIMD math operations
#[inline]
pub fn simd_abs<T: BareSimd>(v: T) -> T {
    v.abs()
}

#[inline]
pub fn simd_min<T: BareSimd>(a: T, b: T) -> T {
    a.min(b)
}

#[inline]
pub fn simd_max<T: BareSimd>(a: T, b: T) -> T {
    a.max(b)
}

// SIMD masking utilities
#[inline]
pub fn simd_blend<T: BareSimd>(a: T, b: T, mask: T::Mask) -> T {
    T::blend(a, b, mask)
}

#[inline]
pub fn simd_select<T: BareSimd>(mask: T::Mask, a: T, b: T) -> T {
    T::select(mask, a, b)
}
