#![no_std]

use playdate::{cstr, FrameResult, Playdate};
use playdate_init::{pd_init, pd_update};

#[pd_init]
fn init(pd: &mut Playdate) {
    pd.system().log_to_console(cstr!("hello world"));
}

#[pd_update]
fn update(pd: &mut Playdate) -> FrameResult {
    pd.system().draw_fps(0, 0);
    FrameResult::Update
}
