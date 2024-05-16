use playdate_sys::playdate_display;

pub struct Display {
    disp: &'static playdate_display,
}

impl Display {
    pub(crate) fn from_ptr(disp: &'static playdate_display) -> Self {
        Self { disp }
    }

    pub fn height(&self) -> u32 {
        invoke_unsafe!(self.disp.getHeight) as _
    }

    pub fn width(&self) -> u32 {
        invoke_unsafe!(self.disp.getWidth) as _
    }

    pub fn set_inverted(&self, state: InvertedState) {
        invoke_unsafe!(self.disp.setInverted, state as _)
    }

    pub fn set_mosaic(&self, x: u32, y: u32) {
        invoke_unsafe!(self.disp.setMosaic, x, y)
    }

    pub fn set_flipped(&self, x: FlipState, y: FlipState) {
        invoke_unsafe!(self.disp.setFlipped, x as _, y as _)
    }

    pub fn set_refresh_rate(&self, rate: f32) {
        invoke_unsafe!(self.disp.setRefreshRate, rate)
    }

    pub fn set_scale(&self, scale: DisplayScale) {
        invoke_unsafe!(self.disp.setScale, scale as _)
    }

    pub fn set_offset(&self, dx: i32, dy: i32) {
        invoke_unsafe!(self.disp.setOffset, dx, dy)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvertedState {
    Normal = 0,
    Inverted = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FlipState {
    Normal = 0,
    Flipped = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayScale {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
}
