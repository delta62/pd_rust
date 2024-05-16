#[macro_export]
macro_rules! cstr {
    ($s:literal) => {{
        let ptr = concat!($s, "\0").as_ptr() as *const _;
        unsafe { ::core::ffi::CStr::from_ptr(ptr) }
    }};
}
