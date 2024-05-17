#![no_std]

use playdate::{cstr, Color, FrameResult, Playdate};
use playdate_init::{pd_init, pd_update};

#[pd_init]
fn init(pd: &mut Playdate) {
    pd.system().log_to_console(cstr!("hello world"));
}

#[pd_update]
fn update(pd: &mut Playdate) -> FrameResult {
    pd.system().draw_fps(0, 0);

    pd.graphics()
        .fill_triangle(100, 200, 200, 50, 300, 200, Color::Black);

    FrameResult::Update
}
