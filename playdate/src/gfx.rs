use crate::{
    display::FlipState,
    error::{Error, Result},
    font::{Font, TextEncoding},
    sprite::{DrawMode, TileMode},
};
use alloc::borrow::ToOwned;
use core::{ffi::CStr, ptr::null_mut};
use playdate_alloc::libc;
use playdate_sys::{
    playdate_graphics, LCDBitmap, LCDBitmapFlip_kBitmapFlippedX, LCDBitmapFlip_kBitmapFlippedXY,
    LCDBitmapFlip_kBitmapFlippedY, LCDBitmapFlip_kBitmapUnflipped, LCDBitmapTable,
    LCDLineCapStyle_kLineCapStyleButt, LCDLineCapStyle_kLineCapStyleRound,
    LCDLineCapStyle_kLineCapStyleSquare, LCDPolygonFillRule_kPolygonFillEvenOdd,
    LCDPolygonFillRule_kPolygonFillNonZero, LCDSolidColor_kColorBlack, LCDSolidColor_kColorClear,
    LCDSolidColor_kColorWhite, LCDSolidColor_kColorXOR,
};

pub struct PlaydateGraphics {
    api: &'static playdate_graphics,
}

impl PlaydateGraphics {
    pub(crate) fn from_ptr(api: &'static playdate_graphics) -> Self {
        Self { api }
    }

    pub fn with_context<F>(&mut self, target: Option<&mut Bitmap>, mut f: F)
    where
        F: FnMut(),
    {
        let target = target.map(|bmp| bmp.ptr).unwrap_or_else(|| null_mut());
        invoke_unsafe!(self.api.pushContext, target);
        f();
        invoke_unsafe!(self.api.popContext)
    }

    pub fn with_stencil<F>(&self, stencil: &mut Bitmap, mut f: F)
    where
        F: FnMut(),
    {
        invoke_unsafe!(self.api.setStencil, stencil.ptr);
        f();
        invoke_unsafe!(self.api.setStencil, null_mut())
    }

    pub fn with_stencil_image<F>(&self, stencil: &mut Bitmap, tile_mode: TileMode, mut f: F)
    where
        F: FnMut(),
    {
        invoke_unsafe!(self.api.setStencilImage, stencil.ptr, tile_mode as _);
        f();
        invoke_unsafe!(self.api.setStencil, null_mut())
    }

    pub fn set_draw_mode(&mut self, draw_mode: DrawMode) {
        invoke_unsafe!(self.api.setDrawMode, draw_mode as _)
    }

    pub fn set_clip_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        invoke_unsafe!(self.api.setClipRect, x, y, width, height)
    }

    pub fn set_screen_clip_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        invoke_unsafe!(self.api.setScreenClipRect, x, y, width, height)
    }

    pub fn clear_clip_rect(&mut self) {
        invoke_unsafe!(self.api.clearClipRect)
    }

    pub fn set_line_cap_style(&mut self, style: LineCapStyle) {
        invoke_unsafe!(self.api.setLineCapStyle, style as _)
    }

    pub fn set_font(&mut self, font: &mut Font) {
        invoke_unsafe!(self.api.setFont, font.as_mut_ptr())
    }

    pub fn set_text_tracking(&mut self, tracking: i32) {
        invoke_unsafe!(self.api.setTextTracking, tracking)
    }

    pub fn set_text_leading(&mut self, leading: i32) {
        invoke_unsafe!(self.api.setTextLeading, leading)
    }

    pub fn load_bitmap(&self, path: &CStr) -> Result<Bitmap> {
        let err = null_mut();
        let bmp = invoke_unsafe!(self.api.loadBitmap, path.as_ptr(), err);

        if bmp.is_null() {
            let cstr = unsafe { CStr::from_ptr(*err) };
            let message = cstr.to_owned();
            Err(Error { message })?;
        }

        Ok(Bitmap::new(self.api, bmp))
    }

    pub fn new_bitmap(&self, width: i32, height: i32, bg_color: Color) -> Bitmap {
        let ptr = invoke_unsafe!(self.api.newBitmap, width, height, bg_color as _);
        Bitmap::new(self.api, ptr)
    }

    pub fn tile_bitmap(
        &self,
        bitmap: &Bitmap,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        flip: FlipState,
    ) {
        invoke_unsafe!(
            self.api.tileBitmap,
            bitmap.ptr,
            x,
            y,
            width,
            height,
            flip as _
        )
    }

    pub fn rotated_bitmap(
        &self,
        bitmap: &Bitmap,
        rotation: f32,
        x_scale: f32,
        y_scale: f32,
    ) -> Bitmap {
        let mut allocated_size = 0;
        let ptr = invoke_unsafe!(
            self.api.rotatedBitmap,
            bitmap.ptr,
            rotation,
            x_scale,
            y_scale,
            &mut allocated_size
        );

        Bitmap::new(self.api, ptr)
    }

    pub fn load_bitmap_table(&self, path: &CStr) -> Result<BitmapTable> {
        let err = null_mut();
        let ptr = invoke_unsafe!(self.api.loadBitmapTable, path.as_ptr(), err);
        if ptr.is_null() {
            let message = unsafe {
                let cstr = CStr::from_ptr(*err);
                let msg = cstr.to_owned();
                libc::free(*err as _);
                msg
            };
            Err(Error { message })
        } else {
            Ok(BitmapTable { ptr, api: self.api })
        }
    }

    pub fn new_bitmap_table(&self, count: i32, width: i32, height: i32) -> BitmapTable {
        let ptr = invoke_unsafe!(self.api.newBitmapTable, count, width, height);
        BitmapTable { api: self.api, ptr }
    }

    pub fn draw_text(&self, text: &CStr, encoding: TextEncoding, x: i32, y: i32) {
        let len = text.to_bytes().len() + 1;
        invoke_unsafe!(
            self.api.drawText,
            text.as_ptr() as _,
            len,
            encoding as _,
            x,
            y
        );
    }

    pub fn load_font(&self, path: &CStr) -> Result<Font> {
        let err = null_mut();
        let ptr = invoke_unsafe!(self.api.loadFont, path.as_ptr(), err);

        if ptr.is_null() {
            let message = unsafe {
                let cstr = CStr::from_ptr(*err);
                let msg = cstr.to_owned();
                libc::free(*err as _);
                msg
            };

            Err(Error { message })?
        }

        Ok(Font::new(self.api, ptr))
    }

    pub fn draw_ellipse(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        line_width: i32,
        start_angle: f32,
        end_angle: f32,
        color: Color,
    ) {
        invoke_unsafe!(
            self.api.drawEllipse,
            x,
            y,
            width,
            height,
            line_width,
            start_angle,
            end_angle,
            color as _
        )
    }

    pub fn fill_ellipse(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        start_angle: f32,
        end_angle: f32,
        color: Color,
    ) {
        invoke_unsafe!(
            self.api.fillEllipse,
            x,
            y,
            width,
            height,
            start_angle,
            end_angle,
            color as _
        )
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32, width: i32, color: Color) {
        invoke_unsafe!(self.api.drawLine, x1, y1, x2, y2, width, color as _)
    }

    pub fn draw_rect(&self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        invoke_unsafe!(self.api.drawRect, x, y, width, height, color as _)
    }

    pub fn fill_rect(&self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        invoke_unsafe!(self.api.fillRect, x, y, width, height, color as _)
    }

    pub fn fill_triangle(
        &self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        color: Color,
    ) {
        invoke_unsafe!(self.api.fillTriangle, x1, y1, x2, y2, x3, y3, color as _)
    }

    pub fn fill_polygon(&self, num_points: i32, points: &[i32], color: Color, fill_rule: FillRule) {
        invoke_unsafe!(
            self.api.fillPolygon,
            num_points,
            points.as_ptr() as _,
            color as _,
            fill_rule as _
        )
    }

    pub fn clear(&mut self, color: Color) {
        invoke_unsafe!(self.api.clear, color as _)
    }

    pub fn set_background_color(&mut self, color: Color) {
        invoke_unsafe!(self.api.setBackgroundColor, color as _)
    }

    pub fn display(&self) {
        invoke_unsafe!(self.api.display)
    }

    pub fn get_debug_bitmap(&self) -> Option<Bitmap> {
        if self.api.getDebugBitmap.is_none() {
            return None;
        }

        let ptr = invoke_unsafe!(self.api.getDebugBitmap);
        Some(Bitmap::new(self.api, ptr))
    }

    pub fn display_frame(&self) -> *mut u8 {
        invoke_unsafe!(self.api.getDisplayFrame)
    }

    pub fn display_buffer_bitmap(&self) -> BitmapRef {
        let ptr = invoke_unsafe!(self.api.getDisplayBufferBitmap);
        BitmapRef(ptr)
    }

    pub fn frame(&self) -> *mut u8 {
        invoke_unsafe!(self.api.getFrame)
    }

    pub fn copy_frame_buffer_bitmap(&self) -> Bitmap {
        let ptr = invoke_unsafe!(self.api.copyFrameBufferBitmap);
        Bitmap::new(self.api, ptr)
    }

    pub fn mark_updated_rows(&mut self, start: i32, end: i32) {
        invoke_unsafe!(self.api.markUpdatedRows, start, end)
    }

    pub fn set_draw_offset(&mut self, dx: i32, dy: i32) {
        invoke_unsafe!(self.api.setDrawOffset, dx, dy)
    }
}

pub struct Bitmap {
    api: &'static playdate_graphics,
    ptr: *mut LCDBitmap,
}

impl Bitmap {
    pub(crate) fn new(api: &'static playdate_graphics, ptr: *mut LCDBitmap) -> Self {
        Self { api, ptr }
    }

    pub fn load(&mut self, path: &CStr) -> Result<()> {
        let err = null_mut();
        invoke_unsafe!(self.api.loadIntoBitmap, path.as_ptr(), self.ptr, err);

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

    pub fn set_mask(&mut self, mask: BitmapRef) {
        invoke_unsafe!(self.api.setBitmapMask, self.ptr, mask.0 as _);
    }

    pub fn mask(&self) -> Option<BitmapRef> {
        let ptr = invoke_unsafe!(self.api.getBitmapMask, self.ptr);
        if ptr.is_null() {
            None
        } else {
            Some(BitmapRef(ptr as _))
        }
    }

    pub fn clear(&mut self, color: Color) {
        invoke_unsafe!(self.api.clearBitmap, self.ptr, color as _)
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
            self.api.checkMaskCollision,
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
        invoke_unsafe!(self.api.drawBitmap, self.ptr, x, y, flip as _)
    }

    pub fn draw_scaled(&self, x: i32, y: i32, x_scale: f32, y_scale: f32) {
        invoke_unsafe!(self.api.drawScaledBitmap, self.ptr, x, y, x_scale, y_scale)
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
            self.api.drawRotatedBitmap,
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
            self.api.getBitmapData,
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

pub struct BitmapData {
    pub width: i32,
    pub height: i32,
    pub row_bytes: i32,
    pub mask: *mut u8,
    pub data: *mut u8,
}

pub struct BitmapTable {
    api: &'static playdate_graphics,
    ptr: *mut LCDBitmapTable,
}

impl BitmapTable {
    pub fn load(&mut self, path: &CStr) -> Result<()> {
        let err = null_mut();
        invoke_unsafe!(self.api.loadIntoBitmapTable, path.as_ptr(), self.ptr, err);

        if err.is_null() {
            let message = unsafe {
                let cstr = CStr::from_ptr(*err);
                let msg = cstr.to_owned();
                libc::free(*err as _);
                msg
            };
            Err(Error { message })?
        }
        Ok(())
    }

    pub fn bitmap(&self, index: i32) -> Option<BitmapRef> {
        let ptr = invoke_unsafe!(self.api.getTableBitmap, self.ptr, index);
        if ptr.is_null() {
            None
        } else {
            Some(BitmapRef(ptr))
        }
    }
}

impl Drop for BitmapTable {
    fn drop(&mut self) {
        invoke_unsafe!(self.api.freeBitmapTable, self.ptr)
    }
}

impl Clone for Bitmap {
    fn clone(&self) -> Self {
        let clone = invoke_unsafe!(self.api.copyBitmap, self.ptr);
        Self {
            api: self.api,
            ptr: clone,
        }
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        invoke_unsafe!(self.api.freeBitmap, self.ptr)
    }
}

pub struct BitmapRef(pub(crate) *const LCDBitmap);

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineCapStyle {
    Butt = LCDLineCapStyle_kLineCapStyleButt,
    Square = LCDLineCapStyle_kLineCapStyleSquare,
    Round = LCDLineCapStyle_kLineCapStyleRound,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BitmapFlip {
    Unflipped = LCDBitmapFlip_kBitmapUnflipped,
    FlippedX = LCDBitmapFlip_kBitmapFlippedX,
    FlippedY = LCDBitmapFlip_kBitmapFlippedY,
    FlippedXY = LCDBitmapFlip_kBitmapFlippedXY,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Black = LCDSolidColor_kColorBlack,
    White = LCDSolidColor_kColorWhite,
    Clear = LCDSolidColor_kColorClear,
    Xor = LCDSolidColor_kColorXOR,
}

pub type Rect = playdate_sys::PDRect;
pub type IntRect = playdate_sys::LCDRect;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FillRule {
    NonZero = LCDPolygonFillRule_kPolygonFillNonZero,
    EvenOdd = LCDPolygonFillRule_kPolygonFillEvenOdd,
}
