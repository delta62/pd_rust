#[macro_export]
macro_rules! cstr {
    ($s:literal) => {{
        let ptr = concat!($s, "\0").as_ptr() as *const _;
        unsafe { ::playdate::Pstr::from_ptr(ptr) }
    }};
}

pub struct Pstr(*const i8);

impl Pstr {
    pub unsafe fn from_ptr(ptr: *const i8) -> Self {
        Self(ptr)
    }

    pub(crate) fn as_ptr(&self) -> *const i8 {
        self.0
    }
}
