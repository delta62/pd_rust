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

use alloc::boxed::Box;
use core::{ffi::c_void, marker::PhantomData, ptr::null_mut};
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
static mut DATA: *mut c_void = null_mut();

pub const VERSION: ApiVersion = ApiVersion {
    major: 2,
    minor: 4,
    patch: 2,
};

pub trait PlaydateState {
    fn init(pd: &mut Playdate<()>) -> Self;
}

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

pub struct Playdate<T>
where
    T: 'static,
{
    unused: PhantomData<T>,
    display: Display,
    file: FileSystem,
    graphics: Graphics,
    sprite: SpriteAPI<T>,
    system: System,
}

impl<T> Playdate<T> {
    pub unsafe fn init() -> Self {
        let system = System::new();
        let display = Display::new();
        let sprite = SpriteAPI::new();
        let file = FileSystem::new();
        let graphics = Graphics::new();
        let unused = Default::default();

        Self {
            unused,
            display,
            file,
            graphics,
            sprite,
            system,
        }
    }

    pub unsafe fn new(_ptr: *mut PlaydateAPI, data: Box<T>) -> Self {
        let data_ptr = Box::into_raw(data);
        unsafe { DATA = data_ptr as _ };
        Self::init()
    }

    pub fn data(&self) -> &T {
        unsafe {
            let ptr = DATA as *mut T;
            &*ptr
        }
    }

    pub fn data_mut(&mut self) -> &mut T {
        unsafe {
            let ptr = DATA as *mut T;
            &mut *ptr
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

    pub fn sprite(&self) -> &SpriteAPI<T> {
        &self.sprite
    }

    pub fn sprite_mut(&mut self) -> &mut SpriteAPI<T> {
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
