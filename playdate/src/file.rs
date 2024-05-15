use crate::Pstr;
use alloc::vec::Vec;
use bitflags::bitflags;
use core::{ffi::c_void, mem::MaybeUninit};
use playdate_sys::{
    playdate_file, FileOptions_kFileAppend, FileOptions_kFileRead, FileOptions_kFileReadData,
    FileOptions_kFileWrite, FileStat, PlaydateAPI, SDFile,
};

macro_rules! invoke_unsafe {
    ( $self:ident, $function:ident ) => {
        invoke_unsafe!($self, $function,)
    };
    ( $self:ident, $function:ident, $( $param:expr ),* $( , )? ) => {
        unsafe {
            let callable = $self.file().$function.unwrap();
            callable($( $param ),*)
        }
    };
}

pub struct PlaydateFileSystem {
    pd: *const PlaydateAPI,
    ptr: *const playdate_file,
}

pub struct FileError {
    pub message: Pstr,
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UnlinkMode {
    NonRecursive,
    Recursive,
}

type Result<T> = core::result::Result<T, FileError>;

impl PlaydateFileSystem {
    pub(crate) fn from_ptr(pd: *const PlaydateAPI, ptr: *const playdate_file) -> Self {
        Self { pd, ptr }
    }

    // TODO listFiles

    pub fn unlink(&self, path: Pstr, recursive: UnlinkMode) -> Result<()> {
        let result = invoke_unsafe!(self, unlink, path.as_ptr(), recursive as i32);
        self.fs_result(result)
    }

    pub fn mkdir(&self, path: Pstr) -> Result<()> {
        let result = invoke_unsafe!(self, mkdir, path.as_ptr());
        self.fs_result(result)
    }

    pub fn rename(&self, from: Pstr, to: Pstr) -> Result<()> {
        let result = invoke_unsafe!(self, rename, from.as_ptr(), to.as_ptr());
        self.fs_result(result)
    }

    pub fn stat(&self, path: Pstr) -> Result<Stat> {
        let mut stat_result = MaybeUninit::<FileStat>::uninit();
        let result = invoke_unsafe!(self, stat, path.as_ptr(), stat_result.as_mut_ptr());
        self.fs_result(result)?;
        let stat = unsafe { stat_result.assume_init() };

        Ok(Stat {
            is_dir: stat.isdir != 0,
            size: stat.size,
            mod_year: stat.m_year,
            mod_month: stat.m_month,
            mod_day: stat.m_day,
            mod_hour: stat.m_hour,
            mod_minute: stat.m_minute,
            mod_second: stat.m_second,
        })
    }

    pub fn open(&self, path: Pstr, mode: FileOptions) -> Result<File> {
        let file_ptr = invoke_unsafe!(self, open, path.as_ptr(), mode.bits());
        if file_ptr.is_null() {
            let ptr = invoke_unsafe!(self, geterr);
            let message = unsafe { Pstr::from_ptr(ptr) };
            Err(FileError { message })
        } else {
            let file = File {
                ptr: self.ptr as *const _,
                pd: self.pd,
            };
            Ok(file)
        }
    }

    fn fs_result(&self, result: i32) -> Result<()> {
        if result == -1 {
            let ptr = invoke_unsafe!(self, geterr);
            let message = unsafe { Pstr::from_ptr(ptr) };
            Err(FileError { message })
        } else {
            Ok(())
        }
    }

    unsafe fn file(&self) -> &playdate_file {
        self.ptr.as_ref().unwrap()
    }
}

#[derive(Clone)]
pub struct Stat {
    pub is_dir: bool,
    pub size: u32,
    pub mod_year: i32,
    pub mod_month: i32,
    pub mod_day: i32,
    pub mod_hour: i32,
    pub mod_minute: i32,
    pub mod_second: i32,
}

// playdate->file->open
pub struct File {
    pd: *const PlaydateAPI,
    ptr: *const SDFile,
}

impl File {
    pub fn flush(&self) -> Result<i32> {
        unsafe {
            let result = self.file().flush.unwrap()(self.ptr as *mut _);
            if result == -1 {
                let message_ptr = self.file().geterr.unwrap()();
                let message = Pstr::from_ptr(message_ptr);
                Err(FileError { message })
            } else {
                Ok(result)
            }
        }
    }

    pub fn read(&mut self, len: u32) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(len as usize);
        unsafe {
            let result =
                self.file().read.unwrap()(self.ptr as *mut _, buf.as_mut_ptr() as *mut _, len);
            if result == -1 {
                let message_ptr = self.file().geterr.unwrap()();
                let message = Pstr::from_ptr(message_ptr);
                Err(FileError { message })
            } else {
                Ok(buf)
            }
        }
    }

    pub fn seek(&mut self, pos: i32, whence: i32) -> Result<()> {
        unsafe {
            let result = self.file().seek.unwrap()(self.ptr as *mut _, pos, whence);
            if result == -1 {
                let message_ptr = self.file().geterr.unwrap()();
                let message = Pstr::from_ptr(message_ptr);
                Err(FileError { message })
            } else {
                Ok(())
            }
        }
    }

    pub fn tell(&self) -> Result<i32> {
        unsafe {
            let result = self.file().tell.unwrap()(self.ptr as *mut _);
            if result == -1 {
                let message_ptr = self.file().geterr.unwrap()();
                let message = Pstr::from_ptr(message_ptr);
                Err(FileError { message })
            } else {
                Ok(result)
            }
        }
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<()> {
        unsafe {
            let result = self.file().write.unwrap()(
                self.ptr as *mut _,
                bytes.as_ptr() as *const _,
                bytes.len() as u32,
            );

            if result == -1 {
                let message_ptr = self.file().geterr.unwrap()();
                let message = Pstr::from_ptr(message_ptr);
                Err(FileError { message })
            } else {
                Ok(())
            }
        }
    }

    unsafe fn file(&self) -> &playdate_file {
        self.pd.as_ref().unwrap().file.as_ref().unwrap()
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            let file_api = self.pd.as_ref().unwrap().file.as_ref().unwrap();
            let close = file_api.close.unwrap();
            let result = close(self.ptr as *mut SDFile as *mut c_void);

            // There's nothing that can be done to recover here, but getting to
            // this point indicates something significantly wrong has happened.
            // In this case, just quit.
            if result == -1 {
                panic!("Attempted to close a file but couldn't");
            }
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
