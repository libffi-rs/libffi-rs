#![allow(clippy::all)]
#![allow(dead_code, improper_ctypes, non_camel_case_types, non_snake_case)]
#![allow(
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_variables
)]
#![allow(static_mut_refs, unexpected_cfgs)]

pub mod arm {
    pub mod ffi;
}
pub mod closures;
pub mod java_raw_api;
pub mod prep_cif;
pub mod raw_api;
pub mod tramp;
pub mod types;

mod atomics {
    use core::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

    pub trait Primitive: Copy {
        unsafe fn load_relaxed(p: *mut Self) -> Self;
        unsafe fn load_acquire(p: *mut Self) -> Self;
        unsafe fn xchg_acquire(p: *mut Self, v: Self) -> Self;
        unsafe fn store_release(p: *mut Self, v: Self);
    }
    impl Primitive for i32 {
        unsafe fn load_relaxed(p: *mut Self) -> Self {
            (&*(p.cast::<AtomicI32>())).load(Ordering::Relaxed)
        }
        unsafe fn load_acquire(p: *mut Self) -> Self {
            (&*(p.cast::<AtomicI32>())).load(Ordering::Acquire)
        }
        unsafe fn xchg_acquire(p: *mut Self, v: Self) -> Self {
            (&*(p.cast::<AtomicI32>())).swap(v, Ordering::Acquire)
        }
        unsafe fn store_release(p: *mut Self, v: Self) {
            (&*(p.cast::<AtomicI32>())).store(v, Ordering::Release)
        }
    }
    impl Primitive for usize {
        unsafe fn load_relaxed(p: *mut Self) -> Self {
            (&*(p.cast::<AtomicUsize>())).load(Ordering::Relaxed)
        }
        unsafe fn load_acquire(p: *mut Self) -> Self {
            (&*(p.cast::<AtomicUsize>())).load(Ordering::Acquire)
        }
        unsafe fn xchg_acquire(p: *mut Self, v: Self) -> Self {
            (&*(p.cast::<AtomicUsize>())).swap(v, Ordering::Acquire)
        }
        unsafe fn store_release(p: *mut Self, v: Self) {
            (&*(p.cast::<AtomicUsize>())).store(v, Ordering::Release)
        }
    }
    pub unsafe fn atomic_load_relaxed<T: Primitive>(p: *mut T) -> T {
        T::load_relaxed(p)
    }
    pub unsafe fn atomic_load_acquire<T: Primitive>(p: *mut T) -> T {
        T::load_acquire(p)
    }
    pub unsafe fn atomic_xchg_acquire<T: Primitive>(p: *mut T, v: T) -> T {
        T::xchg_acquire(p, v)
    }
    pub unsafe fn atomic_store_release<T: Primitive>(p: *mut T, v: T) {
        T::store_release(p, v)
    }
}
