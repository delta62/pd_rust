use crate::Pstr;
use playdate_sys::playdate_json;

pub struct Json {
    ptr: *const playdate_json,
}

impl Json {
    pub(crate) fn from_ptr(ptr: *const playdate_json) -> Self {
        Self { ptr }
    }

    pub fn decode(&self, decoder: (), reader: ()) -> () {
        todo!()
    }

    pub fn decode_string(&self, decoder: (), string: Pstr) -> () {
        todo!()
    }

    unsafe fn json(&self) -> &playdate_json {
        self.ptr.as_ref().unwrap()
    }
}
