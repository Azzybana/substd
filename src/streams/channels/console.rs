/// Checks whether the console is connected to a TTY.
///
/// In a no‑std environment this must be implemented using platform‑specific methods.
/// Replace the stub logic below with the appropriate hardware detection code.
///
/// # Externs
///   kernel32.dll - `GetStdHandle`, `GetConsoleMode`
///
/// ```
#[cfg(!(all(windows, any(target_arch = "x86_64", target_arch = "x86"))))]
pub fn is_tty() -> bool {
    // On platforms with no std, you need to integrate with your hardware driver.
    // Example (pseudo-code):
    // if uart::is_configured_for_tty() {
    //     return true;
    // }
    // return false;

    // For now, we return false as a default stub.
    false
}

use core::ffi::c_void;
#[cfg(all(windows, any(target_arch = "x86_64", target_arch = "x86")))]
use core::ptr::null_mut;

const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
const INVALID_HANDLE_VALUE: *mut c_void = -1isize as *mut c_void;

extern "system" {
    fn GetStdHandle(nStdHandle: u32) -> *mut c_void;
    fn GetConsoleMode(hConsoleHandle: *mut c_void, lpMode: *mut u32) -> i32;
}

/// Checks whether the console is connected to a TTY on Windows.
/// This function makes unsafe FFI calls to the Windows API.
#[inline]
pub fn is_tty() -> bool {
    unsafe {
        // Get the standard output handle.
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle == INVALID_HANDLE_VALUE || handle == null_mut() {
            return false;
        }
        let mut mode = 0u32;
        // If GetConsoleMode returns non-zero, it means the handle is attached to a console.
        GetConsoleMode(handle, &mut mode) != 0
    }
}
