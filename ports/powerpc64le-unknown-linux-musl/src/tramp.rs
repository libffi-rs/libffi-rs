//! Stable stubs for configured static-trampoline support.
//! Returning unsupported selects libffi powerpc64 ELFv2 inline trampolines.
use core::ffi::{c_int, c_void};
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_is_supported() -> c_int {
    0
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_alloc(_flags: c_int) -> *mut c_void {
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_set_parms(
    _arg: *mut c_void,
    _target: *mut c_void,
    _data: *mut c_void,
) {
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_get_addr(_arg: *mut c_void) -> *mut c_void {
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_free(_arg: *mut c_void) {}
