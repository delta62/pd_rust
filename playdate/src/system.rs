use crate::bitmap::Bitmap;
use alloc::boxed::Box;
use bitflags::bitflags;
use core::{
    ffi::{c_char, c_void, CStr},
    mem::MaybeUninit,
};
use playdate_sys::{
    PDButtons, PDButtons_kButtonA, PDButtons_kButtonB, PDButtons_kButtonDown,
    PDButtons_kButtonLeft, PDButtons_kButtonRight, PDButtons_kButtonUp,
    PDLanguage_kPDLanguageEnglish, PDLanguage_kPDLanguageJapanese, PDLanguage_kPDLanguageUnknown,
    PDMenuItem, PDPeripherals_kAccelerometer,
};

pub struct System {
    _unused: [u8; 0],
}

impl System {
    pub(crate) fn new() -> Self {
        let _unused = Default::default();
        Self { _unused }
    }

    pub fn error(&self, s: &CStr) {
        invoke_unsafe!(system.error, s.as_ptr())
    }

    pub fn log_to_console(&self, s: &CStr) {
        invoke_unsafe!(system.logToConsole, s.as_ptr())
    }

    pub fn add_menu_item<C>(&mut self, title: &CStr, callback: C) -> ButtonMenuItem
    where
        C: FnMut() + 'static,
    {
        let user_data = Box::into_raw(Box::new(callback)) as _;
        let ptr = invoke_unsafe!(
            system.addMenuItem,
            title.as_ptr(),
            Some(menu_item_callback::<C>),
            user_data
        );

        ButtonMenuItem { ptr, user_data }
    }

    pub fn add_checkmark_menu_item<C>(
        &mut self,
        title: &CStr,
        checked: Checked,
        callback: C,
    ) -> CheckmarkMenuItem
    where
        C: FnMut() + 'static,
    {
        let user_data = Box::into_raw(Box::new(callback)) as _;
        let ptr = invoke_unsafe!(
            system.addCheckmarkMenuItem,
            title.as_ptr(),
            checked as _,
            Some(menu_item_callback::<C>),
            user_data
        );

        CheckmarkMenuItem { ptr, user_data }
    }

    pub fn add_options_menu_item<C>(
        &mut self,
        title: &CStr,
        options: &[*const core::ffi::c_char],
        callback: C,
    ) -> OptionsMenuItem
    where
        C: FnMut() + 'static,
    {
        let user_data = Box::into_raw(Box::new(callback)) as _;
        let len = options.len();
        let ptr = invoke_unsafe!(
            system.addOptionsMenuItem,
            title.as_ptr(),
            options.as_ptr() as _,
            options.len() as _,
            Some(menu_item_callback::<C>),
            user_data
        );

        OptionsMenuItem {
            len,
            ptr,
            user_data,
        }
    }

    pub fn current_time_milliseconds(&self) -> u32 {
        invoke_unsafe!(system.getCurrentTimeMilliseconds)
    }

    pub fn seconds_since_epoch(&self) -> Duration {
        let mut milliseconds = 0;
        let seconds = invoke_unsafe!(system.getSecondsSinceEpoch, &mut milliseconds);
        Duration {
            seconds,
            milliseconds,
        }
    }

    pub fn reset_elapsed_time(&mut self) {
        invoke_unsafe!(system.resetElapsedTime)
    }

    pub fn elapsed_time(&self) -> f32 {
        invoke_unsafe!(system.getElapsedTime)
    }

    pub fn timezone_offset(&self) -> i32 {
        invoke_unsafe!(system.getTimezoneOffset)
    }

    pub fn convert_epoch_to_datetime(&self, epoch: u32) -> DateTime {
        let mut datetime = MaybeUninit::<DateTime>::uninit();
        invoke_unsafe!(system.convertEpochToDateTime, epoch, datetime.as_mut_ptr());
        unsafe { datetime.assume_init() }
    }

    pub fn convert_datetime_to_epoch(&self, datetime: &DateTime) -> u32 {
        invoke_unsafe!(
            system.convertDateTimeToEpoch,
            datetime as *const _ as *mut _
        )
    }

    pub fn should_display_24_hour_time(&self) -> bool {
        invoke_unsafe!(system.shouldDisplay24HourTime) == 1
    }

    pub fn flipped(&self) -> bool {
        invoke_unsafe!(system.getFlipped) == 1
    }

    pub fn reduce_flashing(&self) -> bool {
        invoke_unsafe!(system.getReduceFlashing) == 1
    }

    pub fn set_menu_image(&mut self, bitmap: &Bitmap, x_offset: i32) {
        invoke_unsafe!(system.setMenuImage, bitmap.as_mut_ptr(), x_offset)
    }

    pub unsafe fn set_serial_message_callback(&mut self, callback: extern "C" fn(*const c_char)) {
        invoke_unsafe!(system.setSerialMessageCallback, Some(callback))
    }

    pub fn draw_fps(&mut self, x: i32, y: i32) {
        invoke_unsafe!(system.drawFPS, x, y)
    }

    pub fn battery_percentage(&self) -> f32 {
        invoke_unsafe!(system.getBatteryPercentage)
    }

    pub fn battery_voltage(&self) -> f32 {
        invoke_unsafe!(system.getBatteryVoltage)
    }

    pub fn clear_icache(&mut self) {
        invoke_unsafe!(system.clearICache)
    }

    pub fn set_peripherals_enabled(&mut self, peripherals: Peripherals) {
        invoke_unsafe!(system.setPeripheralsEnabled, peripherals.bits())
    }

    pub fn accelerometer(&self) -> AccelerometerState {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        invoke_unsafe!(system.getAccelerometer, &mut x, &mut y, &mut z);

        AccelerometerState { x, y, z }
    }

    pub fn button_state(&self) -> Buttons {
        let mut current = Default::default();
        let mut pushed = Default::default();
        let mut released = Default::default();

        invoke_unsafe!(
            system.getButtonState,
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

    pub fn set_button_callback<C>(&mut self, callback: C, queue_size: i32)
    where
        C: FnMut(ButtonState, i32, u32) -> i32 + 'static,
    {
        let user_data = Box::into_raw(Box::new(callback)) as _;
        invoke_unsafe!(
            system.setButtonCallback,
            Some(c_set_button_callback::<C>),
            user_data,
            queue_size
        )
    }

    pub fn crank_angle(&self) -> f32 {
        invoke_unsafe!(system.getCrankAngle)
    }

    pub fn crank_change(&self) -> f32 {
        invoke_unsafe!(system.getCrankChange)
    }

    pub fn crank_state(&self) -> CrankState {
        let is_docked = invoke_unsafe!(system.isCrankDocked) == 1;
        return if is_docked {
            CrankState::Docked
        } else {
            CrankState::Undocked
        };
    }

    pub fn set_auto_lock_enabled(&mut self, state: AutoLockState) {
        invoke_unsafe!(system.setAutoLockDisabled, state as _)
    }

    pub fn set_crank_sounds_enabled(&mut self, state: CrankSoundState) -> CrankSoundState {
        let previous_value = invoke_unsafe!(system.setCrankSoundsDisabled, state as _);

        return if previous_value == 0 {
            CrankSoundState::Enabled
        } else {
            CrankSoundState::Disabled
        };
    }

    pub fn language(&self) -> Language {
        let lang = invoke_unsafe!(system.getLanguage);
        lang.try_into().unwrap()
    }
}

pub type DateTime = ::playdate_sys::PDDateTime;

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum Checked {
    Checked = 0,
    Unchecked = 1,
}

pub trait MenuItem {
    fn as_mut_ptr(&self) -> *mut PDMenuItem;

    fn title(&self) -> &CStr {
        let ptr = invoke_unsafe!(system.getMenuItemTitle, self.as_mut_ptr());
        unsafe { CStr::from_ptr(ptr) }
    }

    fn set_title(&mut self, title: &CStr) {
        invoke_unsafe!(system.setMenuItemTitle, self.as_mut_ptr(), title.as_ptr());
    }
}

pub struct ButtonMenuItem {
    ptr: *mut PDMenuItem,
    user_data: *mut c_void,
}

pub struct CheckmarkMenuItem {
    ptr: *mut PDMenuItem,
    user_data: *mut c_void,
}

pub struct OptionsMenuItem {
    ptr: *mut PDMenuItem,
    len: usize,
    user_data: *mut c_void,
}

impl CheckmarkMenuItem {
    pub fn value(&self) -> usize {
        invoke_unsafe!(system.getMenuItemValue, self.ptr) as usize
    }

    pub fn set_state(&mut self, state: Checked) {
        invoke_unsafe!(system.setMenuItemValue, self.ptr, state as _)
    }
}

impl OptionsMenuItem {
    pub fn value(&self) -> usize {
        invoke_unsafe!(system.getMenuItemValue, self.ptr) as usize
    }

    pub fn set_value(&mut self, index: usize) {
        if index >= self.len {
            panic!("menu item index out of bounds")
        }

        invoke_unsafe!(system.setMenuItemValue, self.ptr, index as i32)
    }
}

impl MenuItem for ButtonMenuItem {
    fn as_mut_ptr(&self) -> *mut PDMenuItem {
        self.ptr
    }
}

impl MenuItem for CheckmarkMenuItem {
    fn as_mut_ptr(&self) -> *mut PDMenuItem {
        self.ptr
    }
}

impl MenuItem for OptionsMenuItem {
    fn as_mut_ptr(&self) -> *mut PDMenuItem {
        self.ptr
    }
}

impl Drop for ButtonMenuItem {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.user_data)) };
        invoke_unsafe!(system.removeMenuItem, self.ptr)
    }
}

impl Drop for CheckmarkMenuItem {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.user_data)) };
        invoke_unsafe!(system.removeMenuItem, self.ptr)
    }
}

impl Drop for OptionsMenuItem {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.user_data)) };
        invoke_unsafe!(system.removeMenuItem, self.ptr)
    }
}

pub struct Duration {
    pub seconds: u32,
    pub milliseconds: u32,
}

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

#[derive(Debug)]
pub struct AccelerometerState {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum CrankState {
    Docked,
    Undocked,
}

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum AutoLockState {
    Enabled = 0,
    Disabled = 1,
}

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum CrankSoundState {
    Enabled = 0,
    Disabled = 1,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum Language {
    English = PDLanguage_kPDLanguageEnglish,
    Japanese = PDLanguage_kPDLanguageJapanese,
    Unknown = PDLanguage_kPDLanguageUnknown,
}

impl TryFrom<u32> for Language {
    type Error = ();

    #[allow(non_upper_case_globals)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            PDLanguage_kPDLanguageEnglish => Self::English,
            PDLanguage_kPDLanguageJapanese => Self::Japanese,
            PDLanguage_kPDLanguageUnknown => Self::Unknown,
            _ => Err(())?,
        })
    }
}

extern "C" fn menu_item_callback<F>(user_data: *mut c_void)
where
    F: FnMut(),
{
    let callback_ptr = user_data as *mut F;
    let callback = unsafe { &mut *callback_ptr };
    callback()
}

extern "C" fn c_set_button_callback<F>(
    button: PDButtons,
    down: i32,
    when: u32,
    user_data: *mut c_void,
) -> i32
where
    F: FnMut(ButtonState, i32, u32) -> i32,
{
    let callback_ptr = user_data as *mut F;
    let callback = unsafe { &mut (*callback_ptr) };
    let state = ButtonState::from_bits_retain(button);
    callback(state, down, when)
}
