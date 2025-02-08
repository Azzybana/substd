#![no_std]

use core::ffi::c_void;
use core::ptr::null_mut;

extern "system" {
    // Returns a handle to the default process heap.
    pub fn GetProcessHeap() -> *mut c_void;

    // Retrieves handles for all heaps in the calling process.
    // nSize is the size of the lpHeaps array.
    pub fn GetProcessHeaps(nSize: u32, lpHeaps: *mut *mut c_void) -> u32;

    // Allocates a memory block from a specified heap.
    pub fn HeapAlloc(hHeap: *mut c_void, dwFlags: u32, dwBytes: usize) -> *mut c_void;

    // Attempts to compact the specified heap.
    pub fn HeapCompact(hHeap: *mut c_void, dwFlags: u32) -> usize;

    // Creates a private heap object.
    pub fn HeapCreate(flOptions: u32, dwInitialSize: usize, dwMaximumSize: usize) -> *mut c_void;

    // Destroys a private heap object.
    pub fn HeapDestroy(hHeap: *mut c_void) -> i32;

    // Frees a memory block allocated from a heap.
    pub fn HeapFree(hHeap: *mut c_void, dwFlags: u32, lpMem: *mut c_void) -> i32;

    // Locks the specified heap.
    pub fn HeapLock(hHeap: *mut c_void) -> i32;

    // Retrieves information about the specified heap.
    pub fn HeapQueryInformation(
        HeapHandle: *mut c_void,
        HeapInformationClass: u32,
        HeapInformation: *mut c_void,
        HeapInformationLength: usize,
        ReturnLength: *mut usize,
    ) -> i32;

    // Reallocates a memory block from a heap.
    pub fn HeapReAlloc(
        hHeap: *mut c_void,
        dwFlags: u32,
        lpMem: *mut c_void,
        dwBytes: usize,
    ) -> *mut c_void;

    // Sets information for the specified heap.
    pub fn HeapSetInformation(
        HeapHandle: *mut c_void,
        HeapInformationClass: u32,
        HeapInformation: *mut c_void,
        HeapInformationLength: usize,
    ) -> i32;

    // Returns the size of a memory block allocated from a heap.
    pub fn HeapSize(hHeap: *mut c_void, dwFlags: u32, lpMem: *const c_void) -> usize;

    // Retrieves summary information for a heap.
    pub fn HeapSummary(hHeap: *mut c_void, dwFlags: u32, lpSummary: *mut HEAP_SUMMARY) -> i32;

    // Unlocks the specified heap.
    pub fn HeapUnlock(hHeap: *mut c_void) -> i32;

    // Validates the specified heap.
    pub fn HeapValidate(hHeap: *mut c_void, dwFlags: u32, lpMem: *const c_void) -> i32;

    // Iterates over the blocks in a heap.
    pub fn HeapWalk(hHeap: *mut c_void, lpEntry: *mut PROCESS_HEAP_ENTRY) -> i32;
}

// A minimal definition for the HEAP_SUMMARY structure.
#[repr(C)]
pub struct HEAP_SUMMARY {
    pub cb: u32,
    pub cbAllocated: usize,
    pub cbCommitted: usize,
    pub cbUsed: usize,
    pub cbReserved: usize,
}

// A minimal definition for the PROCESS_HEAP_ENTRY structure.
// This structure is used by HeapWalk. The actual Windows structure is a union,
// but here a simplified representation is provided.
#[repr(C)]
pub struct PROCESS_HEAP_ENTRY {
    pub lpData: *mut c_void,
    pub cbData: u32,
    pub cbOverhead: u8,
    pub iRegionIndex: u8,
    pub wFlags: u16,
    // A placeholder for the union. For most purposes, the reserved data is sufficient.
    pub Anonymous: PROCESS_HEAP_ENTRY_Unnamed,
}

#[repr(C)]
pub union PROCESS_HEAP_ENTRY_Unnamed {
    pub Reserved: [u8; 16],
    // A compact representation alternative.
    pub Compact: PROCESS_HEAP_ENTRY_Compact,
}

#[repr(C)]
pub struct PROCESS_HEAP_ENTRY_Compact {
    pub cb: u32,
    pub cbRegion: u32,
    pub wFlags: u16,
    pub wReserved: u16,
}

// --- Safe Wrappers for Heap API Endpoints ---
// Each function below is an unsafe wrapper to the corresponding Windows API endpoint.

/// Returns the handle to the default process heap.
pub unsafe fn get_process_heap() -> *mut c_void {
    GetProcessHeap()
}

/// Retrieves handles of all heaps in the calling process.
pub unsafe fn get_process_heaps(n_size: u32, heaps: *mut *mut c_void) -> u32 {
    GetProcessHeaps(n_size, heaps)
}

/// Allocates a block of memory from the specified heap.
pub unsafe fn heap_alloc(h_heap: *mut c_void, flags: u32, bytes: usize) -> *mut c_void {
    HeapAlloc(h_heap, flags, bytes)
}

/// Attempts to compact the specified heap.
pub unsafe fn heap_compact(h_heap: *mut c_void, flags: u32) -> usize {
    HeapCompact(h_heap, flags)
}

/// Creates a new private heap.
pub unsafe fn heap_create(options: u32, initial_size: usize, maximum_size: usize) -> *mut c_void {
    HeapCreate(options, initial_size, maximum_size)
}

/// Destroys the specified heap.
pub unsafe fn heap_destroy(h_heap: *mut c_void) -> i32 {
    HeapDestroy(h_heap)
}

/// Frees a previously allocated block of memory.
pub unsafe fn heap_free(h_heap: *mut c_void, flags: u32, mem: *mut c_void) -> i32 {
    HeapFree(h_heap, flags, mem)
}

/// Locks the specified heap.
pub unsafe fn heap_lock(h_heap: *mut c_void) -> i32 {
    HeapLock(h_heap)
}

/// Queries information about the specified heap.
pub unsafe fn heap_query_information(
    heap: *mut c_void,
    info_class: u32,
    buffer: *mut c_void,
    buffer_len: usize,
    return_len: *mut usize,
) -> i32 {
    HeapQueryInformation(heap, info_class, buffer, buffer_len, return_len)
}

/// Reallocates a block of memory from the specified heap.
pub unsafe fn heap_realloc(
    h_heap: *mut c_void,
    flags: u32,
    mem: *mut c_void,
    bytes: usize,
) -> *mut c_void {
    HeapReAlloc(h_heap, flags, mem, bytes)
}

/// Sets information for the specified heap.
pub unsafe fn heap_set_information(
    heap: *mut c_void,
    info_class: u32,
    info: *mut c_void,
    info_len: usize,
) -> i32 {
    HeapSetInformation(heap, info_class, info, info_len)
}

/// Returns the size of a memory block allocated from the heap.
pub unsafe fn heap_size(h_heap: *mut c_void, flags: u32, mem: *const c_void) -> usize {
    HeapSize(h_heap, flags, mem)
}

/// Retrieves summary information for the specified heap.
pub unsafe fn heap_summary(h_heap: *mut c_void, flags: u32, summary: *mut HEAP_SUMMARY) -> i32 {
    HeapSummary(h_heap, flags, summary)
}

/// Unlocks the specified heap.
pub unsafe fn heap_unlock(h_heap: *mut c_void) -> i32 {
    HeapUnlock(h_heap)
}

/// Validates the specified heap.
pub unsafe fn heap_validate(h_heap: *mut c_void, flags: u32, mem: *const c_void) -> i32 {
    HeapValidate(h_heap, flags, mem)
}

/// Iterates over the blocks in the specified heap.
pub unsafe fn heap_walk(h_heap: *mut c_void, entry: *mut PROCESS_HEAP_ENTRY) -> i32 {
    HeapWalk(h_heap, entry)
}
