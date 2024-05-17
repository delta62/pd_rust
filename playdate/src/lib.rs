#![no_std]

extern crate alloc;
extern crate playdate_alloc;

#[macro_use]
mod macros;

mod display;
mod error;
mod file;
mod gfx;
mod sprite;
mod string;
mod system;

use crate::system::System;
use display::Display;
use file::PlaydateFileSystem;
use gfx::PlaydateGraphics;
use playdate_sys::PlaydateAPI;
use sprite::PlaydateSprite;

pub use gfx::Color;
pub use sprite::Sprite;

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
            file,
            gfx,
            sys,
            display,
            sprite,
        }
    }

    pub fn api_version() -> ApiVersion {
        ApiVersion {
            major: 2,
            minor: 4,
            patch: 2,
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
}
