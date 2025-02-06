#![feature(core_intrinsics, rustc_private)]
#![allow(internal_features)]
#![no_std]

#[cfg(not(windows))]
extern crate libc;
