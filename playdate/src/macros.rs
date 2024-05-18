macro_rules! invoke_unsafe {
    ( $target:expr ) => {
        invoke_unsafe!($target,)
    };
    ( $target:expr, $( $( $param:expr ),+ )? ) => {
        unsafe {
            let callable = $target.unwrap();
            callable($( $( $param ),+ )? )
        }
    };
}

#[macro_export]
macro_rules! format_string {
    ( $api:expr, $fmt:expr $(, $( $arg:expr ),+ )? ) => {{
        use crate::alloc::borrow::ToOwned;

        let mut outstring = ::core::ptr::null_mut();

        unsafe {
            let api = $api.as_ptr().as_ref().unwrap();
            let sys = api.system.as_ref().unwrap();
            let fmt = sys.formatString.unwrap();

            let len = fmt(&mut outstring, $fmt, $( $( $arg ),+ )? );

            if len == -1 {
                panic!("Failed to allocate a formatted string");
            }

            ::core::ffi::CStr::from_ptr(outstring).to_owned()
        }
    }};
}
