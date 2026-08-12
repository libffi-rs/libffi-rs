#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
#![allow(unused_assignments, unused_mut, unused_variables, improper_ctypes)]
#![allow(static_mut_refs, clashing_extern_declarations)]
pub mod closures;
pub mod java_raw_api;
pub mod prep_cif;
pub mod raw_api;
pub mod tramp;
pub mod types;
pub mod powerpc {
    pub mod ffi;
    pub mod ffi_linux64;
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct Float128(pub [u8; 16]);
