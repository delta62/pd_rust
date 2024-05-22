mod attr;

use attr::AppArgs;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemStruct};

struct MacroArgs {
    struct_ident: Ident,
    init_ident: Ident,
    update_ident: Ident,
}

#[proc_macro_attribute]
pub fn pd_app(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AppArgs);
    let struct_meta = parse_macro_input!(item as ItemStruct);

    let struct_ident = struct_meta.ident.clone();
    let init_ident = Ident::new(&args.init_name, Span::call_site());
    let update_ident = Ident::new(&args.update_name, Span::call_site());
    let args = MacroArgs {
        struct_ident,
        update_ident,
        init_ident,
    };

    let panic = panic_handler();
    let init = init(&args);
    let update = update(&args);

    let output = quote! {
        #panic
        #struct_meta
        #init
        #update
    };

    TokenStream::from(output)
}

fn init(args: &MacroArgs) -> proc_macro2::TokenStream {
    let MacroArgs {
        init_ident,
        struct_ident,
        ..
    } = args;
    quote! {
        #[no_mangle]
        unsafe extern "C" fn eventHandler(
            api: *mut ::playdate_sys::PlaydateAPI,
            event: ::playdate_sys::PDSystemEvent,
            arg: u32,
        ) -> i32 {
            use alloc::boxed::Box;

            if event != ::playdate_sys::PDSystemEvent_kEventInit {
                return 0
            }

            let mut pd = ::playdate::Playdate::new(api);
            let app = #struct_ident::#init_ident(pd);
            let app = Box::new(app);
            let ptr = Box::into_raw(app) as *mut ::core::ffi::c_void;

            let api = api.as_ref().unwrap();
            let sys = api.system.as_ref().unwrap();
            let set_update = sys.setUpdateCallback.unwrap();
            set_update(Some(__playdate_sys_update), ptr);

            0
        }
    }
}

fn update(args: &MacroArgs) -> proc_macro2::TokenStream {
    let MacroArgs {
        struct_ident,
        update_ident,
        ..
    } = args;
    quote! {
        #[no_mangle]
        unsafe extern "C" fn __playdate_sys_update(
            data: *mut ::core::ffi::c_void
        ) -> i32 {
            use alloc::boxed::Box;

            let ptr = data as *mut #struct_ident;
            let mut app = Box::from_raw(ptr);
            let frame_result = app.#update_ident();
            ::core::mem::forget(app);
            frame_result as i32
        }
    }
}

fn panic_handler() -> proc_macro2::TokenStream {
    quote! {
        #[cfg(not(test))]
        #[no_mangle]
        #[lang = "eh_personality"]
        fn rust_eh_personality() {}

        #[cfg(not(test))]
        #[panic_handler]
        fn panic_handler(_info: &::core::panic::PanicInfo) -> ! {
            ::core::intrinsics::abort()
        }
    }
}
