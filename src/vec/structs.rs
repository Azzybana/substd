//! Core data structures for the Vec crate.
//!
//! # Architecture-specific Optimizations
//!
//! The `Vec` struct uses different alignment strategies:
//! - 64-byte alignment for x86_64 (optimized for AVX-512)
//! - 32-byte alignment for x86
//! - 16-byte alignment for other architectures

use crate::traits::ToBits;

/// Architecture-specific alignment based on SIMD support
#[cfg(target_arch = "x86_64")]
pub const SIMD_ALIGN: usize = 64; // AVX-512
#[cfg(target_arch = "x86")]
pub const SIMD_ALIGN: usize = 32; // AVX2
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
pub const SIMD_ALIGN: usize = 16; // Generic SIMD

/// Error types for Vec operations
#[derive(Debug)]
pub enum BitVecError {
    InvalidAlignment(usize),
    UnsupportedInstructionSet,
    AllocationError,
}

/// Add the missing InstructionSet enum:
#[derive(Debug)]
pub enum InstructionSet {
    AVX512,
    AVX2,
    SSE,
    None,
}

/// A space-efficient vector that stores elements as packed bits.
pub struct Vec<T: crate::traits::ToBits> {
    pub data: *mut bool, // underlying raw storage
    pub len: usize, // bit length (number of used bits)
    pub bit_capacity: usize, // total capacity in bits
    pub bit_width: usize, // bits per element
    pub alignment: usize, // memory alignment
    pub marker: core::marker::PhantomData<T>,
}

impl<T: ToBits> Vec<T> {
    pub fn new(bit_width: usize, len: usize, alignment: usize) -> Self {
        Self {
            data: core::ptr::null_mut(),
            len,
            bit_capacity: 0,
            bit_width,
            alignment,
            marker: core::marker::PhantomData,
        }
    }
}

// Ensure implementations for basic methods (new, push, pop, etc.), Drop trait,
// Deref and Index trait implementations and others are provided.
