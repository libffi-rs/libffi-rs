#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(static_mut_refs)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]

pub mod src {
    pub mod aarch64 {
        pub mod ffi;
    }
    pub mod closures;
    pub mod java_raw_api;
    pub mod prep_cif;
    pub mod raw_api;
    pub mod tramp;
    pub mod types;
}

/// ABI storage for GNU AArch64's IEEE binary128 `long double`.
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct F128(pub [u8; 16]);

trait AtomicValue: Copy {
    unsafe fn load(ptr: *const Self, order: core::sync::atomic::Ordering) -> Self;
    unsafe fn store(ptr: *mut Self, value: Self, order: core::sync::atomic::Ordering);
    unsafe fn swap(ptr: *mut Self, value: Self, order: core::sync::atomic::Ordering) -> Self;
}

impl AtomicValue for i32 {
    unsafe fn load(ptr: *const Self, order: core::sync::atomic::Ordering) -> Self {
        (&*ptr.cast::<core::sync::atomic::AtomicI32>()).load(order)
    }
    unsafe fn store(ptr: *mut Self, value: Self, order: core::sync::atomic::Ordering) {
        (&*ptr.cast::<core::sync::atomic::AtomicI32>()).store(value, order)
    }
    unsafe fn swap(ptr: *mut Self, value: Self, order: core::sync::atomic::Ordering) -> Self {
        (&*ptr.cast::<core::sync::atomic::AtomicI32>()).swap(value, order)
    }
}

impl AtomicValue for usize {
    unsafe fn load(ptr: *const Self, order: core::sync::atomic::Ordering) -> Self {
        (&*ptr.cast::<core::sync::atomic::AtomicUsize>()).load(order)
    }
    unsafe fn store(ptr: *mut Self, value: Self, order: core::sync::atomic::Ordering) {
        (&*ptr.cast::<core::sync::atomic::AtomicUsize>()).store(value, order)
    }
    unsafe fn swap(ptr: *mut Self, value: Self, order: core::sync::atomic::Ordering) -> Self {
        (&*ptr.cast::<core::sync::atomic::AtomicUsize>()).swap(value, order)
    }
}

unsafe fn atomic_load_relaxed<T: AtomicValue>(ptr: *const T) -> T {
    T::load(ptr, core::sync::atomic::Ordering::Relaxed)
}
unsafe fn atomic_load_acquire<T: AtomicValue>(ptr: *const T) -> T {
    T::load(ptr, core::sync::atomic::Ordering::Acquire)
}
unsafe fn atomic_store_release<T: AtomicValue>(ptr: *mut T, value: T) {
    T::store(ptr, value, core::sync::atomic::Ordering::Release)
}
unsafe fn atomic_xchg_acquire<T: AtomicValue>(ptr: *mut T, value: T) -> T {
    T::swap(ptr, value, core::sync::atomic::Ordering::Acquire)
}
