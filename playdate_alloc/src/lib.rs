#![no_std]

use core::alloc::{GlobalAlloc, Layout};

pub mod libc {
    use core::ffi::c_void;

    extern "C" {
        pub fn free(ptr: *mut c_void);
        pub(crate) fn aligned_alloc(alignment: usize, size: usize) -> *mut c_void;
    }
}

struct PlaydateAllocator;

#[global_allocator]
static ALLOCATOR: PlaydateAllocator = PlaydateAllocator {};

unsafe impl GlobalAlloc for PlaydateAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        libc::aligned_alloc(layout.align(), layout.size()) as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr as _)
    }
}
