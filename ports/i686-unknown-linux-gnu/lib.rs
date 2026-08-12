#![allow(dead_code, improper_ctypes, non_camel_case_types, non_snake_case)]
#![allow(
    non_upper_case_globals,
    static_mut_refs,
    unused_assignments,
    unused_mut
)]
pub mod src {
    pub mod x86 {
        pub mod ffi;
    }
    pub mod closures;
    pub mod java_raw_api;
    pub mod prep_cif;
    pub mod raw_api;
    pub mod tramp;
    pub mod types;
}
