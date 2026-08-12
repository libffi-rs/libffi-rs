#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]

pub mod src {
    pub mod s390 {
        pub mod ffi;
    }
    pub mod closures;
    pub mod java_raw_api;
    pub mod prep_cif;
    pub mod raw_api;
    pub mod tramp;
    pub mod types;
}
