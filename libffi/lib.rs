#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(raw_ref_op)]

pub mod src {
    pub mod aarch64 {
        pub mod ffi;
    } // mod aarch64
    pub mod closures;
    pub mod java_raw_api;
    pub mod prep_cif;
    pub mod raw_api;
    pub mod tramp;
    pub mod types;
} // mod src
