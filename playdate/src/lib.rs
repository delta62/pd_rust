#![no_std]

mod display;
mod string;
mod system;

use crate::system::System;
use display::Display;
use playdate_sys::PlaydateAPI;
pub use string::Pstr;

#[repr(i32)]
pub enum FrameResult {
    NoUpdate,
    Update,
}

pub struct Playdate {
    sys: System,
    display: Display,
}

impl Playdate {
    pub fn new(ptr: *const PlaydateAPI) -> Self {
        let sys_ptr = unsafe { ptr.as_ref().unwrap().system };
        let sys = System::from_ptr(sys_ptr);

        let display_ptr = unsafe { ptr.as_ref().unwrap().display };
        let display = Display::from_ptr(display_ptr);
        Self { sys, display }
    }

    pub fn system(&self) -> &System {
        &self.sys
    }

    pub fn display(&self) -> &Display {
        &self.display
    }
}
