extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ffi_tramp_is_present(closure: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    fn abort() -> !;
    fn ffi_tramp_set_parms(
        tramp: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        code: *mut ::core::ffi::c_void,
    );
    static mut ffi_arm_trampoline: [::core::ffi::c_uint; 2];
    fn ffi_call_SYSV(
        stack: *mut ::core::ffi::c_void,
        _: *mut call_frame,
        fn_0: Option<unsafe extern "C" fn() -> ()>,
    );
    fn ffi_call_VFP(
        vfp_space: *mut ::core::ffi::c_void,
        _: *mut call_frame,
        fn_0: Option<unsafe extern "C" fn() -> ()>,
        vfp_used: ::core::ffi::c_uint,
    );
    fn ffi_closure_SYSV();
    fn ffi_closure_VFP();
    fn ffi_closure_SYSV_alt();
    fn ffi_closure_VFP_alt();
    fn ffi_go_closure_SYSV();
    fn ffi_go_closure_VFP();
}
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 1;
pub const FFI_LAST_ABI: ffi_abi = 3;
pub const FFI_VFP: ffi_abi = 2;
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
    pub vfp_used: ::core::ffi::c_int,
    pub vfp_reg_free: ::core::ffi::c_ushort,
    pub vfp_nargs: ::core::ffi::c_ushort,
    pub vfp_args: [::core::ffi::c_schar; 16],
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
    pub tramp: [::core::ffi::c_char; 12],
    pub ftramp: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_go_closure {
    pub tramp: *mut ::core::ffi::c_void,
    pub cif: *mut ffi_cif,
    pub fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct call_frame {
    pub fp: *mut ::core::ffi::c_void,
    pub lr: *mut ::core::ffi::c_void,
    pub rvalue: *mut ::core::ffi::c_void,
    pub flags: ::core::ffi::c_int,
    pub closure: *mut ::core::ffi::c_void,
}
pub type UINT64 = ::core::ffi::c_ulonglong;
pub type UINT32 = ::core::ffi::c_uint;
pub type UINT16 = ::core::ffi::c_ushort;
pub type SINT16 = ::core::ffi::c_short;
pub type UINT8 = ::core::ffi::c_uchar;
pub type SINT8 = ::core::ffi::c_schar;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct closure_frame {
    pub vfp_space: [::core::ffi::c_char; 64],
    pub result: [::core::ffi::c_char; 32],
    pub argp: [::core::ffi::c_char; 0],
}
pub const FFI_TYPE_VOID: ::core::ffi::c_int = 0;
pub const FFI_TYPE_INT: ::core::ffi::c_int = 1;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn atol(
    mut __nptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_long {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn atoll(
    mut __nptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_longlong {
    return strtoll(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
    return (__bsx & 0xff00000000000000 as __uint64_t) >> 56 as ::core::ffi::c_int
        | (__bsx & 0xff000000000000 as __uint64_t) >> 40 as ::core::ffi::c_int
        | (__bsx & 0xff0000000000 as __uint64_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff00000000 as __uint64_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff000000 as __uint64_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint64_t) << 24 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint64_t) << 40 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint64_t) << 56 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
}
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const ::core::ffi::c_void,
    mut __base: *const ::core::ffi::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut ::core::ffi::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>();
    let mut __comparison: ::core::ffi::c_int = 0;
    __l = 0 as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as size_t);
        __p = (__base as *const ::core::ffi::c_char)
            .offset(__idx.wrapping_mul(__size) as isize) as *const ::core::ffi::c_void;
        __comparison = Some(__compar.expect("non-null function pointer"))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as ::core::ffi::c_int {
            __u = __idx;
        } else if __comparison > 0 as ::core::ffi::c_int {
            __l = __idx.wrapping_add(1 as size_t);
        } else {
            return __p as *mut ::core::ffi::c_void
        }
    }
    return NULL;
}
#[inline]
unsafe extern "C" fn atof(
    mut __nptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    return strtod(__nptr, NULL as *mut *mut ::core::ffi::c_char);
}
pub const ARM_TYPE_VFP_S: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ARM_TYPE_VFP_D: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ARM_TYPE_VFP_N: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ARM_TYPE_INT64: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ARM_TYPE_INT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ARM_TYPE_VOID: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ARM_TYPE_STRUCT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ARM_TRAMP_MAP_SHIFT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const ARM_TRAMP_MAP_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << ARM_TRAMP_MAP_SHIFT;
pub const ARM_TRAMP_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
unsafe extern "C" fn ffi_align(
    mut ty: *mut ffi_type,
    mut p: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut alignment: size_t = 0;
    alignment = (*ty).alignment as size_t;
    if alignment < 4 as size_t {
        alignment = 4 as size_t;
    }
    return ((p as size_t).wrapping_sub(1 as size_t)
        | alignment.wrapping_sub(1 as size_t))
        .wrapping_add(1 as size_t) as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn ffi_put_arg(
    mut ty: *mut ffi_type,
    mut src: *mut ::core::ffi::c_void,
    mut dst: *mut ::core::ffi::c_void,
) -> size_t {
    let mut z: size_t = (*ty).size;
    match (*ty).type_0 as ::core::ffi::c_int {
        FFI_TYPE_SINT8 => {
            *(dst as *mut UINT32) = *(src as *mut SINT8) as UINT32;
        }
        FFI_TYPE_UINT8 => {
            *(dst as *mut UINT32) = *(src as *mut UINT8) as UINT32;
        }
        FFI_TYPE_SINT16 => {
            *(dst as *mut UINT32) = *(src as *mut SINT16) as UINT32;
        }
        FFI_TYPE_UINT16 => {
            *(dst as *mut UINT32) = *(src as *mut UINT16) as UINT32;
        }
        FFI_TYPE_INT
        | FFI_TYPE_SINT32
        | FFI_TYPE_UINT32
        | FFI_TYPE_POINTER
        | FFI_TYPE_FLOAT => {
            *(dst as *mut UINT32) = *(src as *mut UINT32);
        }
        FFI_TYPE_SINT64 | FFI_TYPE_UINT64 | FFI_TYPE_DOUBLE => {
            *(dst as *mut UINT64) = *(src as *mut UINT64);
        }
        FFI_TYPE_STRUCT | FFI_TYPE_COMPLEX => {
            memcpy(dst, src, z);
        }
        _ => {
            abort();
        }
    }
    return (z.wrapping_sub(1 as size_t)
        | (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
        .wrapping_add(1 as size_t);
}
unsafe extern "C" fn ffi_prep_args_SYSV(
    mut cif: *mut ffi_cif,
    mut flags: ::core::ffi::c_int,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
    mut argp: *mut ::core::ffi::c_char,
) {
    let mut arg_types: *mut *mut ffi_type = (*cif).arg_types;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if flags == ARM_TYPE_STRUCT {
        let ref mut fresh0 = *(argp as *mut *mut ::core::ffi::c_void);
        *fresh0 = rvalue;
        argp = argp.offset(4 as ::core::ffi::c_int as isize);
    }
    i = 0 as ::core::ffi::c_int;
    n = (*cif).nargs as ::core::ffi::c_int;
    while i < n {
        let mut ty: *mut ffi_type = *arg_types.offset(i as isize);
        argp = ffi_align(ty, argp as *mut ::core::ffi::c_void)
            as *mut ::core::ffi::c_char;
        argp = argp
            .offset(
                ffi_put_arg(
                    ty,
                    *avalue.offset(i as isize),
                    argp as *mut ::core::ffi::c_void,
                ) as isize,
            );
        i += 1;
    }
}
unsafe extern "C" fn ffi_prep_args_VFP(
    mut cif: *mut ffi_cif,
    mut flags: ::core::ffi::c_int,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
    mut stack: *mut ::core::ffi::c_char,
    mut vfp_space: *mut ::core::ffi::c_char,
) {
    let mut arg_types: *mut *mut ffi_type = (*cif).arg_types;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut vi: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut argp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut regp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut eo_regp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut stack_used: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    let mut done_with_regs: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    regp = stack;
    argp = regp.offset(16 as ::core::ffi::c_int as isize);
    eo_regp = argp;
    if flags == ARM_TYPE_STRUCT {
        let ref mut fresh1 = *(regp as *mut *mut ::core::ffi::c_void);
        *fresh1 = rvalue;
        regp = regp.offset(4 as ::core::ffi::c_int as isize);
    }
    let mut current_block_18: u64;
    i = 0 as ::core::ffi::c_int;
    n = (*cif).nargs as ::core::ffi::c_int;
    while i < n {
        let mut ty: *mut ffi_type = *arg_types.offset(i as isize);
        let mut a: *mut ::core::ffi::c_void = *avalue.offset(i as isize);
        let mut is_vfp_type: ::core::ffi::c_int = vfp_type_p(ty);
        if vi < (*cif).vfp_nargs as ::core::ffi::c_int && is_vfp_type != 0 {
            let fresh2 = vi;
            vi = vi + 1;
            let mut vfp_slot: *mut ::core::ffi::c_char = vfp_space
                .offset(
                    ((*cif).vfp_args[fresh2 as usize] as ::core::ffi::c_int
                        * 4 as ::core::ffi::c_int) as isize,
                );
            ffi_put_arg(ty, a, vfp_slot as *mut ::core::ffi::c_void);
        } else {
            if done_with_regs == 0 && is_vfp_type == 0 {
                let mut tregp: *mut ::core::ffi::c_char = ffi_align(
                    ty,
                    regp as *mut ::core::ffi::c_void,
                ) as *mut ::core::ffi::c_char;
                let mut size: size_t = (*ty).size;
                size = if size < 4 as size_t { 4 as size_t } else { size };
                if tregp.offset(size as isize) <= eo_regp {
                    regp = tregp
                        .offset(
                            ffi_put_arg(ty, a, tregp as *mut ::core::ffi::c_void)
                                as isize,
                        );
                    done_with_regs = (regp == argp) as ::core::ffi::c_int
                        as ::core::ffi::c_char;
                    current_block_18 = 4906268039856690917;
                } else if stack_used == 0 {
                    stack_used = 1 as ::core::ffi::c_char;
                    done_with_regs = 1 as ::core::ffi::c_char;
                    argp = tregp
                        .offset(
                            ffi_put_arg(ty, a, tregp as *mut ::core::ffi::c_void)
                                as isize,
                        );
                    current_block_18 = 4906268039856690917;
                } else {
                    current_block_18 = 15089075282327824602;
                }
            } else {
                current_block_18 = 15089075282327824602;
            }
            match current_block_18 {
                4906268039856690917 => {}
                _ => {
                    stack_used = 1 as ::core::ffi::c_char;
                    argp = ffi_align(ty, argp as *mut ::core::ffi::c_void)
                        as *mut ::core::ffi::c_char;
                    argp = argp
                        .offset(
                            ffi_put_arg(ty, a, argp as *mut ::core::ffi::c_void) as isize,
                        );
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep(mut cif: *mut ffi_cif) -> ffi_status {
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cabi: ::core::ffi::c_int = (*cif).abi as ::core::ffi::c_int;
    let mut bytes: size_t = (*cif).bytes as size_t;
    if cabi == FFI_VFP as ::core::ffi::c_int {
        layout_vfp_args(cif);
    }
    let mut current_block_17: u64;
    match (*(*cif).rtype).type_0 as ::core::ffi::c_int {
        FFI_TYPE_VOID => {
            flags = ARM_TYPE_VOID;
        }
        FFI_TYPE_INT
        | FFI_TYPE_UINT8
        | FFI_TYPE_SINT8
        | FFI_TYPE_UINT16
        | FFI_TYPE_SINT16
        | FFI_TYPE_UINT32
        | FFI_TYPE_SINT32
        | FFI_TYPE_POINTER => {
            flags = ARM_TYPE_INT;
        }
        FFI_TYPE_SINT64 | FFI_TYPE_UINT64 => {
            flags = ARM_TYPE_INT64;
        }
        FFI_TYPE_FLOAT => {
            flags = if cabi == FFI_VFP as ::core::ffi::c_int {
                ARM_TYPE_VFP_S
            } else {
                ARM_TYPE_INT
            };
        }
        FFI_TYPE_DOUBLE => {
            flags = if cabi == FFI_VFP as ::core::ffi::c_int {
                ARM_TYPE_VFP_D
            } else {
                ARM_TYPE_INT64
            };
        }
        FFI_TYPE_STRUCT | FFI_TYPE_COMPLEX => {
            if cabi == FFI_VFP as ::core::ffi::c_int {
                let mut h: ::core::ffi::c_int = vfp_type_p((*cif).rtype);
                flags = ARM_TYPE_VFP_N;
                if h == 0x100 as ::core::ffi::c_int + FFI_TYPE_FLOAT {
                    flags = ARM_TYPE_VFP_S;
                }
                if h == 0x100 as ::core::ffi::c_int + FFI_TYPE_DOUBLE {
                    flags = ARM_TYPE_VFP_D;
                }
                if h != 0 as ::core::ffi::c_int {
                    current_block_17 = 7172762164747879670;
                } else {
                    current_block_17 = 26972500619410423;
                }
            } else {
                current_block_17 = 26972500619410423;
            }
            match current_block_17 {
                7172762164747879670 => {}
                _ => {
                    if (*(*cif).rtype).size <= 4 as size_t {
                        flags = ARM_TYPE_INT;
                    } else {
                        flags = ARM_TYPE_STRUCT;
                        bytes = bytes.wrapping_add(4 as size_t);
                    }
                }
            }
        }
        _ => {
            abort();
        }
    }
    bytes = (bytes.wrapping_sub(1 as size_t)
        | (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
        .wrapping_add(1 as size_t);
    if bytes < (4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as size_t {
        bytes = (4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as size_t;
    }
    (*cif).bytes = bytes as ::core::ffi::c_uint;
    (*cif).flags = flags as ::core::ffi::c_uint;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep_var(
    mut cif: *mut ffi_cif,
    mut nfixedargs: ::core::ffi::c_uint,
    mut ntotalargs: ::core::ffi::c_uint,
) -> ffi_status {
    if (*cif).abi as ::core::ffi::c_uint
        == FFI_VFP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cif).abi = FFI_SYSV;
    }
    return ffi_prep_cif_machdep(cif);
}
unsafe extern "C" fn ffi_call_int(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
    mut closure: *mut ::core::ffi::c_void,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut flags: ::core::ffi::c_int = (*cif).flags as ::core::ffi::c_int;
    let mut rtype: *mut ffi_type = (*cif).rtype;
    let mut bytes: size_t = 0;
    let mut rsize: size_t = 0;
    let mut vfp_size: size_t = 0;
    let mut stack: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut vfp_space: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut new_rvalue: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut frame: *mut call_frame = ::core::ptr::null_mut::<call_frame>();
    rsize = 0 as size_t;
    if rvalue.is_null() {
        if flags == ARM_TYPE_STRUCT {
            rsize = (*rtype).size;
        } else {
            flags = ARM_TYPE_VOID;
        }
    } else if flags == ARM_TYPE_VFP_N {
        rsize = 32 as size_t;
    } else if flags == ARM_TYPE_INT
        && (*rtype).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT
    {
        rsize = 4 as size_t;
    }
    vfp_size = (if (*cif).abi as ::core::ffi::c_uint
        == FFI_VFP as ::core::ffi::c_int as ::core::ffi::c_uint && (*cif).vfp_used != 0
    {
        8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as size_t;
    bytes = (*cif).bytes as size_t;
    alloca_allocations
        .push(
            ::std::vec::from_elem(
                0,
                vfp_size
                    .wrapping_add(bytes)
                    .wrapping_add(::core::mem::size_of::<call_frame>() as size_t)
                    .wrapping_add(rsize) as usize,
            ),
        );
    stack = alloca_allocations.last_mut().unwrap().as_mut_ptr()
        as *mut ::core::ffi::c_char;
    vfp_space = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if vfp_size != 0 {
        vfp_space = stack;
        stack = stack.offset(vfp_size as isize);
    }
    frame = stack.offset(bytes as isize) as *mut call_frame;
    new_rvalue = rvalue as *mut ::core::ffi::c_char;
    if rsize != 0 {
        new_rvalue = frame.offset(1 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char;
    }
    (*frame).rvalue = new_rvalue as *mut ::core::ffi::c_void;
    (*frame).flags = flags;
    (*frame).closure = closure;
    if !vfp_space.is_null() {
        ffi_prep_args_VFP(
            cif,
            flags,
            new_rvalue as *mut ::core::ffi::c_void,
            avalue,
            stack,
            vfp_space,
        );
        ffi_call_VFP(
            vfp_space as *mut ::core::ffi::c_void,
            frame,
            fn_0,
            (*cif).vfp_used as ::core::ffi::c_uint,
        );
    } else {
        ffi_prep_args_SYSV(
            cif,
            flags,
            new_rvalue as *mut ::core::ffi::c_void,
            avalue,
            stack,
        );
        ffi_call_SYSV(stack as *mut ::core::ffi::c_void, frame, fn_0);
    }
    if !rvalue.is_null() && rvalue != new_rvalue as *mut ::core::ffi::c_void {
        memcpy(rvalue, new_rvalue as *const ::core::ffi::c_void, (*rtype).size);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ffi_call(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
) {
    ffi_call_int(cif, fn_0, rvalue, avalue, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_call_go(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
    mut closure: *mut ::core::ffi::c_void,
) {
    ffi_call_int(cif, fn_0, rvalue, avalue, closure);
}
unsafe extern "C" fn ffi_prep_incoming_args_SYSV(
    mut cif: *mut ffi_cif,
    mut rvalue: *mut ::core::ffi::c_void,
    mut argp: *mut ::core::ffi::c_char,
    mut avalue: *mut *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut arg_types: *mut *mut ffi_type = (*cif).arg_types;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if (*cif).flags == ARM_TYPE_STRUCT as ::core::ffi::c_uint {
        rvalue = *(argp as *mut *mut ::core::ffi::c_void);
        argp = argp.offset(4 as ::core::ffi::c_int as isize);
    } else if (*(*cif).rtype).size != 0 && (*(*cif).rtype).size < 4 as size_t {
        *(rvalue as *mut uint32_t) = 0 as uint32_t;
    }
    i = 0 as ::core::ffi::c_int;
    n = (*cif).nargs as ::core::ffi::c_int;
    while i < n {
        let mut ty: *mut ffi_type = *arg_types.offset(i as isize);
        let mut z: size_t = (*ty).size;
        argp = ffi_align(ty, argp as *mut ::core::ffi::c_void)
            as *mut ::core::ffi::c_char;
        let ref mut fresh4 = *avalue.offset(i as isize);
        *fresh4 = argp as *mut ::core::ffi::c_void;
        argp = argp.offset(z as isize);
        i += 1;
    }
    return rvalue;
}
unsafe extern "C" fn ffi_prep_incoming_args_VFP(
    mut cif: *mut ffi_cif,
    mut rvalue: *mut ::core::ffi::c_void,
    mut stack: *mut ::core::ffi::c_char,
    mut vfp_space: *mut ::core::ffi::c_char,
    mut avalue: *mut *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut arg_types: *mut *mut ffi_type = (*cif).arg_types;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut vi: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut argp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut regp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut eo_regp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut done_with_regs: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    let mut stack_used: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    regp = stack;
    argp = regp.offset(16 as ::core::ffi::c_int as isize);
    eo_regp = argp;
    if (*cif).flags == ARM_TYPE_STRUCT as ::core::ffi::c_uint {
        rvalue = *(regp as *mut *mut ::core::ffi::c_void);
        regp = regp.offset(4 as ::core::ffi::c_int as isize);
    }
    let mut current_block_22: u64;
    i = 0 as ::core::ffi::c_int;
    n = (*cif).nargs as ::core::ffi::c_int;
    while i < n {
        let mut ty: *mut ffi_type = *arg_types.offset(i as isize);
        let mut is_vfp_type: ::core::ffi::c_int = vfp_type_p(ty);
        let mut z: size_t = (*ty).size;
        if vi < (*cif).vfp_nargs as ::core::ffi::c_int && is_vfp_type != 0 {
            let fresh5 = vi;
            vi = vi + 1;
            let ref mut fresh6 = *avalue.offset(i as isize);
            *fresh6 = vfp_space
                .offset(
                    ((*cif).vfp_args[fresh5 as usize] as ::core::ffi::c_int
                        * 4 as ::core::ffi::c_int) as isize,
                ) as *mut ::core::ffi::c_void;
        } else {
            if done_with_regs == 0 && is_vfp_type == 0 {
                let mut tregp: *mut ::core::ffi::c_char = ffi_align(
                    ty,
                    regp as *mut ::core::ffi::c_void,
                ) as *mut ::core::ffi::c_char;
                z = if z < 4 as size_t { 4 as size_t } else { z };
                if tregp.offset(z as isize) <= eo_regp || stack_used == 0 {
                    let ref mut fresh7 = *avalue.offset(i as isize);
                    *fresh7 = tregp as *mut ::core::ffi::c_void;
                    regp = tregp.offset(z as isize);
                    if regp > eo_regp {
                        argp = regp;
                    }
                    if regp >= eo_regp {
                        done_with_regs = 1 as ::core::ffi::c_char;
                        stack_used = 1 as ::core::ffi::c_char;
                    }
                    current_block_22 = 4906268039856690917;
                } else {
                    current_block_22 = 11042950489265723346;
                }
            } else {
                current_block_22 = 11042950489265723346;
            }
            match current_block_22 {
                4906268039856690917 => {}
                _ => {
                    stack_used = 1 as ::core::ffi::c_char;
                    argp = ffi_align(ty, argp as *mut ::core::ffi::c_void)
                        as *mut ::core::ffi::c_char;
                    let ref mut fresh8 = *avalue.offset(i as isize);
                    *fresh8 = argp as *mut ::core::ffi::c_void;
                    argp = argp.offset(z as isize);
                }
            }
        }
        i += 1;
    }
    return rvalue;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_inner_SYSV(
    mut cif: *mut ffi_cif,
    mut fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut user_data: *mut ::core::ffi::c_void,
    mut frame: *mut closure_frame,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    alloca_allocations
        .push(
            ::std::vec::from_elem(
                0,
                ((*cif).nargs as usize)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize,
                    ) as usize,
            ),
        );
    let mut avalue: *mut *mut ::core::ffi::c_void = alloca_allocations
        .last_mut()
        .unwrap()
        .as_mut_ptr() as *mut *mut ::core::ffi::c_void;
    let mut rvalue: *mut ::core::ffi::c_void = ffi_prep_incoming_args_SYSV(
        cif,
        &raw mut (*frame).result as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        &raw mut (*frame).argp as *mut ::core::ffi::c_char,
        avalue,
    );
    fun.expect("non-null function pointer")(cif, rvalue, avalue, user_data);
    return (*cif).flags as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_inner_VFP(
    mut cif: *mut ffi_cif,
    mut fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut user_data: *mut ::core::ffi::c_void,
    mut frame: *mut closure_frame,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    alloca_allocations
        .push(
            ::std::vec::from_elem(
                0,
                ((*cif).nargs as usize)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize,
                    ) as usize,
            ),
        );
    let mut avalue: *mut *mut ::core::ffi::c_void = alloca_allocations
        .last_mut()
        .unwrap()
        .as_mut_ptr() as *mut *mut ::core::ffi::c_void;
    let mut rvalue: *mut ::core::ffi::c_void = ffi_prep_incoming_args_VFP(
        cif,
        &raw mut (*frame).result as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        &raw mut (*frame).argp as *mut ::core::ffi::c_char,
        &raw mut (*frame).vfp_space as *mut ::core::ffi::c_char,
        avalue,
    );
    fun.expect("non-null function pointer")(cif, rvalue, avalue, user_data);
    return (*cif).flags as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_go_closure(
    mut closure: *mut ffi_go_closure,
    mut cif: *mut ffi_cif,
    mut fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
) -> ffi_status {
    let mut closure_func: Option<unsafe extern "C" fn() -> ()> = Some(
        ffi_go_closure_SYSV as unsafe extern "C" fn() -> (),
    );
    if (*cif).abi as ::core::ffi::c_uint
        == FFI_VFP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*cif).vfp_used != 0 {
            closure_func = Some(ffi_go_closure_VFP as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>;
        }
    } else if (*cif).abi as ::core::ffi::c_uint
        != FFI_SYSV as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return FFI_BAD_ABI
    }
    (*closure).tramp = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        *mut ::core::ffi::c_void,
    >(closure_func);
    (*closure).cif = cif;
    (*closure).fun = fun;
    return FFI_OK;
}
unsafe extern "C" fn is_hfa0(mut ty: *const ffi_type) -> ::core::ffi::c_int {
    let mut elements: *mut *mut ffi_type = (*ty).elements as *mut *mut ffi_type;
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if !elements.is_null() {
        i = 0 as ::core::ffi::c_int;
        while !(*elements.offset(i as isize)).is_null() {
            ret = (**elements.offset(i as isize)).type_0 as ::core::ffi::c_int;
            if !(ret == FFI_TYPE_STRUCT || ret == FFI_TYPE_COMPLEX) {
                break;
            }
            ret = is_hfa0(*elements.offset(i as isize));
            if !(ret < 0 as ::core::ffi::c_int) {
                break;
            }
            i += 1;
        }
    }
    return ret;
}
unsafe extern "C" fn is_hfa1(
    mut ty: *const ffi_type,
    mut candidate: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut elements: *mut *mut ffi_type = (*ty).elements as *mut *mut ffi_type;
    let mut i: ::core::ffi::c_int = 0;
    if !elements.is_null() {
        i = 0 as ::core::ffi::c_int;
        while !(*elements.offset(i as isize)).is_null() {
            let mut t: ::core::ffi::c_int = (**elements.offset(i as isize)).type_0
                as ::core::ffi::c_int;
            if t == FFI_TYPE_STRUCT || t == FFI_TYPE_COMPLEX {
                if is_hfa1(*elements.offset(i as isize), candidate) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            } else if t != candidate {
                return 0 as ::core::ffi::c_int
            }
            i += 1;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn vfp_type_p(mut ty: *const ffi_type) -> ::core::ffi::c_int {
    let mut elements: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut candidate: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut size: size_t = 0;
    let mut ele_count: size_t = 0;
    candidate = (*ty).type_0 as ::core::ffi::c_int;
    match (*ty).type_0 as ::core::ffi::c_int {
        FFI_TYPE_FLOAT | FFI_TYPE_DOUBLE => {
            ele_count = 1 as size_t;
        }
        FFI_TYPE_COMPLEX => {
            candidate = (**(*ty).elements.offset(0 as ::core::ffi::c_int as isize))
                .type_0 as ::core::ffi::c_int;
            if candidate != FFI_TYPE_FLOAT && candidate != FFI_TYPE_DOUBLE {
                return 0 as ::core::ffi::c_int;
            }
            ele_count = 2 as size_t;
        }
        FFI_TYPE_STRUCT => {
            size = (*ty).size;
            if size < 4 as size_t || size > 32 as size_t {
                return 0 as ::core::ffi::c_int;
            }
            elements = (*ty).elements as *mut *mut ffi_type;
            candidate = (**elements.offset(0 as ::core::ffi::c_int as isize)).type_0
                as ::core::ffi::c_int;
            if candidate == FFI_TYPE_STRUCT || candidate == FFI_TYPE_COMPLEX {
                i = 0 as ::core::ffi::c_int;
                loop {
                    candidate = is_hfa0(*elements.offset(i as isize));
                    if candidate >= 0 as ::core::ffi::c_int {
                        break;
                    }
                    i += 1;
                }
            }
            match candidate {
                FFI_TYPE_FLOAT => {
                    ele_count = size
                        .wrapping_div(
                            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                        );
                    if size
                        != ele_count
                            .wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                            )
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                FFI_TYPE_DOUBLE => {
                    ele_count = size
                        .wrapping_div(
                            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                        );
                    if size
                        != ele_count
                            .wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                            )
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                _ => return 0 as ::core::ffi::c_int,
            }
            if ele_count > 4 as size_t {
                return 0 as ::core::ffi::c_int;
            }
            i = 0 as ::core::ffi::c_int;
            while !(*elements.offset(i as isize)).is_null() {
                let mut t: ::core::ffi::c_int = (**elements.offset(i as isize)).type_0
                    as ::core::ffi::c_int;
                if t == FFI_TYPE_STRUCT || t == FFI_TYPE_COMPLEX {
                    if is_hfa1(*elements.offset(i as isize), candidate) == 0 {
                        return 0 as ::core::ffi::c_int;
                    }
                } else if t != candidate {
                    return 0 as ::core::ffi::c_int
                }
                i += 1;
            }
        }
        _ => return 0 as ::core::ffi::c_int,
    }
    return (ele_count << 8 as ::core::ffi::c_int | candidate as size_t)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn place_vfp_arg(
    mut cif: *mut ffi_cif,
    mut h: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut reg: ::core::ffi::c_ushort = (*cif).vfp_reg_free;
    let mut align: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut nregs: ::core::ffi::c_int = h >> 8 as ::core::ffi::c_int;
    if h & 0xff as ::core::ffi::c_int == FFI_TYPE_DOUBLE {
        align = 2 as ::core::ffi::c_int;
        nregs *= 2 as ::core::ffi::c_int;
    }
    if reg as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
        && align == 2 as ::core::ffi::c_int
    {
        reg = reg.wrapping_add(1);
    }
    while reg as ::core::ffi::c_int + nregs <= 16 as ::core::ffi::c_int {
        let mut current_block_16: u64;
        let mut s: ::core::ffi::c_int = 0;
        let mut new_used: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        s = reg as ::core::ffi::c_int;
        loop {
            if !(s < reg as ::core::ffi::c_int + nregs) {
                current_block_16 = 1917311967535052937;
                break;
            }
            new_used |= (1 as ::core::ffi::c_int) << s;
            if (*cif).vfp_used & (1 as ::core::ffi::c_int) << s != 0 {
                reg = (reg as ::core::ffi::c_int + align) as ::core::ffi::c_ushort;
                current_block_16 = 6009453772311597924;
                break;
            } else {
                s += 1;
            }
        }
        match current_block_16 {
            6009453772311597924 => {}
            _ => {
                (*cif).vfp_used |= new_used;
                let fresh3 = (*cif).vfp_nargs;
                (*cif).vfp_nargs = (*cif).vfp_nargs.wrapping_add(1);
                (*cif).vfp_args[fresh3 as usize] = reg as ::core::ffi::c_schar;
                if (*cif).vfp_used
                    & (1 as ::core::ffi::c_int)
                        << (*cif).vfp_reg_free as ::core::ffi::c_int != 0
                {
                    reg = (reg as ::core::ffi::c_int + nregs) as ::core::ffi::c_ushort;
                    while (*cif).vfp_used
                        & (1 as ::core::ffi::c_int) << reg as ::core::ffi::c_int != 0
                    {
                        reg = (reg as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_ushort;
                    }
                    (*cif).vfp_reg_free = reg;
                }
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    (*cif).vfp_reg_free = 16 as ::core::ffi::c_ushort;
    (*cif).vfp_used = 0xffff as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn layout_vfp_args(mut cif: *mut ffi_cif) {
    let mut i: ::core::ffi::c_uint = 0;
    (*cif).vfp_used = 0 as ::core::ffi::c_int;
    (*cif).vfp_nargs = 0 as ::core::ffi::c_ushort;
    (*cif).vfp_reg_free = 0 as ::core::ffi::c_ushort;
    memset(
        &raw mut (*cif).vfp_args as *mut ::core::ffi::c_schar
            as *mut ::core::ffi::c_void,
        -(1 as ::core::ffi::c_int),
        16 as size_t,
    );
    i = 0 as ::core::ffi::c_uint;
    while i < (*cif).nargs {
        let mut h: ::core::ffi::c_int = vfp_type_p(*(*cif).arg_types.offset(i as isize));
        if h != 0 && place_vfp_arg(cif, h) == 1 as ::core::ffi::c_int {
            break;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_arch(
    mut tramp_size: *mut size_t,
    mut map_size: *mut size_t,
) -> *mut ::core::ffi::c_void {
    extern "C" {
        static mut trampoline_code_table: *mut ::core::ffi::c_void;
    }
    *tramp_size = ARM_TRAMP_SIZE as size_t;
    *map_size = ARM_TRAMP_MAP_SIZE as size_t;
    return &raw mut trampoline_code_table as *mut ::core::ffi::c_void;
}
