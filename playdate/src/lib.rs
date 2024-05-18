#![no_std]

extern crate alloc;
extern crate playdate_alloc;

#[macro_use]
mod macros;

mod display;
mod error;
mod file;
mod font;
mod gfx;
pub mod libc;
mod sprite;
mod string;
mod system;

use display::Display;
use file::PlaydateFileSystem;
use playdate_sys::PlaydateAPI;

pub use font::*;
pub use gfx::*;
pub use sprite::*;
pub use system::*;

pub const VERSION: ApiVersion = ApiVersion {
    major: 2,
    minor: 4,
    patch: 2,
};

#[repr(i32)]
pub enum FrameResult {
    NoUpdate,
    Update,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ApiVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

pub struct Playdate {
    display: Display,
    file: PlaydateFileSystem,
    gfx: PlaydateGraphics,
    ptr: *const PlaydateAPI,
    sprite: PlaydateSprite,
    sys: System,
}

impl Playdate {
    pub unsafe fn new(ptr: *const PlaydateAPI) -> Self {
        let api = *ptr;
        let sys = System::from_ptr(api.system.as_ref().unwrap());
        let display = Display::from_ptr(api.display.as_ref().unwrap());
        let sprite = PlaydateSprite::from_ptr(api.sprite.as_ref().unwrap());
        let file = PlaydateFileSystem::from_ptr(api.file.as_ref().unwrap());
        let gfx = PlaydateGraphics::from_ptr(api.graphics.as_ref().unwrap());

        Self {
            display,
            file,
            gfx,
            ptr,
            sprite,
            sys,
        }
    }

    pub fn system(&self) -> &System {
        &self.sys
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn sprite(&self) -> &PlaydateSprite {
        &self.sprite
    }

    pub fn file(&self) -> &PlaydateFileSystem {
        &self.file
    }

    pub fn graphics(&self) -> &PlaydateGraphics {
        &self.gfx
    }

    pub fn as_ptr(&self) -> *const PlaydateAPI {
        self.ptr
    }
}
