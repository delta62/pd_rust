#![no_std]

use core::ffi::CStr;
use playdate_sys::PlaydateAPI;

#[macro_export]
macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const _
    };
}

#[repr(i32)]
pub enum FrameResult {
    NoUpdate,
    Update,
}

pub struct Playdate {
    api: *const PlaydateAPI,
}

impl Playdate {
    pub fn new(api: *const PlaydateAPI) -> Self {
        Self { api }
    }

    pub fn error_and_pause(&self, s: &CStr) {
        unsafe {
            let log = self.system().error.unwrap();
            log(s.as_ptr());
        }
    }

    pub fn log(&self, s: *const i8) {
        unsafe {
            let log = self.system().logToConsole.unwrap();
            log(s);
        }
    }

    pub fn draw_fps(&self, x: i32, y: i32) {
        unsafe {
            let draw = self.system().drawFPS.unwrap();
            draw(x, y);
        }
    }

    unsafe fn system(&self) -> &::playdate_sys::playdate_sys {
        self.api.as_ref().unwrap().system.as_ref().unwrap()
    }
}
