macro_rules! invoke_unsafe {
    ( $target:expr ) => {
        invoke_unsafe!($target,)
    };
    ( $target:expr, $( $param:expr ),* $( , )? ) => {
        unsafe {
            let callable = $target.unwrap();
            callable($( $param ),*)
        }
    };
}
