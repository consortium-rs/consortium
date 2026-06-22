#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]
#![cfg(target_os = "linux")]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
