#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
