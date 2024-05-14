use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn pd_init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as ItemFn);
    let ident = &f.sig.ident;

    let c_init = quote! {
        #[cfg(not(test))]
        #[panic_handler]
        fn panic(_info: &::core::panic::PanicInfo) -> ! {
            loop {}
        }

        #[no_mangle]
        extern "C" fn eventHandler(
            api: *const ::playdate_sys::PlaydateAPI,
            event: ::playdate_sys::PDSystemEvent,
            arg: u32,
        ) -> i32 {
            if event == ::playdate_sys::PDSystemEvent_kEventInit {
                let mut pd = ::playdate::Playdate::new(api);
                #ident(&mut pd);
            }

            0
        }
    };

    let expanded = quote! {
        #f
        #c_init
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn pd_update(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as ItemFn);
    let ident = &f.sig.ident;

    let c_update = quote! {
        #[no_mangle]
        extern "C" fn __playdate_sys_update(
            data: *const ::core::ffi::c_void
        ) -> i32 {
            let mut pd = data as *mut ::playdate::Playdate;
            let mut pd = unsafe { pd.as_mut().unwrap() };
            #ident(&mut pd) as i32
        }
    };

    let expanded = quote! {
        #f
        #c_update
    };

    TokenStream::from(expanded)
}
