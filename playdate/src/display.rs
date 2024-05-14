use playdate_sys::playdate_display;

macro_rules! invoke_unsafe {
    ( $self:ident, $function:ident ) => {
        invoke_unsafe!($self, $function,)
    };
    ( $self:ident, $function:ident, $( $param:expr ),* $( , )? ) => {
        unsafe {
            let callable = $self.display().$function.unwrap();
            callable($( $param ),*)
        }
    };
}

pub struct Display {
    ptr: *const playdate_display,
}

impl Display {
    pub(crate) fn from_ptr(ptr: *const playdate_display) -> Self {
        Self { ptr }
    }

    pub fn height(&self) -> u32 {
        invoke_unsafe!(self, getHeight) as u32
    }

    pub fn width(&self) -> u32 {
        invoke_unsafe!(self, getWidth) as u32
    }

    pub fn set_inverted(&self, state: InvertedState) {
        invoke_unsafe!(self, setInverted, state as i32)
    }

    pub fn set_mosaic(&self, x: u32, y: u32) {
        invoke_unsafe!(self, setMosaic, x, y)
    }

    pub fn set_flipped(&self, x: FlipState, y: FlipState) {
        invoke_unsafe!(self, setFlipped, x as i32, y as i32)
    }

    pub fn set_refresh_rate(&self, rate: f32) {
        invoke_unsafe!(self, setRefreshRate, rate)
    }

    pub fn set_scale(&self, scale: DisplayScale) {
        invoke_unsafe!(self, setScale, scale as u32)
    }

    pub fn set_offset(&self, dx: i32, dy: i32) {
        invoke_unsafe!(self, setOffset, dx, dy)
    }

    unsafe fn display(&self) -> &::playdate_sys::playdate_display {
        self.ptr.as_ref().unwrap()
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvertedState {
    Normal = 0,
    Inverted = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FlipState {
    Normal = 0,
    Flipped = 1,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayScale {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
}
