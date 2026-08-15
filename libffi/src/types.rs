pub use super::ffi::{_ffi_type, ffi_cif, ffi_type};

pub type __int128_t = i128;
pub type __uint128_t = u128;
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 1;
pub const FFI_LAST_ABI: ffi_abi = 3;
pub const FFI_WIN64: ffi_abi = 2;
pub const FFI_SYSV: ffi_abi = 1;
pub const FFI_FIRST_ABI: ffi_abi = 0;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_closure {
    pub trampoline_table: *mut ::core::ffi::c_void,
    pub trampoline_table_entry: *mut ::core::ffi::c_void,
    pub cif: *mut ffi_cif,
    pub fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub user_data: *mut ::core::ffi::c_void,
}
pub type UINT8 = ::core::ffi::c_uchar;
pub type SINT8 = ::core::ffi::c_schar;
pub type UINT16 = ::core::ffi::c_ushort;
pub type SINT16 = ::core::ffi::c_short;
pub type UINT32 = ::core::ffi::c_uint;
pub type SINT32 = ::core::ffi::c_int;
pub type UINT64 = ::core::ffi::c_ulong;
pub type SINT64 = ::core::ffi::c_long;
pub const FFI_TYPE_VOID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FFI_TYPE_COMPLEX: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const FFI_VERSION_STRING: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"3.5.2\0") };
pub const FFI_VERSION_NUMBER: ::core::ffi::c_int = 30502 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ffi_get_version() -> *const ::core::ffi::c_char {
    return FFI_VERSION_STRING.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn ffi_get_version_number() -> ::core::ffi::c_ulong {
    return FFI_VERSION_NUMBER as ::core::ffi::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_get_default_abi() -> ::core::ffi::c_uint {
    return FFI_DEFAULT_ABI as ::core::ffi::c_int as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_get_closure_size() -> size_t {
    return ::core::mem::size_of::<ffi_closure>() as size_t;
}
#[no_mangle]
pub static mut ffi_type_void: ffi_type = _ffi_type {
    size: 1 as size_t,
    alignment: 1 as ::core::ffi::c_ushort,
    type_0: FFI_TYPE_VOID as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_uint8: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<UINT8>() as size_t,
    alignment: 1 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 5 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_sint8: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<SINT8>() as size_t,
    alignment: 1 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 6 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_uint16: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<UINT16>() as size_t,
    alignment: 2 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 7 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_sint16: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<SINT16>() as size_t,
    alignment: 2 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 8 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_uint32: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<UINT32>() as size_t,
    alignment: 4 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 9 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_sint32: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<SINT32>() as size_t,
    alignment: 4 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 10 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_uint64: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<UINT64>() as size_t,
    alignment: 8 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 11 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_sint64: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<SINT64>() as size_t,
    alignment: 8 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 12 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_pointer: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    alignment: 8 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 14 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_float: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    alignment: 4 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 2 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_double: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
    alignment: 8 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 3 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_longdouble: ffi_type = _ffi_type {
    // long double has the same 64-bit representation as double on aarch64 macOS.
    size: ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
    alignment: 8 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 3 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};

#[repr(transparent)]
struct FfiElements([*mut ffi_type; 2]);

// These arrays are immutable after initialization. The raw pointers refer to
// process-wide ffi_type descriptors, matching libffi's C definitions.
unsafe impl Sync for FfiElements {}

static ffi_elements_complex_float: FfiElements = FfiElements(unsafe {
    [
        &raw const ffi_type_float as *mut ffi_type,
        ::core::ptr::null::<ffi_type>() as *mut ffi_type,
    ]
});
static ffi_elements_complex_double: FfiElements = FfiElements(unsafe {
    [
        &raw const ffi_type_double as *mut ffi_type,
        ::core::ptr::null::<ffi_type>() as *mut ffi_type,
    ]
});
static ffi_elements_complex_longdouble: FfiElements = FfiElements(unsafe {
    [
        &raw const ffi_type_longdouble as *mut ffi_type,
        ::core::ptr::null::<ffi_type>() as *mut ffi_type,
    ]
});

#[no_mangle]
pub static mut ffi_type_complex_float: ffi_type = _ffi_type {
    size: 2 * ::core::mem::size_of::<::core::ffi::c_float>(),
    alignment: ::core::mem::align_of::<::core::ffi::c_float>() as ::core::ffi::c_ushort,
    type_0: FFI_TYPE_COMPLEX as ::core::ffi::c_ushort,
    elements: &ffi_elements_complex_float.0 as *const _ as *mut *mut ffi_type,
};
#[no_mangle]
pub static mut ffi_type_complex_double: ffi_type = _ffi_type {
    size: 2 * ::core::mem::size_of::<::core::ffi::c_double>(),
    alignment: ::core::mem::align_of::<::core::ffi::c_double>() as ::core::ffi::c_ushort,
    type_0: FFI_TYPE_COMPLEX as ::core::ffi::c_ushort,
    elements: &ffi_elements_complex_double.0 as *const _ as *mut *mut ffi_type,
};
#[no_mangle]
pub static mut ffi_type_complex_longdouble: ffi_type = _ffi_type {
    size: 2 * ::core::mem::size_of::<::core::ffi::c_double>(),
    alignment: ::core::mem::align_of::<::core::ffi::c_double>() as ::core::ffi::c_ushort,
    type_0: FFI_TYPE_COMPLEX as ::core::ffi::c_ushort,
    elements: &ffi_elements_complex_longdouble.0 as *const _ as *mut *mut ffi_type,
};
#[no_mangle]
pub static mut ffi_type_uint128: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<__uint128_t>() as size_t,
    alignment: 16 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 16 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_sint128: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<__int128_t>() as size_t,
    alignment: 16 as ::core::ffi::c_ulong as ::core::ffi::c_ushort,
    type_0: 17 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
