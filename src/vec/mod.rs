#![allow(internal_features, unstable_features)]
#![feature(allocator_api, core_intrinsics, stdarch_x86_avx512)]
#![allow(unused_unsafe)]

#[macro_use]
#[allow(unused_macros)]
mod macros;

#[macro_use]
#[allow(unused_macros)]
mod utils;

mod structs;
mod traits;
mod unsafe_impls;
mod safe_impls;

pub use structs::Vec;
pub use traits::{ ToBits, FromBits, OrExplode };
