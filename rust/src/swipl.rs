#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)] // not too sure on this one..

include!(concat!(env!("OUT_DIR"), "/swipl-bindings.rs"));
