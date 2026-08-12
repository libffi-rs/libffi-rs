extern "C" {
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn abort() -> !;
}
pub type ffi_arg = ::core::ffi::c_ulong;
pub type ffi_sarg = ::core::ffi::c_long;
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
pub type size_t = ::core::ffi::c_uint;
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
pub union ffi_raw {
    pub sint: ffi_sarg,
    pub uint: ffi_arg,
    pub flt: ::core::ffi::c_float,
    pub data: [::core::ffi::c_char; 4],
    pub ptr: *mut ::core::ffi::c_void,
}
pub type ffi_java_raw = ffi_raw;
pub type FLOAT32 = ::core::ffi::c_float;
pub type SINT32 = ::core::ffi::c_int;
pub type UINT32 = ::core::ffi::c_uint;
pub type SINT16 = ::core::ffi::c_short;
pub type UINT16 = ::core::ffi::c_ushort;
pub type SINT8 = ::core::ffi::c_schar;
pub type UINT8 = ::core::ffi::c_uchar;
pub const FFI_TYPE_FLOAT: ::core::ffi::c_int = 2;
pub const FFI_TYPE_DOUBLE: ::core::ffi::c_int = 3;
pub const FFI_TYPE_UINT8: ::core::ffi::c_int = 5;
pub const FFI_TYPE_SINT8: ::core::ffi::c_int = 6;
pub const FFI_TYPE_UINT16: ::core::ffi::c_int = 7;
pub const FFI_TYPE_SINT16: ::core::ffi::c_int = 8;
pub const FFI_TYPE_UINT32: ::core::ffi::c_int = 9;
pub const FFI_TYPE_SINT32: ::core::ffi::c_int = 10;
pub const FFI_TYPE_UINT64: ::core::ffi::c_int = 11;
pub const FFI_TYPE_SINT64: ::core::ffi::c_int = 12;
pub const FFI_TYPE_STRUCT: ::core::ffi::c_int = 13;
pub const FFI_TYPE_POINTER: ::core::ffi::c_int = 14;
pub const FFI_TYPE_COMPLEX: ::core::ffi::c_int = 15;
pub const FFI_SIZEOF_ARG: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FFI_SIZEOF_JAVA_RAW: ::core::ffi::c_int = FFI_SIZEOF_ARG;
#[no_mangle]
pub unsafe extern "C" fn ffi_java_raw_size(mut cif: *mut ffi_cif) -> size_t {
    let mut result: size_t = 0 as size_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut at: *mut *mut ffi_type = (*cif).arg_types;
    i = (*cif).nargs.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        match (**at).type_0 as ::core::ffi::c_int {
            FFI_TYPE_UINT64 | FFI_TYPE_SINT64 | FFI_TYPE_DOUBLE => {
                result = (result as ::core::ffi::c_uint).wrapping_add(
                    (2 as ::core::ffi::c_int * FFI_SIZEOF_JAVA_RAW) as ::core::ffi::c_uint,
                ) as size_t as size_t;
            }
            FFI_TYPE_STRUCT => {
                abort();
            }
            FFI_TYPE_COMPLEX => {
                abort();
            }
            _ => {
                result = (result as ::core::ffi::c_uint)
                    .wrapping_add(FFI_SIZEOF_JAVA_RAW as ::core::ffi::c_uint)
                    as size_t as size_t;
            }
        }
        i -= 1;
        at = at.offset(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_java_raw_to_ptrarray(
    mut cif: *mut ffi_cif,
    mut raw: *mut ffi_java_raw,
    mut args: *mut *mut ::core::ffi::c_void,
) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut tp: *mut *mut ffi_type = (*cif).arg_types;
    i = 0 as ::core::ffi::c_uint;
    while i < (*cif).nargs {
        *args = raw as *mut ::core::ffi::c_void;
        raw = raw.offset(
            (((**tp).size as usize).wrapping_sub(1 as usize)
                | (::core::mem::size_of::<ffi_java_raw>() as usize).wrapping_sub(1 as usize))
            .wrapping_add(1 as usize)
            .wrapping_div(::core::mem::size_of::<ffi_java_raw>() as usize) as isize,
        );
        i = i.wrapping_add(1);
        tp = tp.offset(1);
        args = args.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ffi_java_ptrarray_to_raw(
    mut cif: *mut ffi_cif,
    mut args: *mut *mut ::core::ffi::c_void,
    mut raw: *mut ffi_java_raw,
) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut tp: *mut *mut ffi_type = (*cif).arg_types;
    i = 0 as ::core::ffi::c_uint;
    while i < (*cif).nargs {
        match (**tp).type_0 as ::core::ffi::c_int {
            FFI_TYPE_UINT8 => {
                let fresh0 = raw;
                raw = raw.offset(1);
                (*fresh0).uint = *(*args as *mut UINT8) as ffi_arg;
            }
            FFI_TYPE_SINT8 => {
                let fresh1 = raw;
                raw = raw.offset(1);
                (*fresh1).sint = *(*args as *mut SINT8) as ffi_sarg;
            }
            FFI_TYPE_UINT16 => {
                let fresh2 = raw;
                raw = raw.offset(1);
                (*fresh2).uint = *(*args as *mut UINT16) as ffi_arg;
            }
            FFI_TYPE_SINT16 => {
                let fresh3 = raw;
                raw = raw.offset(1);
                (*fresh3).sint = *(*args as *mut SINT16) as ffi_sarg;
            }
            FFI_TYPE_UINT32 => {
                let fresh4 = raw;
                raw = raw.offset(1);
                (*fresh4).uint = *(*args as *mut UINT32) as ffi_arg;
            }
            FFI_TYPE_SINT32 => {
                let fresh5 = raw;
                raw = raw.offset(1);
                (*fresh5).sint = *(*args as *mut SINT32) as ffi_sarg;
            }
            FFI_TYPE_FLOAT => {
                let fresh6 = raw;
                raw = raw.offset(1);
                (*fresh6).flt = *(*args as *mut FLOAT32) as ::core::ffi::c_float;
            }
            FFI_TYPE_POINTER => {
                let fresh7 = raw;
                raw = raw.offset(1);
                (*fresh7).ptr = **(args as *mut *mut *mut ::core::ffi::c_void);
            }
            _ => {
                memcpy(
                    &raw mut (*raw).data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    *args,
                    (**tp).size,
                );
                raw = raw.offset(
                    (((**tp).size as usize).wrapping_sub(1 as usize)
                        | (::core::mem::size_of::<ffi_java_raw>() as usize)
                            .wrapping_sub(1 as usize))
                    .wrapping_add(1 as usize)
                    .wrapping_div(::core::mem::size_of::<ffi_java_raw>() as usize)
                        as isize,
                );
            }
        }
        i = i.wrapping_add(1);
        tp = tp.offset(1);
        args = args.offset(1);
    }
}
