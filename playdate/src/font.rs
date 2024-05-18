use crate::gfx::Bitmap;
use core::{
    ffi::CStr,
    mem::forget,
    ptr::{addr_of_mut, null_mut},
};
use playdate_alloc::libc;
use playdate_sys::{
    playdate_graphics, LCDBitmap, LCDFont, LCDFontGlyph, LCDFontPage,
    PDStringEncoding_k16BitLEEncoding, PDStringEncoding_kASCIIEncoding,
    PDStringEncoding_kUTF8Encoding,
};

pub struct FontPage {
    api: &'static playdate_graphics,
    ptr: *mut LCDFontPage,
}

impl FontPage {
    pub fn glyph(&self, c: u32) -> Glyph {
        let mut advance = 0;
        let bitmap = null_mut();
        let ptr = invoke_unsafe!(self.api.getPageGlyph, self.ptr, c, bitmap, &mut advance);

        Glyph::new(self.api, ptr, advance, bitmap)
    }
}

pub struct Font {
    api: &'static playdate_graphics,
    ptr: *mut LCDFont,
}

impl Font {
    pub(crate) fn new(api: &'static playdate_graphics, ptr: *mut LCDFont) -> Self {
        Self { api, ptr }
    }

    pub fn height(&self) -> u8 {
        invoke_unsafe!(self.api.getFontHeight, self.ptr)
    }

    pub fn font_page(&self, c: u32) -> FontPage {
        let ptr = invoke_unsafe!(self.api.getFontPage, self.ptr, c);
        FontPage { ptr, api: self.api }
    }

    pub fn text_width(
        &self,
        text: &CStr,
        len: usize,
        encoding: TextEncoding,
        tracking: i32,
    ) -> i32 {
        invoke_unsafe!(
            self.api.getTextWidth,
            self.ptr,
            text.as_ptr() as _,
            len,
            encoding as _,
            tracking
        )
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut LCDFont {
        self.ptr
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { libc::free(self.ptr as _) };
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextEncoding {
    Ascii = PDStringEncoding_kASCIIEncoding,
    Utf8 = PDStringEncoding_kUTF8Encoding,
    Le16Bit = PDStringEncoding_k16BitLEEncoding,
}

pub struct Glyph {
    api: &'static playdate_graphics,
    ptr: *mut LCDFontGlyph,
    advance: i32,
    bitmap: *mut Bitmap,
}

impl Glyph {
    fn new(
        api: &'static playdate_graphics,
        ptr: *mut LCDFontGlyph,
        advance: i32,
        bitmap: *mut *mut LCDBitmap,
    ) -> Self {
        let mut bmp = Bitmap::new(api, unsafe { *bitmap });
        let bitmap = addr_of_mut!(bmp);
        forget(bmp);

        Self {
            api,
            ptr,
            advance,
            bitmap,
        }
    }

    pub fn advance(&self) -> i32 {
        self.advance
    }

    pub fn kerning(&self, c1: u32, c2: u32) -> i32 {
        invoke_unsafe!(self.api.getGlyphKerning, self.ptr, c1, c2)
    }

    pub fn bitmap(&self) -> &Bitmap {
        unsafe { self.bitmap.as_ref().unwrap() }
    }
}
