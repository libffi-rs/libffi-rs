pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_is_supported() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_alloc(
    mut flags: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_set_parms(
    mut arg: *mut ::core::ffi::c_void,
    mut target: *mut ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
) {
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_get_addr(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_free(mut arg: *mut ::core::ffi::c_void) {}
