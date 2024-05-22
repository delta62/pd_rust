use alloc::ffi::CString;

#[derive(Debug)]
pub struct Error {
    pub message: CString,
}

pub type Result<T> = core::result::Result<T, Error>;
