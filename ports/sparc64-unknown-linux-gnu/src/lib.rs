#![allow(clippy::all)]
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
#![allow(unused_assignments, unused_mut, unused_parens, unused_unsafe)]
#![allow(improper_ctypes, static_mut_refs)]

pub mod closures;
pub mod java_raw_api;
pub mod prep_cif;
pub mod raw_api;
pub mod tramp;
pub mod types;
pub mod sparc {
    pub mod ffi64;
}

mod atomic_compat {
    use std::sync::atomic::{AtomicI32, AtomicU64, AtomicUsize, Ordering};

    pub unsafe trait AtomicValue: Copy {
        unsafe fn load_relaxed(p: *const Self) -> Self;
        unsafe fn load_acquire(p: *const Self) -> Self;
        unsafe fn exchange_acquire(p: *mut Self, value: Self) -> Self;
        unsafe fn store_release(p: *mut Self, value: Self);
    }

    unsafe impl AtomicValue for i32 {
        unsafe fn load_relaxed(p: *const Self) -> Self {
            (&*(p.cast::<AtomicI32>())).load(Ordering::Relaxed)
        }
        unsafe fn load_acquire(p: *const Self) -> Self {
            (&*(p.cast::<AtomicI32>())).load(Ordering::Acquire)
        }
        unsafe fn exchange_acquire(p: *mut Self, value: Self) -> Self {
            (&*(p.cast::<AtomicI32>())).swap(value, Ordering::Acquire)
        }
        unsafe fn store_release(p: *mut Self, value: Self) {
            (&*(p.cast::<AtomicI32>())).store(value, Ordering::Release)
        }
    }
    unsafe impl AtomicValue for usize {
        unsafe fn load_relaxed(p: *const Self) -> Self {
            (&*(p.cast::<AtomicUsize>())).load(Ordering::Relaxed)
        }
        unsafe fn load_acquire(p: *const Self) -> Self {
            (&*(p.cast::<AtomicUsize>())).load(Ordering::Acquire)
        }
        unsafe fn exchange_acquire(p: *mut Self, value: Self) -> Self {
            (&*(p.cast::<AtomicUsize>())).swap(value, Ordering::Acquire)
        }
        unsafe fn store_release(p: *mut Self, value: Self) {
            (&*(p.cast::<AtomicUsize>())).store(value, Ordering::Release)
        }
    }

    unsafe impl AtomicValue for u64 {
        unsafe fn load_relaxed(p: *const Self) -> Self {
            (&*(p.cast::<AtomicU64>())).load(Ordering::Relaxed)
        }
        unsafe fn load_acquire(p: *const Self) -> Self {
            (&*(p.cast::<AtomicU64>())).load(Ordering::Acquire)
        }
        unsafe fn exchange_acquire(p: *mut Self, value: Self) -> Self {
            (&*(p.cast::<AtomicU64>())).swap(value, Ordering::Acquire)
        }
        unsafe fn store_release(p: *mut Self, value: Self) {
            (&*(p.cast::<AtomicU64>())).store(value, Ordering::Release)
        }
    }

    pub unsafe fn atomic_load_relaxed<T: AtomicValue>(p: *const T) -> T {
        T::load_relaxed(p)
    }
    pub unsafe fn atomic_load_acquire<T: AtomicValue>(p: *const T) -> T {
        T::load_acquire(p)
    }
    pub unsafe fn atomic_xchg_acquire<T: AtomicValue>(p: *mut T, value: T) -> T {
        T::exchange_acquire(p, value)
    }
    pub unsafe fn atomic_store_release<T: AtomicValue>(p: *mut T, value: T) {
        T::store_release(p, value)
    }
}

#[doc(hidden)]
pub fn force_link() {}
