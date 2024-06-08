macro_rules! invoke_unsafe {
    ( $api:ident . $target:ident $(, $( $param:expr ),+ )? ) => {
        unsafe {
            let api = crate::PD.as_ref().unwrap().$api.as_ref().unwrap();
            let callable = api.$target.unwrap();
            callable($( $( $param ),+ )? )
        }
    };
}

macro_rules! function_defined {
    ( $api:ident . $target:ident ) => {
        unsafe {
            let api = crate::PD.as_ref().unwrap().$api.as_ref().unwrap();
            api.$target.is_some()
        }
    };
}

#[macro_export]
macro_rules! format_string {
    ( $fmt:expr $(, $( $arg:expr ),+ )? ) => {{
        use crate::alloc::borrow::ToOwned;

        let mut outstring = ::core::ptr::null_mut();

        unsafe {
            let sys = ::playdate::PD.as_ref().unwrap().system.as_ref().unwrap();
            let fmt = sys.formatString.unwrap();

            let len = fmt(&mut outstring, $fmt.as_ptr(), $( $( $arg ),+ )? );

            if len == -1 {
                panic!("Failed to allocate a formatted string");
            }

            ::core::ffi::CStr::from_ptr(outstring).to_owned()
        }
    }};
}
