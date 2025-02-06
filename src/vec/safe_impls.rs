//! Safe implementations for the `Vec` type.
//!
//! This module provides safe, non-architecture-specific implementations of standard
//! collection traits and core functionality for the `Vec` type.

use core::alloc::AllocError;
use core::cmp::Ordering;
use core::marker::PhantomData;

use crate::{ traits::{ ToBits }, structs::Vec };

/// A collection of safe methods for the `Vec` type that provides
/// bit-packed vector functionality with standard collection semantics.
impl<T: ToBits> Vec<T> {
    /// Returns `true` if the vector contains no elements.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements in the vector.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len / self.bit_width
    }

    /// Returns the total number of elements the vector can hold without reallocating.
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.bit_capacity / self.bit_width
    }

    /// Shortens the vector, keeping the first `len` elements and dropping the rest.
    #[inline(always)]
    pub fn truncate(&mut self, len: usize) {
        let new_bit_len = len * self.bit_width;
        if new_bit_len < self.len {
            self.len = new_bit_len;
        }
    }

    /// Reserves capacity for at least `additional` more elements or explodes.
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocError> {
        if self.bit_capacity - self.len >= additional * self.bit_width {
            Ok(())
        } else {
            let new_cap = self.len + additional * self.bit_width;
            if new_cap <= (isize::MAX as usize) {
                unsafe {
                    self.reserve(new_cap / self.bit_width - self.len());
                    Ok(())
                }
            } else {
                Err(AllocError)
            }
        }
    }

    /// New method to allow calling `reserve` in try_reserve
    pub fn reserve(&mut self, additional: usize) {
        unsafe_or_explode!(self.try_reserve(additional), "Reserve exploded");
    }

    /// New method to obtain a safe slice of elements
    pub fn as_slice(&self) -> &[T] {
        // Assumes that self.data is properly aligned and holds T elements.
        unsafe {
            core::slice::from_raw_parts(self.data as *const T, self.len())
        }
    }

    /// New method to return the number of stored elements
    pub fn len_in_elements(&self) -> usize {
        self.len()
    }

    /// Binary searches this sorted vector for a given element.
    #[inline(always)]
    pub fn binary_search(&self, x: &T) -> Result<usize, usize> where T: Ord {
        self.as_slice().binary_search(x)
    }

    /// Binary searches with a comparator function.
    #[inline(always)]
    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize> where F: FnMut(&T) -> Ordering {
        self.as_slice().binary_search_by(f)
    }

    /// Returns the index of the partition point according to the given predicate.
    #[inline(always)]
    pub fn partition_point<P>(&self, mut pred: P) -> usize where P: FnMut(&T) -> bool {
        let mut left = 0;
        let mut right = self.len_in_elements();

        while left != right {
            let mid = left + (right - left) / 2;
            if pred(&self[mid]) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    pub fn with_capacity(capacity: usize, bit_width: usize) -> Self {
        let mut vec = Self::new(bit_width, 0, 64);
        vec.bit_capacity = capacity * bit_width;
        vec.data = vec.alloc_buffer(capacity);
        vec
    }

    pub fn push(&mut self, item: T) {
        if self.len == self.bit_capacity {
            let new_capacity = (self.bit_capacity / self.bit_width).max(1) * 2;
            let new_data = self.alloc_buffer(new_capacity);
            self.copy_simd(self.data, self.len / self.bit_width);
            self.dealloc_buffer();
            self.data = new_data;
            self.bit_capacity = new_capacity * self.bit_width;
        }
        unsafe_or_explode!(
            {
                let ptr = self.data.add(self.len * self.bit_width) as *mut T;
                ptr.write(item);
            },
            "Push exploded"
        );
        self.len += self.bit_width;
    }
}
