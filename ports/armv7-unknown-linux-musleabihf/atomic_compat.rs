use core::sync::atomic::{AtomicI32, AtomicU32, Ordering};

pub trait AtomicValue: Copy {
    unsafe fn load_relaxed(ptr: *const Self) -> Self;
    unsafe fn load_acquire(ptr: *const Self) -> Self;
    unsafe fn swap_acquire(ptr: *mut Self, value: Self) -> Self;
    unsafe fn store_release(ptr: *mut Self, value: Self);
}

impl AtomicValue for i32 {
    unsafe fn load_relaxed(ptr: *const Self) -> Self {
        (&*(ptr.cast::<AtomicI32>())).load(Ordering::Relaxed)
    }
    unsafe fn load_acquire(ptr: *const Self) -> Self {
        (&*(ptr.cast::<AtomicI32>())).load(Ordering::Acquire)
    }
    unsafe fn swap_acquire(ptr: *mut Self, value: Self) -> Self {
        (&*(ptr.cast::<AtomicI32>())).swap(value, Ordering::Acquire)
    }
    unsafe fn store_release(ptr: *mut Self, value: Self) {
        (&*(ptr.cast::<AtomicI32>())).store(value, Ordering::Release)
    }
}

impl AtomicValue for u32 {
    unsafe fn load_relaxed(ptr: *const Self) -> Self {
        (&*(ptr.cast::<AtomicU32>())).load(Ordering::Relaxed)
    }
    unsafe fn load_acquire(ptr: *const Self) -> Self {
        (&*(ptr.cast::<AtomicU32>())).load(Ordering::Acquire)
    }
    unsafe fn swap_acquire(ptr: *mut Self, value: Self) -> Self {
        (&*(ptr.cast::<AtomicU32>())).swap(value, Ordering::Acquire)
    }
    unsafe fn store_release(ptr: *mut Self, value: Self) {
        (&*(ptr.cast::<AtomicU32>())).store(value, Ordering::Release)
    }
}

pub unsafe fn atomic_load_relaxed<T: AtomicValue>(ptr: *const T) -> T {
    T::load_relaxed(ptr)
}
pub unsafe fn atomic_load_acquire<T: AtomicValue>(ptr: *const T) -> T {
    T::load_acquire(ptr)
}
pub unsafe fn atomic_xchg_acquire<T: AtomicValue>(ptr: *mut T, value: T) -> T {
    T::swap_acquire(ptr, value)
}
pub unsafe fn atomic_store_release<T: AtomicValue>(ptr: *mut T, value: T) {
    T::store_release(ptr, value)
}
