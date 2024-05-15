#![no_std]

use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
};

extern "C" {
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    pub fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

struct PlaydateAllocator;

#[global_allocator]
static ALLOCATOR: PlaydateAllocator = PlaydateAllocator {};

unsafe impl GlobalAlloc for PlaydateAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size()) as _
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        calloc(1, layout.size()) as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as _)
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        _layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        realloc(ptr as _, new_size) as _
    }
}
