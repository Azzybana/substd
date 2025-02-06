pub struct Ptr<T> {
    ptr: *const T,
}

impl<T> Ptr<T> {
    /// Creates a new `Ptr` from a raw pointer.
    pub fn new(ptr: *const T) -> Self {
        Ptr { ptr }
    }

    /// Dereferences the pointer, returning a reference.
    /// # Safety
    /// This function is unsafe because it dereferences a raw pointer.
    pub unsafe fn deref(&self) -> &T {
        &*self.ptr
    }

    /// Returns the raw pointer.
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    /// Converts the pointer to mutable.
    /// # Safety
    /// This function is unsafe because it allows mutation through a raw pointer.
    pub unsafe fn as_mut_ptr(&self) -> *mut T {
        self.ptr as *mut T
    }

    /// Check if the pointer is null.
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_ptr() {
        let data = 42;
        let ptr = Ptr::new(&data as *const _);

        unsafe {
            assert_eq!(*ptr.deref(), 42);
            assert!(!ptr.is_null());
        }
    }
}
