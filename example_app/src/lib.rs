#![allow(internal_features)]
#![feature(core_intrinsics, lang_items)]
#![no_std]

extern crate alloc;

use playdate::{cstr, FrameResult, Playdate};
use playdate_init::pd_app;

const SCREEN_WIDTH: i32 = 400;
const SCREEN_HEIGHT: i32 = 240;

const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

#[pd_app(init = "new", update = "update")]
struct Game {
    pd: Playdate,
    dx: i32,
    dy: i32,
    x: i32,
    y: i32,
}

impl Game {
    fn new(pd: Playdate) -> Self {
        pd.system().log_to_console(cstr!("hello world"));
        let dx = 1;
        let dy = 2;

        let x = (SCREEN_WIDTH - TEXT_WIDTH) / 2;
        let y = (SCREEN_HEIGHT - TEXT_HEIGHT) / 2;

        Self { pd, dx, dy, x, y }
    }

    fn update(&mut self) -> FrameResult {
        self.pd.graphics().clear(playdate::Color::White);

        self.pd.sprite().draw_sprites();
        self.pd.system().draw_fps(0, 0);
        self.pd.graphics().draw_text(
            cstr!("hello world!"),
            playdate::TextEncoding::Ascii,
            self.x,
            self.y,
        );

        self.x += self.dx;
        self.y += self.dy;

        if self.x < 0 || self.x > SCREEN_WIDTH - TEXT_WIDTH {
            self.dx = -self.dx;
        }

        if self.y < 0 || self.y > SCREEN_HEIGHT - TEXT_HEIGHT {
            self.dy = -self.dy;
        }

        FrameResult::Update
    }
}
