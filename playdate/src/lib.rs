#![no_std]

extern crate alloc;
extern crate playdate_alloc;

#[macro_use]
mod macros;

mod display;
mod file;
mod sprite;
mod string;
mod system;

use crate::system::System;
use display::Display;
use file::PlaydateFileSystem;
use playdate_sys::PlaydateAPI;
use sprite::PlaydateSprite;
pub use sprite::Sprite;

#[repr(i32)]
pub enum FrameResult {
    NoUpdate,
    Update,
}

pub struct Playdate {
    display: Display,
    file: PlaydateFileSystem,
    sprite: PlaydateSprite,
    sys: System,
}

impl Playdate {
    pub unsafe fn new(ptr: *const PlaydateAPI) -> Self {
        let api = *ptr;
        let sys = System::from_ptr(api.system.as_ref().unwrap());
        let display = Display::from_ptr(api.display);
        let sprite = PlaydateSprite::from_ptr(api.sprite.as_ref().unwrap());
        let file = PlaydateFileSystem::from_ptr(api.file.as_ref().unwrap());

        Self {
            file,
            sys,
            display,
            sprite,
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
}
