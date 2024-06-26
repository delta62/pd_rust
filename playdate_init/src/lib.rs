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
    state_ident: Option<Ident>,
}

#[proc_macro_attribute]
pub fn pd_app(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AppArgs);
    let struct_meta = parse_macro_input!(item as ItemStruct);

    let struct_ident = struct_meta.ident.clone();
    let init_ident = Ident::new(&args.init_name, Span::call_site());
    let update_ident = Ident::new(&args.update_name, Span::call_site());
    let state_ident = args
        .state_name
        .as_ref()
        .map(|state_name| Ident::new(state_name, Span::call_site()));
    let args = MacroArgs {
        struct_ident,
        update_ident,
        init_ident,
        state_ident,
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
        state_ident,
        ..
    } = args;
    let state = if let Some(state_ident) = state_ident {
        quote! {
            #state_ident::init(&mut unsafe { ::playdate::Playdate::init() })
        }
    } else {
        quote! { () }
    };

    quote! {
        use playdate::PlaydateState;
        struct PlaydateAppUserData {
            api: *mut ::playdate_sys::PlaydateAPI,
            app: #struct_ident,
        }

        #[no_mangle]
        extern "C" fn eventHandler(
            api: *mut ::playdate_sys::PlaydateAPI,
            event: ::playdate_sys::PDSystemEvent,
            arg: u32,
        ) -> i32 {
            use alloc::boxed::Box;

            if event != ::playdate_sys::PDSystemEvent_kEventInit {
                return 0
            }

            unsafe { ::playdate::PD = api };

            // let state = #state_ident::init(&mut unsafe { ::playdate::Playdate::init() });
            let state = #state;
            let mut pd = unsafe { ::playdate::Playdate::new(api, Box::new(state)) };
            let app = #struct_ident::#init_ident(&mut pd);

            let app_data = Box::new(PlaydateAppUserData { api, app });
            let app_data_ptr = Box::into_raw(app_data) as *mut ::core::ffi::c_void;

            let api = unsafe { api.as_ref().unwrap() };
            let sys = unsafe { api.system.as_ref().unwrap() };
            let set_update = sys.setUpdateCallback.unwrap();
            unsafe { set_update(Some(__playdate_sys_update), app_data_ptr) };

            0
        }
    }
}

fn update(args: &MacroArgs) -> proc_macro2::TokenStream {
    let MacroArgs { update_ident, .. } = args;
    quote! {
        #[no_mangle]
        extern "C" fn __playdate_sys_update(
            data: *mut ::core::ffi::c_void
        ) -> i32 {
            let ptr = data as *mut PlaydateAppUserData;

            // let log = unsafe { ptr.as_ref().unwrap().system.as_ref().unwrap().logToConsole.unwrap() };
            // unsafe { log(cstr!("update").as_ptr()) };

            let mut app_data = unsafe { ::alloc::boxed::Box::from_raw(ptr) };
            let mut pd = unsafe { ::playdate::Playdate::init() };
            let frame_result = app_data.app.#update_ident(&mut pd);
            ::core::mem::forget(app_data);
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
        fn panic_handler(info: &::core::panic::PanicInfo) -> ! {
            use alloc::{string::String, ffi::CString};
            use core::panic::Location;

            fn print_stack_frame(location: &Location) {
                let error = unsafe { ::playdate::PD.as_ref().unwrap().system.as_ref().unwrap().error.unwrap() };
                let file = CString::new(location.file()).unwrap().into_raw();
                let line = location.line();
                let col = location.column();
                unsafe { error(b"Panic in %s:%u:%u\n\0".as_ptr() as _, file, line, col) }
            }

            fn busy_loop() -> ! {
                loop {
                    unsafe { ::playdate_sys::libc::pause() };
                }
            }

            if unsafe { ::playdate::PD.is_null() } {
                busy_loop()
            }

            if let Some(location) = info.location() {
                print_stack_frame(&location);
            }

            busy_loop()
        }
    }
}
