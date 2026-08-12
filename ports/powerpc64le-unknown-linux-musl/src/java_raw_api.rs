extern "C" {
    fn ffi_prep_closure_loc(
        _: *mut ffi_closure,
        _: *mut ffi_cif,
        fun: Option<
            unsafe extern "C" fn(
                *mut ffi_cif,
                *mut ::core::ffi::c_void,
                *mut *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        user_data: *mut ::core::ffi::c_void,
        codeloc: *mut ::core::ffi::c_void,
    ) -> ffi_status;
    fn ffi_call(
        cif: *mut ffi_cif,
        fn_0: Option<unsafe extern "C" fn() -> ()>,
        rvalue: *mut ::core::ffi::c_void,
        avalue: *mut *mut ::core::ffi::c_void,
    );
    fn abort() -> !;
}
pub type ffi_arg = ::core::ffi::c_ulong;
pub type ffi_sarg = ::core::ffi::c_long;
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_LAST_ABI: ffi_abi = 32;
pub const FFI_DEFAULT_ABI: ffi_abi = 8;
pub const FFI_SYSV_LONG_DOUBLE_128: ffi_abi = 16;
pub const FFI_SYSV_IBM_LONG_DOUBLE: ffi_abi = 4;
pub const FFI_SYSV_STRUCT_RET: ffi_abi = 2;
pub const FFI_SYSV_SOFT_FLOAT: ffi_abi = 1;
pub const FFI_SYSV: ffi_abi = 8;
pub const FFI_COMPAT_LINUX_SOFT_FLOAT: ffi_abi = 5;
pub const FFI_COMPAT_LINUX: ffi_abi = 4;
pub const FFI_COMPAT_LINUX64: ffi_abi = 3;
pub const FFI_COMPAT_GCC_SYSV: ffi_abi = 2;
pub const FFI_COMPAT_SYSV: ffi_abi = 1;
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
    pub nfixedargs: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ffi_raw {
    pub sint: ffi_sarg,
    pub uint: ffi_arg,
    pub flt: ::core::ffi::c_float,
    pub data: [::core::ffi::c_char; 8],
    pub ptr: *mut ::core::ffi::c_void,
}
pub type ffi_java_raw = ffi_raw;
pub type UINT64 = ::core::ffi::c_ulong;
pub type FLOAT32 = ::core::ffi::c_float;
pub type SINT32 = ::core::ffi::c_int;
pub type UINT32 = ::core::ffi::c_uint;
pub type SINT16 = ::core::ffi::c_short;
pub type UINT16 = ::core::ffi::c_ushort;
pub type SINT8 = ::core::ffi::c_schar;
pub type UINT8 = ::core::ffi::c_uchar;
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
    pub tramp: [::core::ffi::c_char; 40],
    pub ftramp: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_raw_closure {
    pub tramp: [::core::ffi::c_char; 40],
    pub cif: *mut ffi_cif,
    pub translate_args: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub this_closure: *mut ::core::ffi::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_java_raw_closure {
    pub tramp: [::core::ffi::c_char; 40],
    pub cif: *mut ffi_cif,
    pub translate_args: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub this_closure: *mut ::core::ffi::c_void,
    pub fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut ffi_java_raw,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub user_data: *mut ::core::ffi::c_void,
}
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
pub const FFI_SIZEOF_ARG: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
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
                result =
                    result.wrapping_add((2 as ::core::ffi::c_int * FFI_SIZEOF_JAVA_RAW) as size_t);
            }
            FFI_TYPE_STRUCT => {
                abort();
            }
            FFI_TYPE_COMPLEX => {
                abort();
            }
            _ => {
                result = result.wrapping_add(FFI_SIZEOF_JAVA_RAW as size_t);
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
        match (**tp).type_0 as ::core::ffi::c_int {
            FFI_TYPE_UINT64 | FFI_TYPE_SINT64 | FFI_TYPE_DOUBLE => {
                *args = raw as *mut ::core::ffi::c_void;
                raw = raw.offset(2 as ::core::ffi::c_int as isize);
            }
            FFI_TYPE_COMPLEX => {
                abort();
            }
            _ => {
                let fresh0 = raw;
                raw = raw.offset(1);
                *args = fresh0 as *mut ::core::ffi::c_void;
            }
        }
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
                let fresh1 = raw;
                raw = raw.offset(1);
                (*fresh1).uint = *(*args as *mut UINT8) as ffi_arg;
            }
            FFI_TYPE_SINT8 => {
                let fresh2 = raw;
                raw = raw.offset(1);
                (*fresh2).sint = *(*args as *mut SINT8) as ffi_sarg;
            }
            FFI_TYPE_UINT16 => {
                let fresh3 = raw;
                raw = raw.offset(1);
                (*fresh3).uint = *(*args as *mut UINT16) as ffi_arg;
            }
            FFI_TYPE_SINT16 => {
                let fresh4 = raw;
                raw = raw.offset(1);
                (*fresh4).sint = *(*args as *mut SINT16) as ffi_sarg;
            }
            FFI_TYPE_UINT32 => {
                let fresh5 = raw;
                raw = raw.offset(1);
                (*fresh5).uint = *(*args as *mut UINT32) as ffi_arg;
            }
            FFI_TYPE_SINT32 => {
                let fresh6 = raw;
                raw = raw.offset(1);
                (*fresh6).sint = *(*args as *mut SINT32) as ffi_sarg;
            }
            FFI_TYPE_FLOAT => {
                let fresh7 = raw;
                raw = raw.offset(1);
                (*fresh7).flt = *(*args as *mut FLOAT32) as ::core::ffi::c_float;
            }
            FFI_TYPE_UINT64 | FFI_TYPE_SINT64 | FFI_TYPE_DOUBLE => {
                (*raw).uint = *(*args as *mut UINT64) as ffi_arg;
                raw = raw.offset(2 as ::core::ffi::c_int as isize);
            }
            FFI_TYPE_POINTER => {
                let fresh8 = raw;
                raw = raw.offset(1);
                (*fresh8).ptr = **(args as *mut *mut *mut ::core::ffi::c_void);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        tp = tp.offset(1);
        args = args.offset(1);
    }
}
unsafe extern "C" fn ffi_java_rvalue_to_raw(
    mut cif: *mut ffi_cif,
    mut rvalue: *mut ::core::ffi::c_void,
) {
}
unsafe extern "C" fn ffi_java_raw_to_rvalue(
    mut cif: *mut ffi_cif,
    mut rvalue: *mut ::core::ffi::c_void,
) {
}
#[no_mangle]
pub unsafe extern "C" fn ffi_java_raw_call(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut raw: *mut ffi_java_raw,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    alloca_allocations.push(::std::vec::from_elem(
        0,
        ((*cif).nargs as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
            as usize,
    ));
    let mut avalue: *mut *mut ::core::ffi::c_void =
        alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_void;
    ffi_java_raw_to_ptrarray(cif, raw, avalue);
    ffi_call(cif, fn_0, rvalue, avalue);
    ffi_java_rvalue_to_raw(cif, rvalue);
}
unsafe extern "C" fn ffi_java_translate_args(
    mut cif: *mut ffi_cif,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
    mut user_data: *mut ::core::ffi::c_void,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    alloca_allocations.push(::std::vec::from_elem(0, ffi_java_raw_size(cif) as usize));
    let mut raw: *mut ffi_java_raw =
        alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ffi_java_raw;
    let mut cl: *mut ffi_raw_closure = user_data as *mut ffi_raw_closure;
    ffi_java_ptrarray_to_raw(cif, avalue, raw);
    Some((*cl).fun.expect("non-null function pointer")).expect("non-null function pointer")(
        cif,
        rvalue,
        raw as *mut ffi_raw,
        (*cl).user_data,
    );
    ffi_java_raw_to_rvalue(cif, rvalue);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_java_raw_closure_loc(
    mut cl: *mut ffi_java_raw_closure,
    mut cif: *mut ffi_cif,
    mut fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut ffi_java_raw,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut user_data: *mut ::core::ffi::c_void,
    mut codeloc: *mut ::core::ffi::c_void,
) -> ffi_status {
    let mut status: ffi_status = FFI_OK;
    status = ffi_prep_closure_loc(
        cl as *mut ffi_closure,
        cif,
        Some(
            ffi_java_translate_args
                as unsafe extern "C" fn(
                    *mut ffi_cif,
                    *mut ::core::ffi::c_void,
                    *mut *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        codeloc,
        codeloc,
    );
    if status as ::core::ffi::c_uint == FFI_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*cl).fun = fun;
        (*cl).user_data = user_data;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_java_raw_closure(
    mut cl: *mut ffi_java_raw_closure,
    mut cif: *mut ffi_cif,
    mut fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut ffi_java_raw,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut user_data: *mut ::core::ffi::c_void,
) -> ffi_status {
    return ffi_prep_java_raw_closure_loc(cl, cif, fun, user_data, cl as *mut ::core::ffi::c_void);
}
