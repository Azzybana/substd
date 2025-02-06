#![allow(dead_code)]
use core::{mem, ptr};

#[macro_export]
macro_rules! offset_of {
    ($Type:ty, $field:ident) => {{
        let uninit = core::mem::MaybeUninit::<$Type>::uninit();
        let base = uninit.as_ptr();
        unsafe { (&(*base).$field as *const _ as usize) - (base as usize) }
    }};
}

pub struct Discriminant(pub usize);

#[repr(transparent)]
pub struct ManuallyDrop<T>(pub T);

impl<T> ManuallyDrop<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        Self(val)
    }
}

#[inline]
pub fn align_of<T>() -> usize {
    mem::align_of::<T>()
}

#[inline]
pub fn align_of_val<T: ?Sized>(val: &T) -> usize {
    mem::align_of_val(val)
}

#[inline]
pub fn discriminant<T>(v: &T) -> Discriminant {
    // Use std::mem::discriminant and transmute to usize.
    let d = mem::discriminant(v);
    unsafe { Discriminant(mem::transmute(d)) }
}

#[inline]
pub fn drop<T>(_: T) {}

#[inline]
pub fn forget<T>(val: T) {
    mem::forget(val);
}

#[inline]
pub fn needs_drop<T>() -> bool {
    mem::needs_drop::<T>()
}

#[inline]
pub fn replace<T>(dest: &mut T, src: T) -> T {
    mem::replace(dest, src)
}

#[inline]
pub fn size_of<T>() -> usize {
    mem::size_of::<T>()
}

#[inline]
pub fn size_of_val<T: ?Sized>(val: &T) -> usize {
    mem::size_of_val(val)
}

#[inline]
pub fn swap<T>(a: &mut T, b: &mut T) {
    mem::swap(a, b);
}

#[inline]
pub fn take<T: Default>(dest: &mut T) -> T {
    mem::take(dest)
}

#[inline]
pub unsafe fn transmute<Src, Dst>(src: Src) -> Dst {
    mem::transmute(src)
}

#[inline]
pub unsafe fn transmute_copy<Src, Dst>(src: &Src) -> Dst {
    mem::transmute_copy(src)
}

#[inline]
pub unsafe fn zeroed<T>() -> T {
    mem::zeroed()
}
