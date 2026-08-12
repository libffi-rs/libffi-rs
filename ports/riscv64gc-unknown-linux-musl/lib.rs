#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

pub mod src {
    pub mod closures;
    pub mod java_raw_api;
    pub mod prep_cif;
    pub mod raw_api;
    pub mod riscv {
        pub mod ffi;
    } // mod riscv
    pub mod tramp;
    pub mod types;
} // mod src
