pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 1;
pub const FFI_LAST_ABI: ffi_abi = 9;
pub const FFI_MS_CDECL: ffi_abi = 8;
pub const FFI_REGISTER: ffi_abi = 7;
pub const FFI_PASCAL: ffi_abi = 6;
pub const FFI_STDCALL: ffi_abi = 5;
pub const FFI_FASTCALL: ffi_abi = 4;
pub const FFI_THISCALL: ffi_abi = 3;
pub const FFI_SYSV: ffi_abi = 1;
pub const FFI_FIRST_ABI: ffi_abi = 0;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ffi_type {
    pub size: size_t,
    pub alignment: ::core::ffi::c_ushort,
    pub type_0: ::core::ffi::c_ushort,
    pub elements: *mut *mut _ffi_type,
}
pub type ffi_type = _ffi_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_cif {
    pub abi: ffi_abi,
    pub nargs: ::core::ffi::c_uint,
    pub arg_types: *mut *mut ffi_type,
    pub rtype: *mut ffi_type,
    pub bytes: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_closure {
    pub c2rust_unnamed: C2RustUnnamed,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub tramp: [::core::ffi::c_char; 16],
    pub ftramp: *mut ::core::ffi::c_void,
}
pub type UINT8 = ::core::ffi::c_uchar;
pub type SINT8 = ::core::ffi::c_schar;
pub type UINT16 = ::core::ffi::c_ushort;
pub type SINT16 = ::core::ffi::c_short;
pub type UINT32 = ::core::ffi::c_uint;
pub type SINT32 = ::core::ffi::c_int;
pub type UINT64 = ::core::ffi::c_ulonglong;
pub type SINT64 = ::core::ffi::c_longlong;
pub const FFI_TYPE_VOID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FFI_TYPE_COMPLEX: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const FFI_VERSION_STRING: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"3.5.2\0") };
pub const FFI_VERSION_NUMBER: ::core::ffi::c_int = 30502 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
    alignment: 1 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 5 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_sint8: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<SINT8>() as size_t,
    alignment: 1 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 6 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_uint16: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<UINT16>() as size_t,
    alignment: 2 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 7 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_sint16: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<SINT16>() as size_t,
    alignment: 2 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 8 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_uint32: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<UINT32>() as size_t,
    alignment: 4 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 9 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_sint32: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<SINT32>() as size_t,
    alignment: 4 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 10 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_uint64: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<UINT64>() as size_t,
    alignment: 4 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 11 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_sint64: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<SINT64>() as size_t,
    alignment: 4 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 12 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_pointer: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    alignment: 4 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 14 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_float: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
    alignment: 4 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 2 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_double: ffi_type = _ffi_type {
    size: ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
    alignment: 4 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 3 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
#[no_mangle]
pub static mut ffi_type_longdouble: ffi_type = _ffi_type {
    // i686 GNU/Linux uses an 80-bit x87 value in 12 bytes, aligned to 4.
    // configure measured SIZEOF_LONG_DOUBLE=12 with gcc -m32.
    size: 12 as size_t,
    alignment: 4 as ::core::ffi::c_uint as ::core::ffi::c_ushort,
    type_0: 4 as ::core::ffi::c_ushort,
    elements: ::core::ptr::null::<*mut _ffi_type>() as *mut *mut _ffi_type,
};
static mut ffi_elements_complex_float: [*mut ffi_type; 2] = unsafe {
    [
        &raw const ffi_type_float as *mut ffi_type,
        ::core::ptr::null::<ffi_type>() as *mut ffi_type,
    ]
};
static mut ffi_elements_complex_double: [*mut ffi_type; 2] = unsafe {
    [
        &raw const ffi_type_double as *mut ffi_type,
        ::core::ptr::null::<ffi_type>() as *mut ffi_type,
    ]
};
static mut ffi_elements_complex_longdouble: [*mut ffi_type; 2] = unsafe {
    [
        &raw const ffi_type_longdouble as *mut ffi_type,
        ::core::ptr::null::<ffi_type>() as *mut ffi_type,
    ]
};

// C2Rust 0.22.1 cannot emit C complex-valued static initializers.
#[no_mangle]
pub static mut ffi_type_complex_float: ffi_type = _ffi_type {
    size: 8,
    alignment: 4,
    type_0: FFI_TYPE_COMPLEX as ::core::ffi::c_ushort,
    elements: &raw mut ffi_elements_complex_float as *mut *mut ffi_type,
};
#[no_mangle]
pub static mut ffi_type_complex_double: ffi_type = _ffi_type {
    size: 16,
    alignment: 4,
    type_0: FFI_TYPE_COMPLEX as ::core::ffi::c_ushort,
    elements: &raw mut ffi_elements_complex_double as *mut *mut ffi_type,
};
#[no_mangle]
pub static mut ffi_type_complex_longdouble: ffi_type = _ffi_type {
    size: 24,
    alignment: 4,
    type_0: FFI_TYPE_COMPLEX as ::core::ffi::c_ushort,
    elements: &raw mut ffi_elements_complex_longdouble as *mut *mut ffi_type,
};
