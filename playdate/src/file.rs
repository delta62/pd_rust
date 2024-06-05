use crate::error::{Error, Result};
use alloc::{boxed::Box, ffi::CString, vec::Vec};
use bitflags::bitflags;
use core::{
    ffi::{c_char, c_void, CStr},
    mem::MaybeUninit,
};
use playdate_sys::{
    FileOptions_kFileAppend, FileOptions_kFileRead, FileOptions_kFileReadData,
    FileOptions_kFileWrite, FileStat, SDFile,
};

const FS_FAILURE: i32 = -1;

pub struct FileSystem {
    _unused: [u8; 0],
}

impl FileSystem {
    pub(crate) fn new() -> Self {
        let _unused = Default::default();
        Self { _unused }
    }

    pub fn list_files<C>(&self, path: &CStr, callback: C, hidden_files: HiddenFiles) -> Result<()>
    where
        C: FnMut(&CStr) + 'static,
    {
        let data = Box::into_raw(Box::new(callback));

        let result = invoke_unsafe!(
            file.listfiles,
            path.as_ptr(),
            Some(list_file_callback::<C>),
            data as _,
            hidden_files as _
        );

        // listing files is done, free the given closure now
        drop(unsafe { Box::from_raw(data) });
        self.fs_result_from_int(result)
    }

    pub fn unlink(&self, path: &CStr, recursive: UnlinkMode) -> Result<()> {
        let result = invoke_unsafe!(file.unlink, path.as_ptr(), recursive as _);
        self.fs_result_from_int(result)
    }

    pub fn mkdir(&self, path: &CStr) -> Result<()> {
        let result = invoke_unsafe!(file.mkdir, path.as_ptr());
        self.fs_result_from_int(result)
    }

    pub fn rename(&self, from: &CStr, to: &CStr) -> Result<()> {
        let result = invoke_unsafe!(file.rename, from.as_ptr(), to.as_ptr());
        self.fs_result_from_int(result)
    }

    pub fn stat(&self, path: &CStr) -> Result<FileStat> {
        let mut stat_result = MaybeUninit::<FileStat>::uninit();
        let result = invoke_unsafe!(file.stat, path.as_ptr(), stat_result.as_mut_ptr());
        self.fs_result_from_int(result)?;
        Ok(unsafe { stat_result.assume_init() })
    }

    pub fn open(&self, path: &CStr, mode: FileOptions) -> Result<File> {
        let file_ptr = invoke_unsafe!(file.open, path.as_ptr(), mode.bits());
        if file_ptr.is_null() {
            self.fs_fail()?;
        }

        Ok(File(file_ptr))
    }

    fn fs_result_from_int(&self, result: i32) -> Result<()> {
        if result != FS_FAILURE {
            Ok(())
        } else {
            self.fs_fail()
        }
    }

    fn fs_fail(&self) -> Result<()> {
        let ptr = invoke_unsafe!(file.geterr);
        let message = unsafe { CStr::from_ptr(ptr) };
        let message = CString::new(message.to_bytes()).unwrap();
        Err(Error { message })
    }
}

pub struct File(*mut SDFile);

impl File {
    pub fn flush(&self) -> Result<u32> {
        let result = invoke_unsafe!(file.flush, self.0);
        self.fs_result(result)?;
        Ok(result as _)
    }

    pub fn read(&mut self, len: u32) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(len as usize);
        let result = invoke_unsafe!(file.read, self.0, buf.as_mut_ptr() as _, len);
        self.fs_result(result)?;
        Ok(buf)
    }

    pub fn seek(&mut self, pos: i32, whence: i32) -> Result<()> {
        let result = invoke_unsafe!(file.seek, self.0, pos, whence);
        self.fs_result(result)
    }

    pub fn tell(&self) -> Result<u32> {
        let result = invoke_unsafe!(file.tell, self.0);
        self.fs_result(result)?;
        Ok(result as _)
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<()> {
        let result = invoke_unsafe!(file.write, self.0, bytes.as_ptr() as _, bytes.len() as _);
        self.fs_result(result)
    }

    fn fs_result(&self, result: i32) -> Result<()> {
        if result != FS_FAILURE {
            return Ok(());
        }

        let ptr = invoke_unsafe!(file.geterr);
        let message = unsafe { CStr::from_ptr(ptr) };
        let message = CString::new(message.to_bytes()).unwrap();
        Err(Error { message })
    }
}

impl Drop for File {
    fn drop(&mut self) {
        invoke_unsafe!(file.close, self.0 as _);
    }
}

bitflags! {
    pub struct FileOptions: u32 {
        const READ = FileOptions_kFileRead;
        const READ_DATA = FileOptions_kFileReadData;
        const WRITE = FileOptions_kFileWrite;
        const WRITE_DATA = FileOptions_kFileAppend;
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UnlinkMode {
    NonRecursive = 0,
    Recursive = 1,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum HiddenFiles {
    Hide = 0,
    Show = 1,
}

unsafe extern "C" fn list_file_callback<F>(filename: *const c_char, user_data: *mut c_void)
where
    F: FnMut(&CStr),
{
    let callback_ptr = user_data as *mut F;
    let callback = &mut *callback_ptr;
    let filename = CStr::from_ptr(filename);
    callback(filename);
}
