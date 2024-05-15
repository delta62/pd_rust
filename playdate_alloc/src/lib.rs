#![no_std]

use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
};

extern "C" {
    pub fn free(ptr: *mut c_void);
    pub fn aligned_alloc(alignment: usize, size: usize) -> *mut c_void;
}

struct PlaydateAllocator;

#[global_allocator]
static ALLOCATOR: PlaydateAllocator = PlaydateAllocator {};

unsafe impl GlobalAlloc for PlaydateAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        aligned_alloc(layout.align(), layout.size()) as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as _)
    }
}
