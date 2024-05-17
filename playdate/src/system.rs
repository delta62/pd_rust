use alloc::boxed::Box;
use bitflags::bitflags;
use core::{
    ffi::{c_char, c_void, CStr},
    mem::MaybeUninit,
};
use playdate_sys::{
    LCDBitmap, PDButtons_kButtonA, PDButtons_kButtonB, PDButtons_kButtonDown,
    PDButtons_kButtonLeft, PDButtons_kButtonRight, PDButtons_kButtonUp,
    PDLanguage_kPDLanguageEnglish, PDLanguage_kPDLanguageJapanese, PDLanguage_kPDLanguageUnknown,
    PDMenuItem, PDPeripherals_kAccelerometer,
};

pub struct System {
    sys_api: &'static playdate_sys::playdate_sys,
}

unsafe extern "C" fn menu_item_callback<F>(user_data: *mut c_void)
where
    F: FnMut(),
{
    let callback_ptr = user_data as *mut F;
    let callback = &mut *callback_ptr;
    callback()
}

impl System {
    pub(crate) fn from_ptr(sys_api: &'static playdate_sys::playdate_sys) -> Self {
        Self { sys_api }
    }

    pub fn error(&self, s: &CStr) {
        invoke_unsafe!(self.sys_api.error, s.as_ptr())
    }

    pub fn log_to_console(&self, s: &CStr) {
        invoke_unsafe!(self.sys_api.logToConsole, s.as_ptr())
    }

    pub fn add_menu_item<C>(&self, title: &CStr, callback: C) -> ButtonMenuItem
    where
        C: FnMut() + 'static,
    {
        let user_data = Box::into_raw(Box::new(callback)) as _;
        let sys_api = self.sys_api;
        let ptr = invoke_unsafe!(
            self.sys_api.addMenuItem,
            title.as_ptr(),
            Some(menu_item_callback::<C>),
            user_data
        );

        ButtonMenuItem {
            sys_api,
            ptr,
            user_data,
        }
    }

    pub fn add_checkmark_menu_item<C>(
        &self,
        title: &CStr,
        checked: Checked,
        callback: C,
    ) -> CheckmarkMenuItem
    where
        C: FnMut() + 'static,
    {
        let user_data = Box::into_raw(Box::new(callback)) as _;
        let sys_api = self.sys_api;
        let ptr = invoke_unsafe!(
            self.sys_api.addCheckmarkMenuItem,
            title.as_ptr(),
            checked.into(),
            Some(menu_item_callback::<C>),
            user_data
        );

        CheckmarkMenuItem {
            sys_api,
            ptr,
            user_data,
        }
    }

    pub fn add_options_menu_item<C>(
        &self,
        title: &CStr,
        options: &[*const core::ffi::c_char],
        callback: C,
    ) -> OptionsMenuItem
    where
        C: FnMut() + 'static,
    {
        let user_data = Box::into_raw(Box::new(callback)) as _;
        let sys_api = self.sys_api;
        let len = options.len();
        let ptr = invoke_unsafe!(
            self.sys_api.addOptionsMenuItem,
            title.as_ptr(),
            options.as_ptr() as _,
            options.len() as _,
            Some(menu_item_callback::<C>),
            user_data
        );

        OptionsMenuItem {
            len,
            sys_api,
            ptr,
            user_data,
        }
    }

    pub fn current_time_milliseconds(&self) -> u32 {
        invoke_unsafe!(self.sys_api.getCurrentTimeMilliseconds)
    }

    pub fn seconds_since_epoch(&self) -> Duration {
        let mut milliseconds = 0;
        let seconds = invoke_unsafe!(self.sys_api.getSecondsSinceEpoch, &mut milliseconds);
        Duration {
            seconds,
            milliseconds,
        }
    }

    pub fn reset_elapsed_time(&self) {
        invoke_unsafe!(self.sys_api.resetElapsedTime)
    }

    pub fn elapsed_time(&self) -> f32 {
        invoke_unsafe!(self.sys_api.getElapsedTime)
    }

    pub fn timezone_offset(&self) -> i32 {
        invoke_unsafe!(self.sys_api.getTimezoneOffset)
    }

    pub fn convert_epoch_to_datetime(&self, epoch: u32) -> DateTime {
        let mut datetime = MaybeUninit::<DateTime>::uninit();
        invoke_unsafe!(
            self.sys_api.convertEpochToDateTime,
            epoch,
            datetime.as_mut_ptr()
        );
        unsafe { datetime.assume_init() }
    }

    pub fn convert_datetime_to_epoch(&self, datetime: &DateTime) -> u32 {
        // Assumption: convertDateTimeToEpoch does not modify the datetime struct
        invoke_unsafe!(
            self.sys_api.convertDateTimeToEpoch,
            datetime as *const _ as *mut _
        )
    }

    pub fn should_display_24_hour_time(&self) -> bool {
        invoke_unsafe!(self.sys_api.shouldDisplay24HourTime) == 1
    }

    pub fn flipped(&self) -> bool {
        invoke_unsafe!(self.sys_api.getFlipped) == 1
    }

    pub fn reduce_flashing(&self) -> bool {
        invoke_unsafe!(self.sys_api.getReduceFlashing) == 1
    }

    // TODO
    // formatString
    // vaFormatString
    // parseString

    pub fn set_menu_image(&self, bitmap: &Bitmap, x_offset: i32) {
        invoke_unsafe!(self.sys_api.setMenuImage, bitmap.0 as *mut _, x_offset)
    }

    pub fn set_serial_message_callback(&self, callback: extern "C" fn(*const c_char)) {
        invoke_unsafe!(self.sys_api.setSerialMessageCallback, Some(callback))
    }

    pub fn draw_fps(&self, x: i32, y: i32) {
        invoke_unsafe!(self.sys_api.drawFPS, x, y)
    }

    pub fn battery_percentage(&self) -> f32 {
        invoke_unsafe!(self.sys_api.getBatteryPercentage)
    }

    pub fn battery_voltage(&self) -> f32 {
        invoke_unsafe!(self.sys_api.getBatteryVoltage)
    }

    pub fn clear_icache(&self) {
        invoke_unsafe!(self.sys_api.clearICache)
    }

    pub fn set_peripherals_enabled(&self, peripherals: Peripherals) {
        invoke_unsafe!(self.sys_api.setPeripheralsEnabled, peripherals.bits())
    }

    pub fn accelerometer(&self) -> AccelerometerState {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        invoke_unsafe!(self.sys_api.getAccelerometer, &mut x, &mut y, &mut z);

        AccelerometerState { x, y, z }
    }

    pub fn button_state(&self) -> Buttons {
        let mut current = Default::default();
        let mut pushed = Default::default();
        let mut released = Default::default();

        invoke_unsafe!(
            self.sys_api.getButtonState,
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
        invoke_unsafe!(self.sys_api.getCrankAngle)
    }

    pub fn crank_change(&self) -> f32 {
        invoke_unsafe!(self.sys_api.getCrankChange)
    }

    pub fn crank_state(&self) -> CrankState {
        let is_docked = invoke_unsafe!(self.sys_api.isCrankDocked) == 1;
        return if is_docked {
            CrankState::Docked
        } else {
            CrankState::Undocked
        };
    }

    pub fn set_auto_lock_enabled(&self, state: AutoLockState) {
        invoke_unsafe!(self.sys_api.setAutoLockDisabled, state as _)
    }

    pub fn set_crank_sounds_enabled(&self, state: CrankSoundState) -> CrankSoundState {
        let previous_value = invoke_unsafe!(self.sys_api.setCrankSoundsDisabled, state as _);

        return if previous_value == 0 {
            CrankSoundState::Enabled
        } else {
            CrankSoundState::Disabled
        };
    }

    pub fn language(&self) -> Language {
        let lang = invoke_unsafe!(self.sys_api.getLanguage);
        lang.try_into().unwrap()
    }
}

pub type DateTime = ::playdate_sys::PDDateTime;

pub enum Checked {
    Checked,
    Unchecked,
}

impl Into<i32> for Checked {
    fn into(self) -> i32 {
        match self {
            Self::Unchecked => 0,
            Self::Checked => 1,
        }
    }
}

trait MenuItem {
    fn mut_ptr(&self) -> *mut PDMenuItem;
    fn sys_api(&self) -> &'static playdate_sys::playdate_sys;

    fn title(&self) -> &CStr {
        let ptr = invoke_unsafe!(self.sys_api().getMenuItemTitle, self.mut_ptr());
        unsafe { CStr::from_ptr(ptr) }
    }

    fn set_title(&mut self, title: &CStr) {
        invoke_unsafe!(
            self.sys_api().setMenuItemTitle,
            self.mut_ptr(),
            title.as_ptr()
        );
    }
}

pub struct ButtonMenuItem {
    ptr: *mut PDMenuItem,
    sys_api: &'static playdate_sys::playdate_sys,
    user_data: *mut c_void,
}

pub struct CheckmarkMenuItem {
    ptr: *mut PDMenuItem,
    sys_api: &'static playdate_sys::playdate_sys,
    user_data: *mut c_void,
}

pub struct OptionsMenuItem {
    ptr: *mut PDMenuItem,
    len: usize,
    sys_api: &'static playdate_sys::playdate_sys,
    user_data: *mut c_void,
}

impl CheckmarkMenuItem {
    pub fn value(&self) -> usize {
        invoke_unsafe!(self.sys_api.getMenuItemValue, self.ptr) as usize
    }

    pub fn set_state(&mut self, state: Checked) {
        invoke_unsafe!(self.sys_api.setMenuItemValue, self.ptr, state.into())
    }
}

impl OptionsMenuItem {
    pub fn value(&self) -> usize {
        invoke_unsafe!(self.sys_api.getMenuItemValue, self.ptr) as usize
    }

    pub fn set_value(&mut self, index: usize) {
        if index >= self.len {
            panic!("menu item index out of bounds")
        }

        invoke_unsafe!(self.sys_api.setMenuItemValue, self.ptr, index as i32)
    }
}

impl MenuItem for ButtonMenuItem {
    fn mut_ptr(&self) -> *mut PDMenuItem {
        self.ptr
    }

    fn sys_api(&self) -> &'static playdate_sys::playdate_sys {
        self.sys_api
    }
}

impl MenuItem for CheckmarkMenuItem {
    fn mut_ptr(&self) -> *mut PDMenuItem {
        self.ptr
    }

    fn sys_api(&self) -> &'static playdate_sys::playdate_sys {
        self.sys_api
    }
}

impl MenuItem for OptionsMenuItem {
    fn mut_ptr(&self) -> *mut PDMenuItem {
        self.ptr
    }

    fn sys_api(&self) -> &'static playdate_sys::playdate_sys {
        self.sys_api
    }
}

impl Drop for ButtonMenuItem {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.user_data)) };
        invoke_unsafe!(self.sys_api.removeMenuItem, self.ptr)
    }
}

impl Drop for CheckmarkMenuItem {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.user_data)) };
        invoke_unsafe!(self.sys_api.removeMenuItem, self.ptr)
    }
}

impl Drop for OptionsMenuItem {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.user_data)) };
        invoke_unsafe!(self.sys_api.removeMenuItem, self.ptr)
    }
}

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
