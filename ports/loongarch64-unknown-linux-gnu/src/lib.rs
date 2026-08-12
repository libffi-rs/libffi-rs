#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_unsafe)]
#![allow(unused_must_use)]
#![allow(clashing_extern_declarations)]

#[cfg(not(all(target_arch = "loongarch64", target_os = "linux", target_env = "gnu")))]
compile_error!("libffi-loongarch64-c2rust only supports loongarch64-unknown-linux-gnu");

pub mod loongarch {
    pub mod ffi;
}
pub mod closures;
pub mod java_raw_api;
pub mod prep_cif;
pub mod raw_api;
pub mod tramp;
pub mod types;

pub use types::{ffi_cif, ffi_closure, ffi_type};
