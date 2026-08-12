extern "C" {
    fn ffi_prep_raw_closure_loc(
        _: *mut ffi_raw_closure,
        cif: *mut ffi_cif,
        fun: Option<
            unsafe extern "C" fn(
                *mut ffi_cif,
                *mut ::core::ffi::c_void,
                *mut ffi_raw,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        user_data: *mut ::core::ffi::c_void,
        codeloc: *mut ::core::ffi::c_void,
    ) -> ffi_status;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
pub type ffi_status = ::core::ffi::c_uint;
pub const FFI_BAD_ARGTYPE: ffi_status = 3;
pub const FFI_BAD_ABI: ffi_status = 2;
pub const FFI_BAD_TYPEDEF: ffi_status = 1;
pub const FFI_OK: ffi_status = 0;
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
pub type SINT32 = ::core::ffi::c_int;
pub type UINT32 = ::core::ffi::c_uint;
pub type SINT16 = ::core::ffi::c_short;
pub type UINT16 = ::core::ffi::c_ushort;
pub type SINT8 = ::core::ffi::c_schar;
pub type UINT8 = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_raw_closure {
    pub tramp: [::core::ffi::c_char; 16],
    pub cif: *mut ffi_cif,
    pub fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut ffi_raw,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub user_data: *mut ::core::ffi::c_void,
}
pub const FFI_TYPE_UINT8: ::core::ffi::c_int = 5;
pub const FFI_TYPE_SINT8: ::core::ffi::c_int = 6;
pub const FFI_TYPE_UINT16: ::core::ffi::c_int = 7;
pub const FFI_TYPE_SINT16: ::core::ffi::c_int = 8;
pub const FFI_TYPE_UINT32: ::core::ffi::c_int = 9;
pub const FFI_TYPE_SINT32: ::core::ffi::c_int = 10;
pub const FFI_TYPE_STRUCT: ::core::ffi::c_int = 13;
pub const FFI_TYPE_POINTER: ::core::ffi::c_int = 14;
pub const FFI_TYPE_COMPLEX: ::core::ffi::c_int = 15;
pub const FFI_SIZEOF_ARG: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ffi_raw_size(mut cif: *mut ffi_cif) -> size_t {
    let mut result: size_t = 0 as size_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut at: *mut *mut ffi_type = (*cif).arg_types;
    i = (*cif).nargs.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        if (**at).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT {
            result = result.wrapping_add(
                (::core::mem::size_of::<*mut ::core::ffi::c_void>().wrapping_sub(1 as size_t)
                    | (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(1 as size_t),
            );
        } else {
            result = result.wrapping_add(
                ((**at).size.wrapping_sub(1 as size_t)
                    | (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(1 as size_t),
            );
        }
        i -= 1;
        at = at.offset(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_raw_to_ptrarray(
    mut cif: *mut ffi_cif,
    mut raw: *mut ffi_raw,
    mut args: *mut *mut ::core::ffi::c_void,
) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut tp: *mut *mut ffi_type = (*cif).arg_types;
    i = 0 as ::core::ffi::c_uint;
    while i < (*cif).nargs {
        if (**tp).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT {
            let fresh9 = raw;
            raw = raw.offset(1);
            *args = (*fresh9).ptr;
        } else if (**tp).type_0 as ::core::ffi::c_int == FFI_TYPE_COMPLEX {
            let fresh10 = raw;
            raw = raw.offset(1);
            *args = (*fresh10).ptr;
        } else {
            *args = raw as *mut ::core::ffi::c_void;
            raw = raw.offset(
                ((**tp).size.wrapping_sub(1 as size_t)
                    | (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                        .wrapping_sub(1 as size_t))
                .wrapping_add(1 as size_t)
                .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                    as isize,
            );
        }
        i = i.wrapping_add(1);
        tp = tp.offset(1);
        args = args.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ffi_ptrarray_to_raw(
    mut cif: *mut ffi_cif,
    mut args: *mut *mut ::core::ffi::c_void,
    mut raw: *mut ffi_raw,
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
            FFI_TYPE_STRUCT => {
                let fresh6 = raw;
                raw = raw.offset(1);
                (*fresh6).ptr = *args;
            }
            FFI_TYPE_COMPLEX => {
                let fresh7 = raw;
                raw = raw.offset(1);
                (*fresh7).ptr = *args;
            }
            FFI_TYPE_POINTER => {
                let fresh8 = raw;
                raw = raw.offset(1);
                (*fresh8).ptr = **(args as *mut *mut *mut ::core::ffi::c_void);
            }
            _ => {
                memcpy(
                    &raw mut (*raw).data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    *args,
                    (**tp).size,
                );
                raw = raw.offset(
                    ((**tp).size.wrapping_sub(1 as size_t)
                        | (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                        .wrapping_add(1 as size_t)
                        .wrapping_div(FFI_SIZEOF_ARG as size_t) as isize,
                );
            }
        }
        i = i.wrapping_add(1);
        tp = tp.offset(1);
        args = args.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_raw_closure(
    mut cl: *mut ffi_raw_closure,
    mut cif: *mut ffi_cif,
    mut fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut ffi_raw,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut user_data: *mut ::core::ffi::c_void,
) -> ffi_status {
    return ffi_prep_raw_closure_loc(cl, cif, fun, user_data, cl as *mut ::core::ffi::c_void);
}
