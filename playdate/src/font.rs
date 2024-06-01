use crate::{
    bitmap::Bitmap,
    error::{Error, Result},
    libc,
};
use alloc::borrow::ToOwned;
use core::{
    ffi::CStr,
    marker::PhantomData,
    ptr::{null, null_mut},
};
use playdate_sys::{
    LCDBitmap, LCDFont, LCDFontGlyph, LCDFontPage, PDStringEncoding_k16BitLEEncoding,
    PDStringEncoding_kASCIIEncoding, PDStringEncoding_kUTF8Encoding,
};

pub struct Font(*mut LCDFont);

impl Font {
    pub fn new(path: &CStr) -> Result<Self> {
        let mut err = null();
        let ptr = invoke_unsafe!(graphics.loadFont, path.as_ptr(), &mut err);

        if ptr.is_null() {
            let message = unsafe { CStr::from_ptr(err) }.to_owned();
            unsafe { libc::free(err as _) };
            Err(Error { message })?;
        }

        Ok(Self(ptr))
    }

    pub fn height(&self) -> u8 {
        invoke_unsafe!(graphics.getFontHeight, self.0)
    }

    pub fn font_page<'a>(&'a self, c: u32) -> FontPage<'a> {
        let ptr = invoke_unsafe!(graphics.getFontPage, self.0, c);
        FontPage {
            lifetime: Default::default(),
            ptr,
        }
    }

    pub fn text_width(
        &self,
        text: &CStr,
        len: usize,
        encoding: TextEncoding,
        tracking: i32,
    ) -> i32 {
        invoke_unsafe!(
            graphics.getTextWidth,
            self.0,
            text.as_ptr() as _,
            len,
            encoding as _,
            tracking
        )
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut LCDFont {
        self.0
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { libc::free(self.0 as _) };
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextEncoding {
    Ascii = PDStringEncoding_kASCIIEncoding,
    Utf8 = PDStringEncoding_kUTF8Encoding,
    Le16Bit = PDStringEncoding_k16BitLEEncoding,
}

pub struct FontPage<'a> {
    lifetime: PhantomData<&'a ()>,
    ptr: *mut LCDFontPage,
}

impl<'a> FontPage<'a> {
    pub fn glyph(&'a self, c: u32) -> Glyph<'a> {
        let mut advance = 0;
        let mut bitmap = null_mut();
        let ptr = invoke_unsafe!(
            graphics.getPageGlyph,
            self.ptr,
            c,
            &mut bitmap,
            &mut advance
        );

        Glyph::new(ptr, advance, bitmap)
    }
}

pub struct Glyph<'a> {
    lifetime: PhantomData<&'a ()>,
    ptr: *mut LCDFontGlyph,
    advance: i32,
    bitmap: Bitmap,
}

impl<'a> Glyph<'a> {
    fn new(ptr: *mut LCDFontGlyph, advance: i32, bitmap: *mut LCDBitmap) -> Self {
        let bitmap = Bitmap {
            ptr: bitmap,
            mask: None,
        };

        Self {
            lifetime: Default::default(),
            ptr,
            advance,
            bitmap,
        }
    }

    pub fn advance(&self) -> i32 {
        self.advance
    }

    pub fn kerning(&self, c1: u32, c2: u32) -> i32 {
        invoke_unsafe!(graphics.getGlyphKerning, self.ptr, c1, c2)
    }

    pub fn bitmap(&self) -> &Bitmap {
        &self.bitmap
    }
}
