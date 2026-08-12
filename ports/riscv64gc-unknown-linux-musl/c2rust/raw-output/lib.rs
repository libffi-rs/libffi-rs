#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(raw_ref_op)]

#[macro_use]
extern crate c2rust_asm_casts;
extern crate f128;

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
