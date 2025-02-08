// Common file and I/O functions
extern "system" {
    // File I/O
    pub fn CreateFileA(
        lpFileName: *const u8,
        dwDesiredAccess: u32,
        dwShareMode: u32,
        lpSecurityAttributes: *mut core::ffi::c_void,
        dwCreationDisposition: u32,
        dwFlagsAndAttributes: u32,
        hTemplateFile: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;

    pub fn CreateFileW(
        lpFileName: *const u16,
        dwDesiredAccess: u32,
        dwShareMode: u32,
        lpSecurityAttributes: *mut core::ffi::c_void,
        dwCreationDisposition: u32,
        dwFlagsAndAttributes: u32,
        hTemplateFile: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;

    pub fn ReadFile(
        hFile: *mut core::ffi::c_void,
        lpBuffer: *mut core::ffi::c_void,
        nNumberOfBytesToRead: u32,
        lpNumberOfBytesRead: *mut u32,
        lpOverlapped: *mut core::ffi::c_void,
    ) -> i32;

    pub fn WriteFile(
        hFile: *mut core::ffi::c_void,
        lpBuffer: *const core::ffi::c_void,
        nNumberOfBytesToWrite: u32,
        lpNumberOfBytesWritten: *mut u32,
        lpOverlapped: *mut core::ffi::c_void,
    ) -> i32;

    pub fn CloseHandle(hObject: *mut core::ffi::c_void) -> i32;

    // Console functions
    pub fn GetStdHandle(nStdHandle: u32) -> *mut core::ffi::c_void;
    pub fn GetConsoleMode(hConsoleHandle: *mut core::ffi::c_void, lpMode: *mut u32) -> i32;
    pub fn SetConsoleMode(hConsoleHandle: *mut core::ffi::c_void, dwMode: u32) -> i32;

    // Error handling
    pub fn GetLastError() -> u32;
    pub fn SetLastError(dwErrCode: u32);

    // Timing functions
    pub fn Sleep(dwMilliseconds: u32);
    pub fn GetTickCount() -> u32;
    pub fn QueryPerformanceCounter(lpPerformanceCount: *mut i64) -> i32;
    pub fn QueryPerformanceFrequency(lpFrequency: *mut i64) -> i32;

    // Process and thread functions
    pub fn GetCurrentProcess() -> *mut core::ffi::c_void;
    pub fn GetCurrentThread() -> *mut core::ffi::c_void;
    pub fn CreateThread(
        lpThreadAttributes: *mut core::ffi::c_void,
        dwStackSize: usize,
        lpStartAddress: extern "system" fn(*mut core::ffi::c_void) -> u32,
        lpParameter: *mut core::ffi::c_void,
        dwCreationFlags: u32,
        lpThreadId: *mut u32,
    ) -> *mut core::ffi::c_void;
    pub fn WaitForSingleObject(hHandle: *mut core::ffi::c_void, dwMilliseconds: u32) -> u32;
    pub fn ExitThread(dwExitCode: u32) -> !;

    // Memory management
    pub fn VirtualAlloc(
        lpAddress: *mut core::ffi::c_void,
        dwSize: usize,
        flAllocationType: u32,
        flProtect: u32,
    ) -> *mut core::ffi::c_void;
    pub fn VirtualFree(lpAddress: *mut core::ffi::c_void, dwSize: usize, dwFreeType: u32) -> i32;

    // Synchronization primitives
    pub fn CreateMutexA(
        lpMutexAttributes: *mut core::ffi::c_void,
        bInitialOwner: i32,
        lpName: *const u8,
    ) -> *mut core::ffi::c_void;
    pub fn ReleaseMutex(hMutex: *mut core::ffi::c_void) -> i32;

    pub fn CreateEventA(
        lpEventAttributes: *mut core::ffi::c_void,
        bManualReset: i32,
        bInitialState: i32,
        lpName: *const u8,
    ) -> *mut core::ffi::c_void;
    pub fn SetEvent(hEvent: *mut core::ffi::c_void) -> i32;
    pub fn ResetEvent(hEvent: *mut core::ffi::c_void) -> i32;

    // Module handling
    pub fn GetModuleHandleA(lpModuleName: *const u8) -> *mut core::ffi::c_void;
    pub fn GetModuleHandleW(lpModuleName: *const u16) -> *mut core::ffi::c_void;

    // Debugging
    pub fn IsDebuggerPresent() -> i32;

    // String formatting (if needed)
    pub fn FormatMessageA(
        dwFlags: u32,
        lpSource: *const core::ffi::c_void,
        dwMessageId: u32,
        dwLanguageId: u32,
        lpBuffer: *mut u8,
        nSize: u32,
        Arguments: *mut core::ffi::c_void,
    ) -> u32;
}
extern "system" {
    // File I/O (already listed) …

    // Console functions (already listed) …

    // Error handling (already listed) …

    // Timing functions (already listed) …

    // Process and thread functions (already listed) …

    // Memory management via VirtualAlloc/VirtualFree (already listed) …

    // --- Additional memory management functions for custom allocators ---

    /// Retrieves a handle to the default heap of the current process.
    pub fn GetProcessHeap() -> *mut core::ffi::c_void;

    /// Allocates a block of memory from a specific heap.
    ///
    /// Use this to create custom allocation routines.
    pub fn HeapAlloc(
        hHeap: *mut core::ffi::c_void,
        dwFlags: u32,
        dwBytes: usize,
    ) -> *mut core::ffi::c_void;

    /// Reallocates a block of memory from a specific heap.
    pub fn HeapReAlloc(
        hHeap: *mut core::ffi::c_void,
        dwFlags: u32,
        lpMem: *mut core::ffi::c_void,
        dwBytes: usize,
    ) -> *mut core::ffi::c_void;

    /// Frees a block of memory allocated from a specific heap.
    pub fn HeapFree(
        hHeap: *mut core::ffi::c_void,
        dwFlags: u32,
        lpMem: *mut core::ffi::c_void,
    ) -> i32;

    /// Destroys the specified heap object.
    pub fn HeapDestroy(hHeap: *mut core::ffi::c_void) -> i32;

    // Synchronization primitives (already listed) …

    // Module handling (already listed) …

    // Debugging (already listed) …

    // String formatting (already listed) …
}
