#![allow(internal_features)]
#![feature(core_intrinsics, lang_items)]
#![no_std]

extern crate alloc;

use playdate::{cstr, format_string, Color, FrameResult, Playdate};
use playdate_init::{pd_init, pd_update};

#[cfg(not(test))]
#[no_mangle]
#[lang = "eh_personality"]
fn rust_eh_personality() {}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &::core::panic::PanicInfo) -> ! {
    ::core::intrinsics::abort()
}

#[pd_init]
fn init(pd: &mut Playdate) {
    let s = format_string!(pd, cstr!("[%d] hello %s").as_ptr(), 42, cstr!("world"));
    pd.system().log_to_console(&s);
}

#[pd_update]
fn update(pd: &mut Playdate) -> FrameResult {
    pd.system().draw_fps(0, 0);

    pd.graphics()
        .fill_triangle(100, 200, 200, 50, 300, 200, Color::Black);

    FrameResult::Update
}
