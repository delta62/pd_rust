use crate::{
    bitmap::{Bitmap, BitmapFlip},
    error::Result,
    font::{Font, TextEncoding},
    sprite::{DrawMode, TileMode},
};
use alloc::vec::Vec;
use core::{
    ffi::CStr,
    mem::{self, ManuallyDrop},
    ptr::null_mut,
};
use playdate_sys::{
    LCDLineCapStyle_kLineCapStyleButt, LCDLineCapStyle_kLineCapStyleRound,
    LCDLineCapStyle_kLineCapStyleSquare, LCDPolygonFillRule_kPolygonFillEvenOdd,
    LCDPolygonFillRule_kPolygonFillNonZero, LCDSolidColor_kColorBlack, LCDSolidColor_kColorClear,
    LCDSolidColor_kColorWhite, LCDSolidColor_kColorXOR,
};

pub struct Graphics {
    font: Option<Font>,
    stencil: Option<Bitmap>,
    context_stack: Vec<Option<Bitmap>>,
}

impl Graphics {
    pub(crate) fn new() -> Self {
        Self {
            font: Default::default(),
            stencil: Default::default(),
            context_stack: Default::default(),
        }
    }

    pub fn new_bitmap(&self, width: i32, height: i32, bg_color: Color) -> Bitmap {
        Bitmap::new(width, height, bg_color)
    }

    pub fn load_bitmap(&self, path: &CStr) -> Result<Bitmap> {
        Bitmap::load(path)
    }

    pub fn rotated_bitmap(
        &self,
        bitmap: &Bitmap,
        rotation: f32,
        x_scale: f32,
        y_scale: f32,
    ) -> Bitmap {
        Bitmap::rotated(bitmap, rotation, x_scale, y_scale)
    }

    pub fn push_context(&mut self, context: Option<Bitmap>) {
        let bitmap_ptr = context
            .as_ref()
            .map(|bmp| bmp.as_mut_ptr())
            .unwrap_or(null_mut());
        self.context_stack.push(context);
        invoke_unsafe!(graphics.pushContext, bitmap_ptr);
    }

    pub fn pop_context(&mut self) -> Option<Bitmap> {
        invoke_unsafe!(graphics.popContext);
        self.context_stack.pop().and_then(|ctx| ctx)
    }

    pub fn set_stencil(&mut self, stencil: Option<Bitmap>) -> Option<Bitmap> {
        let stencil_ptr = stencil
            .as_ref()
            .map(|s| s.as_mut_ptr())
            .unwrap_or(null_mut());
        let last_stencil = mem::replace(&mut self.stencil, stencil);
        invoke_unsafe!(graphics.setStencil, stencil_ptr);
        last_stencil
    }

    pub fn set_stencil_image(
        &mut self,
        stencil: Option<Bitmap>,
        tile_mode: TileMode,
    ) -> Option<Bitmap> {
        let stencil_ptr = stencil
            .as_ref()
            .map(|s| s.as_mut_ptr())
            .unwrap_or(null_mut());
        let last_stencil = mem::replace(&mut self.stencil, stencil);
        invoke_unsafe!(graphics.setStencilImage, stencil_ptr, tile_mode as _);
        last_stencil
    }

    pub fn set_draw_mode(&mut self, draw_mode: DrawMode) {
        invoke_unsafe!(graphics.setDrawMode, draw_mode as _);
    }

    pub fn set_clip_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        invoke_unsafe!(graphics.setClipRect, x, y, width, height)
    }

    pub fn set_screen_clip_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        invoke_unsafe!(graphics.setScreenClipRect, x, y, width, height)
    }

    pub fn clear_clip_rect(&mut self) {
        invoke_unsafe!(graphics.clearClipRect)
    }

    pub fn set_line_cap_style(&mut self, style: LineCapStyle) {
        invoke_unsafe!(graphics.setLineCapStyle, style as _)
    }

    pub fn set_font(&mut self, font: Font) -> Option<Font> {
        let font_ptr = font.as_mut_ptr();
        let last_font = mem::replace(&mut self.font, Some(font));
        invoke_unsafe!(graphics.setFont, font_ptr);
        last_font
    }

    pub fn set_text_tracking(&mut self, tracking: i32) {
        invoke_unsafe!(graphics.setTextTracking, tracking)
    }

    pub fn set_text_leading(&mut self, leading: i32) {
        invoke_unsafe!(graphics.setTextLeading, leading)
    }

    pub fn tile_bitmap(
        &mut self,
        bitmap: &Bitmap,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        flip: BitmapFlip,
    ) {
        invoke_unsafe!(
            graphics.tileBitmap,
            bitmap.as_mut_ptr(),
            x,
            y,
            width,
            height,
            flip as _
        )
    }

    pub fn draw_text(&mut self, text: &CStr, encoding: TextEncoding, x: i32, y: i32) {
        let len = text.to_bytes().len() + 1;
        invoke_unsafe!(
            graphics.drawText,
            text.as_ptr() as _,
            len,
            encoding as _,
            x,
            y
        );
    }

    pub fn draw_ellipse(
        &mut self,
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
            graphics.drawEllipse,
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
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        start_angle: f32,
        end_angle: f32,
        color: Color,
    ) {
        invoke_unsafe!(
            graphics.fillEllipse,
            x,
            y,
            width,
            height,
            start_angle,
            end_angle,
            color as _
        )
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, width: i32, color: Color) {
        invoke_unsafe!(graphics.drawLine, x1, y1, x2, y2, width, color as _)
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        invoke_unsafe!(graphics.drawRect, x, y, width, height, color as _)
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        invoke_unsafe!(graphics.fillRect, x, y, width, height, color as _)
    }

    pub fn fill_triangle(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        color: Color,
    ) {
        invoke_unsafe!(graphics.fillTriangle, x1, y1, x2, y2, x3, y3, color as _)
    }

    pub fn fill_polygon(
        &mut self,
        num_points: i32,
        points: &[i32],
        color: Color,
        fill_rule: FillRule,
    ) {
        invoke_unsafe!(
            graphics.fillPolygon,
            num_points,
            points.as_ptr() as _,
            color as _,
            fill_rule as _
        )
    }

    pub fn clear(&mut self, color: Color) {
        invoke_unsafe!(graphics.clear, color as _)
    }

    pub fn set_background_color(&mut self, color: Color) {
        invoke_unsafe!(graphics.setBackgroundColor, color as _)
    }

    pub fn display(&mut self) {
        invoke_unsafe!(graphics.display)
    }

    pub fn with_debug_bitmap<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(&mut Bitmap),
    {
        if !function_defined!(graphics.getDebugBitmap) {
            return false;
        }

        let ptr = invoke_unsafe!(graphics.getDebugBitmap);
        let mask = None;
        let bmp = Bitmap { ptr, mask };
        let mut bmp = ManuallyDrop::new(bmp);
        f(&mut bmp);

        true
    }

    pub fn display_frame(&mut self) -> *mut u8 {
        invoke_unsafe!(graphics.getDisplayFrame)
    }

    pub fn with_display_buffer_bitmap<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Bitmap),
    {
        let ptr = invoke_unsafe!(graphics.getDisplayBufferBitmap);
        let mask = None;
        let bmp = Bitmap { ptr, mask };
        let mut bmp = ManuallyDrop::new(bmp);
        f(&mut bmp);
    }

    pub fn frame(&mut self) -> *mut u8 {
        invoke_unsafe!(graphics.getFrame)
    }

    pub fn copy_frame_buffer_bitmap(&self) -> Bitmap {
        let ptr = invoke_unsafe!(graphics.copyFrameBufferBitmap);
        let mask = None;
        Bitmap { ptr, mask }
    }

    pub fn mark_updated_rows(&mut self, start: i32, end: i32) {
        invoke_unsafe!(graphics.markUpdatedRows, start, end)
    }

    pub fn set_draw_offset(&mut self, dx: i32, dy: i32) {
        invoke_unsafe!(graphics.setDrawOffset, dx, dy)
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineCapStyle {
    Butt = LCDLineCapStyle_kLineCapStyleButt,
    Square = LCDLineCapStyle_kLineCapStyleSquare,
    Round = LCDLineCapStyle_kLineCapStyleRound,
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
