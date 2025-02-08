use core::panic::PanicInfo;

#[panic_handler]
fn explode() -> ! {
    core::intrinsics::abort()
}

#[macro_export]
macro_rules! panic_or {
    ($reason:expr, $thing:expr) => {
        match unsafe { $thing } {
            Ok(val) => val,
            Err(_) => $crate::panic::explode($reason),
        }
    };
}

pub trait OrPanic<T> {
    fn or_panic(self, reason: &str) -> T;
}

impl<T, E> OrPanic<T> for Result<T, E> {
    fn or_panic(self, reason: &str) -> T {
        match self {
            Ok(val) => val,
            Err(_) => explode(reason),
        }
    }
}

impl<T> OrPanic<T> for Option<T> {
    fn or_panic(self, reason: &str) -> T {
        match self {
            Some(val) => val,
            None => explode(reason),
        }
    }
}
