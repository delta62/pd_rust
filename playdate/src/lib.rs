#![no_std]

extern crate alloc;
extern crate playdate_alloc;

mod display;
mod file;
mod json;
mod sprite;
mod string;
mod system;

use crate::system::System;
use display::Display;
use file::PlaydateFileSystem;
use playdate_sys::PlaydateAPI;
use sprite::PlaydateSprite;
pub use sprite::Sprite;
pub use string::Pstr;

#[repr(i32)]
pub enum FrameResult {
    NoUpdate,
    Update,
}

pub struct Playdate {
    file: PlaydateFileSystem,
    ptr: *const PlaydateAPI,
    sys: System,
    display: Display,
    sprite: PlaydateSprite,
}

impl Playdate {
    pub fn new(ptr: *const PlaydateAPI) -> Self {
        let sys_ptr = unsafe { ptr.as_ref().unwrap().system };
        let sys = System::from_ptr(sys_ptr);

        let display_ptr = unsafe { ptr.as_ref().unwrap().display };
        let display = Display::from_ptr(display_ptr);

        let sprite_ptr = unsafe { ptr.as_ref().unwrap().sprite };
        let sprite = PlaydateSprite::from_ptr(sprite_ptr);

        let file_ptr = unsafe { ptr.as_ref().unwrap().file };
        let file = PlaydateFileSystem::from_ptr(ptr, file_ptr);

        Self {
            file,
            ptr,
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

    pub(crate) fn ptr(&self) -> *const PlaydateAPI {
        self.ptr
    }
}
