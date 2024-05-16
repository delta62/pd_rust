use alloc::{boxed::Box, ffi::CString, vec::Vec};
use bitflags::bitflags;
use core::{
    ffi::{c_char, c_void, CStr},
    mem::MaybeUninit,
};
use playdate_sys::{
    playdate_file, FileOptions_kFileAppend, FileOptions_kFileRead, FileOptions_kFileReadData,
    FileOptions_kFileWrite, FileStat, SDFile,
};

const FS_FAILURE: i32 = -1;

pub struct PlaydateFileSystem {
    api: &'static playdate_file,
}

pub struct FileError {
    pub message: CString,
}

type Result<T> = core::result::Result<T, FileError>;

unsafe extern "C" fn list_file_callback<F>(filename: *const c_char, user_data: *mut c_void)
where
    F: FnMut(&CStr),
{
    let callback_ptr = user_data as *mut F;
    let callback = &mut *callback_ptr;
    let filename = CStr::from_ptr(filename);
    callback(filename);
}

impl PlaydateFileSystem {
    pub(crate) unsafe fn from_ptr(api: &'static playdate_file) -> Self {
        Self { api }
    }

    pub fn list_files<C>(&self, path: &CStr, callback: C, list_opts: ListOptions) -> Result<()>
    where
        C: FnMut(&CStr) + 'static,
    {
        let data = Box::into_raw(Box::new(callback));

        let result = invoke_unsafe!(
            self.api.listfiles,
            path.as_ptr(),
            Some(list_file_callback::<C>),
            data as *mut c_void,
            list_opts.into()
        );

        // listing files is done, free the given closure now
        unsafe { drop(Box::from_raw(data)) };
        self.fs_result_from_int(result)
    }

    pub fn unlink(&self, path: &CStr, recursive: UnlinkMode) -> Result<()> {
        let result = invoke_unsafe!(self.api.unlink, path.as_ptr(), recursive.into());
        self.fs_result_from_int(result)
    }

    pub fn mkdir(&self, path: &CStr) -> Result<()> {
        let result = invoke_unsafe!(self.api.mkdir, path.as_ptr());
        self.fs_result_from_int(result)
    }

    pub fn rename(&self, from: &CStr, to: &CStr) -> Result<()> {
        let result = invoke_unsafe!(self.api.rename, from.as_ptr(), to.as_ptr());
        self.fs_result_from_int(result)
    }

    pub fn stat(&self, path: &CStr) -> Result<FileStat> {
        let mut stat_result = MaybeUninit::<FileStat>::uninit();
        let result = invoke_unsafe!(self.api.stat, path.as_ptr(), stat_result.as_mut_ptr());
        self.fs_result_from_int(result)?;
        Ok(unsafe { stat_result.assume_init() })
    }

    pub fn open(&self, path: &CStr, mode: FileOptions) -> Result<File> {
        let file_ptr = invoke_unsafe!(self.api.open, path.as_ptr(), mode.bits());
        if file_ptr.is_null() {
            self.fs_result()?;
        }

        Ok(File {
            ptr: file_ptr,
            file_api: self.api,
        })
    }

    fn fs_result_from_int(&self, result: i32) -> Result<()> {
        if result != FS_FAILURE {
            Ok(())
        } else {
            self.fs_result()
        }
    }

    fn fs_result(&self) -> Result<()> {
        let ptr = invoke_unsafe!(self.api.geterr);
        let message = unsafe { CStr::from_ptr(ptr) };
        let message = CString::new(message.to_bytes()).unwrap();
        Err(FileError { message })
    }
}

pub struct File {
    file_api: &'static playdate_file,
    ptr: *mut SDFile,
}

impl File {
    pub fn flush(&self) -> Result<u32> {
        let result = invoke_unsafe!(self.file_api.flush, self.ptr);
        self.fs_result(result)?;
        Ok(result as u32)
    }

    pub fn read(&mut self, len: u32) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(len as usize);
        let result = invoke_unsafe!(self.file_api.read, self.ptr, buf.as_mut_ptr() as _, len);
        self.fs_result(result)?;
        Ok(buf)
    }

    pub fn seek(&mut self, pos: i32, whence: i32) -> Result<()> {
        let result = invoke_unsafe!(self.file_api.seek, self.ptr, pos, whence);
        self.fs_result(result)
    }

    pub fn tell(&self) -> Result<u32> {
        let result = invoke_unsafe!(self.file_api.tell, self.ptr);
        self.fs_result(result)?;
        Ok(result as u32)
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<()> {
        let result = invoke_unsafe!(
            self.file_api.write,
            self.ptr,
            bytes.as_ptr() as _,
            bytes.len() as u32
        );
        self.fs_result(result)
    }

    fn fs_result(&self, result: i32) -> Result<()> {
        if result != FS_FAILURE {
            return Ok(());
        }

        let ptr = invoke_unsafe!(self.file_api.geterr);
        let message = unsafe { CStr::from_ptr(ptr) };
        let message = CString::new(message.to_bytes()).unwrap();
        Err(FileError { message })
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let result = invoke_unsafe!(self.file_api.close, self.ptr as *mut SDFile as *mut c_void);
        // There's nothing that can be done to recover here, but getting to
        // this point indicates something significantly wrong has happened.
        // In this case, just quit.
        if result == FS_FAILURE {
            panic!("Attempted to close a file but couldn't");
        }
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
    NonRecursive,
    Recursive,
}

impl Into<i32> for UnlinkMode {
    fn into(self) -> i32 {
        match self {
            Self::Recursive => 1,
            Self::NonRecursive => 0,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListOptions {
    ShowHidden,
    HideHidden,
}

impl Into<i32> for ListOptions {
    fn into(self) -> i32 {
        match self {
            Self::ShowHidden => 1,
            Self::HideHidden => 0,
        }
    }
}
