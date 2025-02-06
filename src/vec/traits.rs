use core::{
    default::Default,
    fmt::{ self, Debug, Formatter },
    iter::{ FromIterator, IntoIterator },
    marker::PhantomData,
    mem::size_of,
    ops::{ Deref, DerefMut, Index, IndexMut },
    ptr,
    slice,
};
use std::alloc::Global; // add this to satisfy Global

use crate::Vec;
use crate::structs::InstructionSet;

/// Trait for types that can be converted to and from bits
pub trait ToBits {
    type BitTuple: Sized;
    fn to_bits(&self) -> crate::structs::Vec<Self::BitTuple> where Self::BitTuple: ToBits;
}

/// Trait for types that can be constructed from bits
pub trait FromBits: Sized {
    type BitTuple: Sized;

    fn from_bits(bits: &[Self::BitTuple]) -> Self;
}

/// Generic SIMD operations trait
pub trait BareSimd: Sized {
    type Element;
    type Mask;

    // Vector creation
    fn splat(value: Self::Element) -> Self;
    fn zero() -> Self;

    // Basic arithmetic
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;

    // Comparison operations
    fn eq(self, rhs: Self) -> Self::Mask;
    fn ne(self, rhs: Self) -> Self::Mask;
    fn gt(self, rhs: Self) -> Self::Mask;
    fn ge(self, rhs: Self) -> Self::Mask;
    fn lt(self, rhs: Self) -> Self::Mask;
    fn le(self, rhs: Self) -> Self::Mask;

    // Bitwise operations
    fn and(self, rhs: Self) -> Self;
    fn or(self, rhs: Self) -> Self;
    fn xor(self, rhs: Self) -> Self;
    fn not(self) -> Self;

    // Math functions
    fn abs(self) -> Self;
    fn min(self, rhs: Self) -> Self;
    fn max(self, rhs: Self) -> Self;

    // Masking/blending
    fn blend(self, rhs: Self, mask: Self::Mask) -> Self;
    fn select(mask: Self::Mask, a: Self, b: Self) -> Self;
}

/// Error handling trait for unwrapping Results/Options
pub trait OrExplode<T> {
    fn or_explode(self, msg: &str) -> T;
}

// Implement OrExplode for Option and Result
impl<T> OrExplode<T> for Option<T> {
    fn or_explode(self, msg: &str) -> T {
        self.unwrap_or_else(|| unreachable!("{}", msg))
    }
}

impl<T, E> OrExplode<T> for Result<T, E> {
    fn or_explode(self, msg: &str) -> T {
        self.unwrap_or_else(|_| unreachable!("{}", msg))
    }
}

// Core trait implementations for Vec<T>
impl<T: ToBits> Default for Vec<T> {
    fn default() -> Self {
        Self::new(0, 0, 64)
    }
}

// Iterator traits
impl<T: ToBits> FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        let inst_set = InstructionSet::detect();
        let bit_width = size_of::<T>() * 8;
        let mut vec = Self::with_capacity(lower, bit_width, inst_set);
        vec.extend(iter);
        vec
    }
}

impl<T: ToBits> Extend<T> for Vec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

// Slice access traits
impl<T: ToBits> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.data as *const T, self.len / self.bit_width) }
    }
}

impl<T: ToBits> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.data as *mut T, self.len / self.bit_width) }
    }
}

// Index access traits
impl<T: ToBits> Index<usize> for Vec<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*(self.data.add(index * self.bit_width) as *const T) }
    }
}

impl<T: ToBits> IndexMut<usize> for Vec<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *(self.data.add(index * self.bit_width) as *mut T) }
    }
}

// Debug formatting
impl<T: ToBits + Debug> Debug for Vec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

// Conversion traits
impl<T: ToBits + Copy, const N: usize> From<[T; N]> for Vec<T> {
    fn from(array: [T; N]) -> Self {
        let inst_set = InstructionSet::detect();
        let bit_width = size_of::<T>() * 8;
        let mut vec = Self::with_capacity(N, bit_width, inst_set);
        for item in array {
            vec.push(item);
        }
        vec
    }
}

// Basic numeric traits
pub trait BareMath: Sized {
    fn bare_abs(self) -> Self;
    fn bare_add(self, rhs: Self) -> Self;
    fn bare_sub(self, rhs: Self) -> Self;
    fn bare_mul(self, rhs: Self) -> Self;
    fn bare_div(self, rhs: Self) -> Self;
    fn bare_rem(self, rhs: Self) -> Self;
    fn bare_neg(self) -> Self;
}

// Provide a minimal ToBits/FromBits for bool:
impl ToBits for bool {
    type BitTuple = bool;
    fn to_bits(&self) -> crate::Vec<Self::BitTuple> {
        let mut v = crate::Vec::new(1, 8, 64); // alignment value as usize
        v.push(*self);
        v
    }
}

impl FromBits for bool {
    type BitTuple = bool;
    fn from_bits(bits: &[Self::BitTuple]) -> Self {
        bits.first().copied().unwrap_or(false)
    }
}

// Implement basic numeric traits for primitive types
macro_rules! impl_numeric_tobits {
    ($($t:ty),*) => {
        $(
            impl ToBits for $t {
                type BitTuple = bool;

                fn to_bits(&self) -> Vec<Self::BitTuple> {
                    let mut bits = Vec::with_capacity(size_of::<Self>() * 8, size_of::<Self>() * 8, InstructionSet::None);
                    for i in 0..size_of::<Self>() * 8 {
                        bits.push((*self & (1 << i) as $t) != 0);
                    }
                    bits
                }
            }

            impl FromBits for $t {
                type BitTuple = bool;

                fn from_bits(bits: &[Self::BitTuple]) -> Self {
                    let mut value = 0;
                    for (i, &bit) in bits.iter().enumerate() {
                        if bit {
                            value |= 1 << i;
                        }
                    }
                    value
                }
            }
        )*
    };
}

impl_numeric_tobits!(u8, u16, u32, u64, i8, i16, i32, i64);

// In case code calls InstructionSet::detect(), define a dummy method.
impl InstructionSet {
    pub fn detect() -> Self {
        // In real code, detect features; here we fallback.
        Self::None
    }
}
