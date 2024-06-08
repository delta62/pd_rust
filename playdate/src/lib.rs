#![no_std]
#![feature(trait_upcasting)]

extern crate alloc;

#[macro_use]
mod macros;

mod bitmap;
mod display;
mod error;
mod file;
mod font;
mod gfx;
pub mod rng;
mod sprite;
mod string;
mod system;

use core::ptr::null_mut;
use libc_alloc::LibcAlloc;
use playdate_sys::PlaydateAPI;

pub use bitmap::*;
pub use display::*;
pub use file::*;
pub use font::*;
pub use gfx::*;
pub use playdate_sys::libc;
pub use sprite::*;
pub use system::*;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

pub static mut PD: *mut PlaydateAPI = null_mut();

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

pub struct Playdate {
    display: Display,
    file: FileSystem,
    graphics: Graphics,
    sprite: SpriteAPI,
    system: System,
}

impl Playdate {
    pub unsafe fn new(_ptr: *mut PlaydateAPI) -> Self {
        let system = System::new();
        let display = Display::new();
        let sprite = SpriteAPI::new();
        let file = FileSystem::new();
        let graphics = Graphics::new();

        Self {
            display,
            file,
            graphics,
            sprite,
            system,
        }
    }

    pub fn system(&self) -> &System {
        &self.system
    }

    pub fn system_mut(&mut self) -> &mut System {
        &mut self.system
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn display_mut(&mut self) -> &mut Display {
        &mut self.display
    }

    pub fn sprite(&self) -> &SpriteAPI {
        &self.sprite
    }

    pub fn sprite_mut(&mut self) -> &mut SpriteAPI {
        &mut self.sprite
    }

    pub fn file_system(&mut self) -> &FileSystem {
        &mut self.file
    }

    pub fn graphics(&self) -> &Graphics {
        &self.graphics
    }

    pub fn graphics_mut(&mut self) -> &mut Graphics {
        &mut self.graphics
    }
}
