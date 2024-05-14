use crate::string::Pstr;
use bitflags::bitflags;
use core::mem::MaybeUninit;
use playdate_sys::{
    LCDBitmap, PDButtons_kButtonA, PDButtons_kButtonB, PDButtons_kButtonDown,
    PDButtons_kButtonLeft, PDButtons_kButtonRight, PDButtons_kButtonUp, PDMenuItem,
    PDPeripherals_kAccelerometer,
};

macro_rules! invoke_unsafe {
    ( $self:ident, $function:ident ) => {
        invoke_unsafe!($self, $function,)
    };
    ( $self:ident, $function:ident, $( $param:expr ),* $( , )? ) => {
        unsafe {
            let callable = $self.sys().$function.unwrap();
            callable($( $param ),*)
        }
    };
}

pub struct System {
    ptr: *const playdate_sys::playdate_sys,
}

impl System {
    pub(crate) fn from_ptr(ptr: *const playdate_sys::playdate_sys) -> Self {
        Self { ptr }
    }

    pub fn error(&self, s: Pstr) {
        invoke_unsafe!(self, error, s.as_ptr())
    }

    pub fn log_to_console(&self, s: Pstr) {
        invoke_unsafe!(self, logToConsole, s.as_ptr())
    }

    pub fn add_menu_item<C, D>(&self, title: Pstr, callback: C, user_data: D) -> MenuItem
    where
        C: FnMut(D) + 'static,
    {
        todo!()
    }

    pub fn add_checkmark_menu_item<C, D>(
        &self,
        title: Pstr,
        checked: Checked,
        callback: C,
        user_data: D,
    ) -> MenuItem
    where
        C: FnMut(D),
    {
        todo!()
    }

    pub fn add_options_menu_item<C, D>(
        &self,
        title: Pstr,
        options: &[Pstr],
        callback: C,
        user_data: D,
    ) -> MenuItem
    where
        C: FnMut(D),
    {
        todo!()
    }

    pub fn remove_menu_item(&self, menu_item: MenuItem) {
        todo!()
    }

    pub fn remove_all_menu_items(&self) {
        todo!()
    }

    pub fn menu_item_title(&self, menu_item: MenuItem) -> Pstr {
        todo!()
    }

    pub fn set_menu_item_title(&self, menu_item: MenuItem, title: Pstr) {
        todo!()
    }

    pub fn menu_item_value(&self, menu_item: MenuItem) -> i32 {
        todo!()
    }

    pub fn set_menu_item_value(&self, menu_item: MenuItem, value: i32) {
        todo!()
    }

    pub fn menu_item_user_data<D>(&self, menu_item: MenuItem) -> D {
        todo!()
    }

    pub fn set_menu_item_user_data<D>(&self, menu_item: MenuItem, user_data: D) {
        todo!()
    }

    pub fn current_time_milliseconds(&self) -> u32 {
        invoke_unsafe!(self, getCurrentTimeMilliseconds)
    }

    pub fn seconds_since_epoch(&self) -> Duration {
        let mut milliseconds = 0;
        let seconds = invoke_unsafe!(self, getSecondsSinceEpoch, &mut milliseconds);
        Duration {
            seconds,
            milliseconds,
        }
    }

    pub fn reset_elapsed_time(&self) {
        invoke_unsafe!(self, resetElapsedTime)
    }

    pub fn elapsed_time(&self) -> f32 {
        invoke_unsafe!(self, getElapsedTime)
    }

    pub fn timezone_offset(&self) -> i32 {
        invoke_unsafe!(self, getTimezoneOffset)
    }

    pub fn convert_epoch_to_datetime(&self, epoch: u32) -> DateTime {
        let mut datetime = MaybeUninit::<DateTime>::uninit();
        invoke_unsafe!(self, convertEpochToDateTime, epoch, datetime.as_mut_ptr());
        unsafe { datetime.assume_init() }
    }

    pub fn convert_datetime_to_epoch(&self, datetime: &DateTime) -> u32 {
        // Assumption: convertDateTimeToEpoch does not modify the datetime struct
        invoke_unsafe!(self, convertDateTimeToEpoch, datetime as *const _ as *mut _)
    }

    pub fn should_display_24_hour_time(&self) -> bool {
        invoke_unsafe!(self, shouldDisplay24HourTime) == 1
    }

    pub fn flipped(&self) -> bool {
        invoke_unsafe!(self, getFlipped) == 1
    }

    pub fn reduce_flashing(&self) -> bool {
        invoke_unsafe!(self, getReduceFlashing) == 1
    }

    // TODO
    // formatString
    // vaFormatString
    // parseString

    pub fn set_menu_image(&self, bitmap: &Bitmap, x_offset: i32) {
        invoke_unsafe!(self, setMenuImage, bitmap.0 as *mut _, x_offset)
    }

    pub fn set_serial_message_callback<F>(&self, callback: F)
    where
        F: FnMut(Pstr),
    {
        todo!()
    }

    pub fn draw_fps(&self, x: i32, y: i32) {
        invoke_unsafe!(self, drawFPS, x, y)
    }

    pub fn battery_percentage(&self) -> f32 {
        invoke_unsafe!(self, getBatteryPercentage)
    }

    pub fn battery_voltage(&self) -> f32 {
        invoke_unsafe!(self, getBatteryVoltage)
    }

    pub fn clear_icache(&self) {
        invoke_unsafe!(self, clearICache)
    }

    pub fn set_peripherals_enabled(&self, peripherals: Peripherals) {
        invoke_unsafe!(self, setPeripheralsEnabled, peripherals.bits())
    }

    pub fn accelerometer(&self) -> AccelerometerState {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        invoke_unsafe!(self, getAccelerometer, &mut x, &mut y, &mut z);

        AccelerometerState { x, y, z }
    }

    pub fn button_state(&self) -> Buttons {
        let mut current = Default::default();
        let mut pushed = Default::default();
        let mut released = Default::default();

        invoke_unsafe!(
            self,
            getButtonState,
            &mut current,
            &mut pushed,
            &mut released
        );

        Buttons {
            current: ButtonState::from_bits_retain(current),
            pushed: ButtonState::from_bits_retain(pushed),
            released: ButtonState::from_bits_retain(released),
        }
    }

    // TODO getButtonState callback style

    pub fn crank_angle(&self) -> f32 {
        invoke_unsafe!(self, getCrankAngle)
    }

    pub fn crank_change(&self) -> f32 {
        invoke_unsafe!(self, getCrankChange)
    }

    pub fn crank_state(&self) -> CrankState {
        let is_docked = invoke_unsafe!(self, isCrankDocked) == 1;
        return if is_docked {
            CrankState::Docked
        } else {
            CrankState::Undocked
        };
    }

    pub fn set_auto_lock_enabled(&self, state: AutoLockState) {
        let disabled = state as i32;
        invoke_unsafe!(self, setAutoLockDisabled, disabled)
    }

    pub fn set_crank_sounds_enabled(&self, state: CrankSoundState) -> CrankSoundState {
        let disabled = state as i32;
        let previous_value = invoke_unsafe!(self, setCrankSoundsDisabled, disabled);

        return if previous_value == 0 {
            CrankSoundState::Enabled
        } else {
            CrankSoundState::Disabled
        };
    }

    unsafe fn sys(&self) -> &playdate_sys::playdate_sys {
        self.ptr.as_ref().unwrap()
    }
}

pub type DateTime = ::playdate_sys::PDDateTime;

#[repr(i32)]
pub enum Checked {
    Checked,
    Unchecked,
}

pub struct MenuItem(*const PDMenuItem);

pub struct Duration {
    pub seconds: u32,
    pub milliseconds: u32,
}

pub struct Bitmap(*const LCDBitmap);

bitflags! {
    pub struct Peripherals: u32 {
        const ACCELEROMETER = PDPeripherals_kAccelerometer;
    }
}

bitflags! {
    pub struct ButtonState: u32 {
        const LEFT = PDButtons_kButtonLeft;
        const RIGHT = PDButtons_kButtonRight;
        const UP = PDButtons_kButtonUp;
        const DOWN = PDButtons_kButtonDown;
        const A = PDButtons_kButtonA;
        const B = PDButtons_kButtonB;
    }
}

pub struct Buttons {
    pub pushed: ButtonState,
    pub current: ButtonState,
    pub released: ButtonState,
}

pub struct AccelerometerState {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CrankState {
    Docked,
    Undocked,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AutoLockState {
    Enabled = 0,
    Disabled = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CrankSoundState {
    Enabled = 0,
    Disabled = 1,
}
