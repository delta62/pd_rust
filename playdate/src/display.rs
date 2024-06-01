#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScreenInversion {
    Normal = 0,
    Inverted = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScreenFlip {
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

pub struct Display {
    _unused: [u8; 0],
}

impl Display {
    pub(crate) fn new() -> Self {
        let _unused = Default::default();
        Self { _unused }
    }

    pub fn height(&self) -> i32 {
        invoke_unsafe!(display.getHeight)
    }

    pub fn width(&self) -> i32 {
        invoke_unsafe!(display.getWidth)
    }

    pub fn set_inverted(&mut self, state: ScreenInversion) {
        invoke_unsafe!(display.setInverted, state as _)
    }

    pub fn set_mosaic(&mut self, x: u32, y: u32) {
        invoke_unsafe!(display.setMosaic, x, y)
    }

    pub fn set_flipped(&mut self, x: ScreenFlip, y: ScreenFlip) {
        invoke_unsafe!(display.setFlipped, x as _, y as _)
    }

    pub fn set_refresh_rate(&mut self, rate: f32) {
        invoke_unsafe!(display.setRefreshRate, rate)
    }

    pub fn set_scale(&mut self, scale: DisplayScale) {
        invoke_unsafe!(display.setScale, scale as _)
    }

    pub fn set_offset(&mut self, dx: i32, dy: i32) {
        invoke_unsafe!(display.setOffset, dx, dy)
    }
}
