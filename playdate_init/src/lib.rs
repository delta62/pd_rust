use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn pd_init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as ItemFn);
    let ident = &f.sig.ident;

    let c_init = quote! {

        #[no_mangle]
        extern "C" fn eventHandler(
            api: *const ::playdate_sys::PlaydateAPI,
            event: ::playdate_sys::PDSystemEvent,
            arg: u32,
        ) -> i32 {
            if event == ::playdate_sys::PDSystemEvent_kEventInit {
                let api_ptr = api as *mut _;
                let mut pd = unsafe { ::playdate::Playdate::new(api) };

                unsafe {
                    let api = api.as_ref().unwrap();
                    let sys = api.system.as_ref().unwrap();
                    let set_update = sys.setUpdateCallback.unwrap();
                    set_update(Some(__playdate_sys_update), api_ptr);
                };

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
            data: *mut ::core::ffi::c_void
        ) -> i32 {
            let api_pointer = data as *mut ::playdate_sys::PlaydateAPI;
            let mut pd = unsafe { ::playdate::Playdate::new(api_pointer) };
            #ident(&mut pd) as i32
        }
    };

    let expanded = quote! {
        #f
        #c_update
    };

    TokenStream::from(expanded)
}
