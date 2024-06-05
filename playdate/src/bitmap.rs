use crate::{
    error::{Error, Result},
    gfx::{Color, IntRect},
    libc,
};
use alloc::{borrow::ToOwned, rc::Rc};
use core::{
    ffi::CStr,
    mem::ManuallyDrop,
    ptr::{null, null_mut},
};
use playdate_sys::{
    LCDBitmap, LCDBitmapFlip_kBitmapFlippedX, LCDBitmapFlip_kBitmapFlippedXY,
    LCDBitmapFlip_kBitmapFlippedY, LCDBitmapFlip_kBitmapUnflipped, LCDBitmapTable,
};

#[derive(Clone, Debug)]
pub struct Bitmap {
    pub(crate) ptr: *mut LCDBitmap,
    pub(crate) mask: Option<Rc<Bitmap>>,
}

impl Bitmap {
    pub fn new(width: i32, height: i32, bg_color: Color) -> Self {
        let ptr = invoke_unsafe!(graphics.newBitmap, width, height, bg_color as _);
        let mask = None;
        Self { ptr, mask }
    }

    pub fn load(path: &CStr) -> Result<Self> {
        let mut err = null();
        let ptr = invoke_unsafe!(graphics.loadBitmap, path.as_ptr(), &mut err);
        let mask = None;

        if ptr.is_null() {
            let cstr = unsafe { CStr::from_ptr(err) };
            let message = cstr.to_owned();
            unsafe { libc::free(err as _) };
            Err(Error { message })?;
        }

        Ok(Self { ptr, mask })
    }

    pub fn rotated(bitmap: &Bitmap, rotation: f32, x_scale: f32, y_scale: f32) -> Self {
        let mask = None;
        let mut allocated_size = 0;
        let ptr = invoke_unsafe!(
            graphics.rotatedBitmap,
            bitmap.as_mut_ptr(),
            rotation,
            x_scale,
            y_scale,
            &mut allocated_size
        );

        Self { ptr, mask }
    }

    pub fn load_from(&mut self, path: &CStr) -> Result<()> {
        let err = null_mut();
        invoke_unsafe!(graphics.loadIntoBitmap, path.as_ptr(), self.ptr, err);

        if !err.is_null() {
            let message = unsafe {
                let cstr = CStr::from_ptr(*err);
                let msg = cstr.to_owned();
                libc::free(*err as _);
                msg
            };

            Err(Error { message })?;
        }

        Ok(())
    }

    pub fn set_mask(&mut self, mask: Rc<Bitmap>) {
        invoke_unsafe!(graphics.setBitmapMask, self.ptr, mask.as_mut_ptr());
        self.mask = Some(mask);
    }

    pub fn clear_mask(&mut self) {
        if self.mask.is_some() {
            invoke_unsafe!(graphics.setBitmapMask, self.ptr, null_mut());
            self.mask = None;
        }
    }

    pub fn clear(&mut self, color: Color) {
        invoke_unsafe!(graphics.clearBitmap, self.ptr, color as _)
    }

    pub fn check_mask_collision(
        &self,
        x: i32,
        y: i32,
        flip: BitmapFlip,
        other: &Bitmap,
        other_x: i32,
        other_y: i32,
        other_flip: BitmapFlip,
        rect: IntRect,
    ) -> bool {
        let result = invoke_unsafe!(
            graphics.checkMaskCollision,
            self.ptr,
            x,
            y,
            flip as _,
            other.ptr,
            other_x,
            other_y,
            other_flip as _,
            rect
        );
        return result == 1;
    }

    pub fn draw(&self, x: i32, y: i32, flip: BitmapFlip) {
        invoke_unsafe!(graphics.drawBitmap, self.ptr, x, y, flip as _)
    }

    pub fn draw_scaled(&self, x: i32, y: i32, x_scale: f32, y_scale: f32) {
        invoke_unsafe!(graphics.drawScaledBitmap, self.ptr, x, y, x_scale, y_scale)
    }

    pub fn draw_rotated(
        &self,
        x: i32,
        y: i32,
        degrees: f32,
        center_x: f32,
        center_y: f32,
        x_scale: f32,
        y_scale: f32,
    ) {
        invoke_unsafe!(
            graphics.drawRotatedBitmap,
            self.ptr,
            x,
            y,
            degrees,
            center_x,
            center_y,
            x_scale,
            y_scale
        )
    }

    pub fn data(&self) -> BitmapData {
        let mut width = 0;
        let mut height = 0;
        let mut row_bytes = 0;
        let mut mask = null_mut();
        let mut data = null_mut();

        invoke_unsafe!(
            graphics.getBitmapData,
            self.ptr,
            &mut width,
            &mut height,
            &mut row_bytes,
            &mut mask,
            &mut data
        );

        BitmapData {
            width,
            height,
            row_bytes,
            mask,
            data,
        }
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut LCDBitmap {
        self.ptr
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        if let Some(mask) = self.mask.as_ref() {
            invoke_unsafe!(graphics.freeBitmap, mask.as_mut_ptr());
        }

        invoke_unsafe!(graphics.freeBitmap, self.ptr)
    }
}

pub struct BitmapData {
    pub width: i32,
    pub height: i32,
    pub row_bytes: i32,
    pub mask: *mut u8,
    pub data: *mut u8,
}

pub struct BitmapTable(*mut LCDBitmapTable);

impl BitmapTable {
    pub fn new(count: i32, width: i32, height: i32) -> Self {
        let ptr = invoke_unsafe!(graphics.newBitmapTable, count, width, height);
        Self(ptr)
    }

    pub fn load(path: &CStr) -> Result<Self> {
        let mut err = null();
        let ptr = invoke_unsafe!(graphics.loadBitmapTable, path.as_ptr(), &mut err);

        if ptr.is_null() {
            let cstr = unsafe { CStr::from_ptr(err) };
            let message = cstr.to_owned();
            unsafe { libc::free(*err as _) };
            Err(Error { message })?;
        }

        Ok(Self(ptr))
    }

    pub fn load_from(&mut self, path: &CStr) -> Result<()> {
        let mut err = null();

        invoke_unsafe!(
            graphics.loadIntoBitmapTable,
            path.as_ptr(),
            self.0,
            &mut err
        );

        if err.is_null() {
            return Ok(());
        }

        let message = unsafe { CStr::from_ptr(err) }.to_owned();
        unsafe { libc::free(err as _) };
        Err(Error { message })
    }

    pub fn with_bitmap<F>(&mut self, index: i32, mut f: F) -> bool
    where
        F: FnMut(&mut Bitmap),
    {
        let ptr = invoke_unsafe!(graphics.getTableBitmap, self.0, index);

        if ptr.is_null() {
            return false;
        }

        let mask = None;
        let bmp = Bitmap { ptr, mask };
        let mut bmp = ManuallyDrop::new(bmp);
        f(&mut bmp);
        true
    }
}

impl Drop for BitmapTable {
    fn drop(&mut self) {
        invoke_unsafe!(graphics.freeBitmapTable, self.0)
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BitmapFlip {
    Unflipped = LCDBitmapFlip_kBitmapUnflipped,
    FlippedX = LCDBitmapFlip_kBitmapFlippedX,
    FlippedY = LCDBitmapFlip_kBitmapFlippedY,
    FlippedXY = LCDBitmapFlip_kBitmapFlippedXY,
}

impl TryFrom<u32> for BitmapFlip {
    type Error = ();

    #[allow(non_upper_case_globals)]
    fn try_from(value: u32) -> core::result::Result<Self, Self::Error> {
        match value {
            LCDBitmapFlip_kBitmapFlippedX => Ok(Self::FlippedX),
            LCDBitmapFlip_kBitmapFlippedY => Ok(Self::FlippedY),
            LCDBitmapFlip_kBitmapFlippedXY => Ok(Self::FlippedXY),
            LCDBitmapFlip_kBitmapUnflipped => Ok(Self::Unflipped),
            _ => Err(()),
        }
    }
}
