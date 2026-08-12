extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn abort() -> !;
    fn ffi_call_v9(
        cif: *mut ffi_cif,
        fn_0: Option<unsafe extern "C" fn() -> ()>,
        rvalue: *mut ::core::ffi::c_void,
        avalue: *mut *mut ::core::ffi::c_void,
        bytes: size_t,
        closure: *mut ::core::ffi::c_void,
    );
    fn ffi_closure_v9();
    fn ffi_go_closure_v9();
}
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_LAST_ABI: ffi_abi = 2;
pub const FFI_DEFAULT_ABI: ffi_abi = 1;
pub const FFI_V9: ffi_abi = 1;
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
    pub tramp: [::core::ffi::c_char; 24],
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
pub type UINT8 = ::core::ffi::c_uchar;
pub type SINT8 = ::core::ffi::c_schar;
pub type UINT16 = ::core::ffi::c_ushort;
pub type SINT16 = ::core::ffi::c_short;
pub type UINT32 = ::core::ffi::c_uint;
pub type SINT32 = ::core::ffi::c_int;
pub type UINT64 = ::core::ffi::c_ulong;
pub const FFI_TYPE_VOID: ::core::ffi::c_int = 0;
pub const FFI_TYPE_INT: ::core::ffi::c_int = 1;
pub const FFI_TYPE_FLOAT: ::core::ffi::c_int = 2;
pub const FFI_TYPE_DOUBLE: ::core::ffi::c_int = 3;
pub const FFI_TYPE_LONGDOUBLE: ::core::ffi::c_int = 4;
pub const FFI_TYPE_UINT8: ::core::ffi::c_int = 5;
pub const FFI_TYPE_SINT8: ::core::ffi::c_int = 6;
pub const FFI_TYPE_UINT16: ::core::ffi::c_int = 7;
pub const FFI_TYPE_SINT16: ::core::ffi::c_int = 8;
pub const FFI_TYPE_UINT32: ::core::ffi::c_int = 9;
pub const FFI_TYPE_SINT32: ::core::ffi::c_int = 10;
pub const FFI_TYPE_UINT64: ::core::ffi::c_int = 11;
pub const FFI_TYPE_SINT64: ::core::ffi::c_int = 12;
pub const FFI_TYPE_STRUCT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FFI_TYPE_POINTER: ::core::ffi::c_int = 14;
pub const FFI_TYPE_COMPLEX: ::core::ffi::c_int = 15;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" {
    fn ffi_flush_icache(p: *mut ::core::ffi::c_void);
}
pub const SPARC_RET_VOID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SPARC_RET_STRUCT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SPARC_RET_UINT8: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SPARC_RET_SINT8: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SPARC_RET_UINT16: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SPARC_RET_SINT16: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SPARC_RET_UINT32: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SP_V9_RET_SINT32: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SPARC_RET_INT64: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SPARC_RET_INT128: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SPARC_RET_F_8: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SPARC_RET_F_6: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SPARC_RET_F_4: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SPARC_RET_F_2: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SP_V9_RET_F_3: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SPARC_RET_F_1: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SPARC_FLAG_RET_IN_MEM: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SPARC_FLAG_FP_ARGS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const SPARC_SIZEMASK_SHIFT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
unsafe extern "C" fn ffi_struct_float_mask(
    mut outer_type: *mut ffi_type,
    mut size_mask: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut elts: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut t: *mut ffi_type = ::core::ptr::null_mut::<ffi_type>();
    if (*outer_type).type_0 as ::core::ffi::c_int == FFI_TYPE_COMPLEX {
        let mut m: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut tt: ::core::ffi::c_int = (**(*outer_type)
            .elements
            .offset(0 as ::core::ffi::c_int as isize))
        .type_0 as ::core::ffi::c_int;
        let mut z: size_t = (*outer_type).size;
        if tt == FFI_TYPE_FLOAT || tt == FFI_TYPE_DOUBLE || tt == FFI_TYPE_LONGDOUBLE {
            m = ((1 as ::core::ffi::c_int) << z.wrapping_div(4 as size_t))
                - 1 as ::core::ffi::c_int;
        }
        return ((m << 8 as ::core::ffi::c_int) as size_t | z) as ::core::ffi::c_int;
    }
    let mut current_block_13: u64;
    elts = (*outer_type).elements as *mut *mut ffi_type;
    loop {
        t = *elts;
        if t.is_null() {
            break;
        }
        let mut z_0: size_t = (*t).size;
        let mut o: ::core::ffi::c_int = 0;
        let mut m_0: ::core::ffi::c_int = 0;
        let mut tt_0: ::core::ffi::c_int = 0;
        size_mask = ((size_mask as size_t).wrapping_sub(1 as size_t)
            | ((*t).alignment as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(1 as size_t) as ::core::ffi::c_int;
        match (*t).type_0 as ::core::ffi::c_int {
            FFI_TYPE_STRUCT => {
                size_mask = ffi_struct_float_mask(t, size_mask);
                current_block_13 = 6483416627284290920;
            }
            FFI_TYPE_COMPLEX => {
                tt_0 = (**(*t).elements.offset(0 as ::core::ffi::c_int as isize)).type_0
                    as ::core::ffi::c_int;
                if tt_0 != FFI_TYPE_FLOAT && tt_0 != FFI_TYPE_DOUBLE && tt_0 != FFI_TYPE_LONGDOUBLE
                {
                    current_block_13 = 7149356873433890176;
                } else {
                    current_block_13 = 2498456585483420490;
                }
            }
            FFI_TYPE_FLOAT | FFI_TYPE_DOUBLE | FFI_TYPE_LONGDOUBLE => {
                current_block_13 = 2498456585483420490;
            }
            _ => {
                current_block_13 = 7149356873433890176;
            }
        }
        match current_block_13 {
            2498456585483420490 => {
                m_0 = ((1 as ::core::ffi::c_int) << z_0.wrapping_div(4 as size_t))
                    - 1 as ::core::ffi::c_int;
                o = size_mask >> 2 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int;
                size_mask |= m_0 << o + 8 as ::core::ffi::c_int;
                current_block_13 = 7149356873433890176;
            }
            _ => {}
        }
        match current_block_13 {
            7149356873433890176 => {
                size_mask = (size_mask as size_t).wrapping_add(z_0) as ::core::ffi::c_int
                    as ::core::ffi::c_int;
            }
            _ => {}
        }
        elts = elts.offset(1);
    }
    size_mask = ((size_mask as size_t).wrapping_sub(1 as size_t)
        | ((*outer_type).alignment as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
        .wrapping_add(1 as size_t) as ::core::ffi::c_int;
    return size_mask;
}
unsafe extern "C" fn ffi_struct_float_merge(
    mut size_mask: ::core::ffi::c_int,
    mut vi: *mut ::core::ffi::c_void,
    mut vf: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut size: ::core::ffi::c_int = size_mask & 0xff as ::core::ffi::c_int;
    let mut mask: ::core::ffi::c_int = size_mask >> 8 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = size >> 2 as ::core::ffi::c_int;
    if mask == 0 as ::core::ffi::c_int {
        return vi;
    } else if mask == ((1 as ::core::ffi::c_int) << n) - 1 as ::core::ffi::c_int {
        return vf;
    } else {
        let mut wi: *mut ::core::ffi::c_uint = vi as *mut ::core::ffi::c_uint;
        let mut wf: *mut ::core::ffi::c_uint = vf as *mut ::core::ffi::c_uint;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            if mask >> i & 1 as ::core::ffi::c_int != 0 {
                *wi.offset(i as isize) = *wf.offset(i as isize);
            }
            i += 1;
        }
        return vi;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ffi_struct_float_copy(
    mut size_mask: ::core::ffi::c_int,
    mut vd: *mut ::core::ffi::c_void,
    mut vi: *mut ::core::ffi::c_void,
    mut vf: *mut ::core::ffi::c_void,
) {
    let mut size: ::core::ffi::c_int = size_mask & 0xff as ::core::ffi::c_int;
    let mut mask: ::core::ffi::c_int = size_mask >> 8 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = size >> 2 as ::core::ffi::c_int;
    if !(mask == 0 as ::core::ffi::c_int) {
        if mask == ((1 as ::core::ffi::c_int) << n) - 1 as ::core::ffi::c_int {
            vi = vf;
        } else {
            let mut wd: *mut ::core::ffi::c_uint = vd as *mut ::core::ffi::c_uint;
            let mut wi: *mut ::core::ffi::c_uint = vi as *mut ::core::ffi::c_uint;
            let mut wf: *mut ::core::ffi::c_uint = vf as *mut ::core::ffi::c_uint;
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < n {
                *wd.offset(i as isize) = *if mask >> i & 1 as ::core::ffi::c_int != 0 {
                    wf
                } else {
                    wi
                }
                .offset(i as isize);
                i += 1;
            }
            return;
        }
    }
    memcpy(vd, vi, size as size_t);
}
unsafe extern "C" fn ffi_prep_cif_machdep_core(mut cif: *mut ffi_cif) -> ffi_status {
    let mut rtype: *mut ffi_type = (*cif).rtype;
    let mut rtt: ::core::ffi::c_int = (*rtype).type_0 as ::core::ffi::c_int;
    let mut bytes: size_t = 0 as size_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    match rtt {
        FFI_TYPE_VOID => {
            flags = SPARC_RET_VOID;
        }
        FFI_TYPE_FLOAT => {
            flags = SPARC_RET_F_1;
        }
        FFI_TYPE_DOUBLE => {
            flags = SPARC_RET_F_2;
        }
        FFI_TYPE_LONGDOUBLE => {
            flags = SPARC_RET_F_4;
        }
        FFI_TYPE_COMPLEX | FFI_TYPE_STRUCT => {
            if (*rtype).size > 32 as size_t {
                flags = SPARC_RET_VOID | SPARC_FLAG_RET_IN_MEM;
                bytes = 8 as size_t;
            } else {
                let mut size_mask: ::core::ffi::c_int =
                    ffi_struct_float_mask(rtype, 0 as ::core::ffi::c_int);
                let mut word_size: ::core::ffi::c_int =
                    size_mask >> 2 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int;
                let mut all_mask: ::core::ffi::c_int =
                    ((1 as ::core::ffi::c_int) << word_size) - 1 as ::core::ffi::c_int;
                let mut fp_mask: ::core::ffi::c_int = size_mask >> 8 as ::core::ffi::c_int;
                flags = size_mask << SPARC_SIZEMASK_SHIFT | SPARC_RET_STRUCT;
                if fp_mask == 0 as ::core::ffi::c_int {
                    if (*rtype).alignment as ::core::ffi::c_int >= 8 as ::core::ffi::c_int {
                        if (*rtype).size == 8 as size_t {
                            flags = SPARC_RET_INT64;
                        } else if (*rtype).size == 16 as size_t {
                            flags = SPARC_RET_INT128;
                        }
                    }
                } else if fp_mask == all_mask {
                    match word_size {
                        1 => {
                            flags = SPARC_RET_F_1;
                        }
                        2 => {
                            flags = SPARC_RET_F_2;
                        }
                        3 => {
                            flags = SP_V9_RET_F_3;
                        }
                        4 => {
                            flags = SPARC_RET_F_4;
                        }
                        6 => {
                            flags = SPARC_RET_F_6;
                        }
                        8 => {
                            flags = SPARC_RET_F_8;
                        }
                        _ => {}
                    }
                }
            }
        }
        FFI_TYPE_SINT8 => {
            flags = SPARC_RET_SINT8;
        }
        FFI_TYPE_UINT8 => {
            flags = SPARC_RET_UINT8;
        }
        FFI_TYPE_SINT16 => {
            flags = SPARC_RET_SINT16;
        }
        FFI_TYPE_UINT16 => {
            flags = SPARC_RET_UINT16;
        }
        FFI_TYPE_INT | FFI_TYPE_SINT32 => {
            flags = SP_V9_RET_SINT32;
        }
        FFI_TYPE_UINT32 => {
            flags = SPARC_RET_UINT32;
        }
        FFI_TYPE_SINT64 | FFI_TYPE_UINT64 | FFI_TYPE_POINTER => {
            flags = SPARC_RET_INT64;
        }
        _ => {
            abort();
        }
    }
    bytes = 0 as size_t;
    i = 0 as ::core::ffi::c_int;
    n = (*cif).nargs as ::core::ffi::c_int;
    while i < n {
        let mut ty: *mut ffi_type = *(*cif).arg_types.offset(i as isize);
        let mut z: size_t = (*ty).size;
        let mut a: size_t = (*ty).alignment as size_t;
        let mut current_block_38: u64;
        match (*ty).type_0 as ::core::ffi::c_int {
            FFI_TYPE_COMPLEX | FFI_TYPE_STRUCT => {
                if z > 16 as size_t {
                    z = 8 as size_t;
                    a = z;
                    current_block_38 = 8545136480011357681;
                } else if bytes >= (16 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as size_t {
                    current_block_38 = 8545136480011357681;
                } else if ffi_struct_float_mask(ty, 0 as ::core::ffi::c_int)
                    & 0xff00 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    current_block_38 = 8545136480011357681;
                } else {
                    current_block_38 = 3532678959753507824;
                }
            }
            FFI_TYPE_FLOAT | FFI_TYPE_DOUBLE | FFI_TYPE_LONGDOUBLE => {
                current_block_38 = 3532678959753507824;
            }
            _ => {
                current_block_38 = 8545136480011357681;
            }
        }
        match current_block_38 {
            3532678959753507824 => {
                flags |= SPARC_FLAG_FP_ARGS;
            }
            _ => {}
        }
        bytes = (bytes.wrapping_sub(1 as size_t) | a.wrapping_sub(1 as size_t))
            .wrapping_add(1 as size_t);
        bytes = bytes.wrapping_add(
            (z.wrapping_sub(1 as size_t)
                | (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_add(1 as size_t),
        );
        i += 1;
    }
    if bytes < (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as size_t {
        bytes = (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as size_t;
    }
    bytes = (bytes.wrapping_sub(1 as size_t)
        | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
        .wrapping_add(1 as size_t);
    bytes = bytes.wrapping_add(
        (8 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
            + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as size_t,
    );
    (*cif).bytes = bytes as ::core::ffi::c_uint;
    (*cif).flags = flags as ::core::ffi::c_uint;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep(mut cif: *mut ffi_cif) -> ffi_status {
    (*cif).nfixedargs = (*cif).nargs;
    return ffi_prep_cif_machdep_core(cif);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep_var(
    mut cif: *mut ffi_cif,
    mut nfixedargs: ::core::ffi::c_uint,
    mut ntotalargs: ::core::ffi::c_uint,
) -> ffi_status {
    (*cif).nfixedargs = nfixedargs;
    return ffi_prep_cif_machdep_core(cif);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_args_v9(
    mut cif: *mut ffi_cif,
    mut argp: *mut ::core::ffi::c_ulong,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p_arg: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut flags: ::core::ffi::c_int = (*cif).flags as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut nargs: ::core::ffi::c_int = 0;
    if rvalue.is_null() {
        if flags & SPARC_FLAG_RET_IN_MEM != 0 {
            rvalue = (argp as *mut ::core::ffi::c_char).offset((*cif).bytes as isize)
                as *mut ::core::ffi::c_void;
        } else {
            flags = SPARC_RET_VOID;
        }
    }
    if flags & SPARC_FLAG_RET_IN_MEM != 0 {
        let fresh1 = argp;
        argp = argp.offset(1);
        *fresh1 = rvalue as ::core::ffi::c_ulong;
    }
    p_arg = (*cif).arg_types;
    i = 0 as ::core::ffi::c_int;
    nargs = (*cif).nargs as ::core::ffi::c_int;
    while i < nargs {
        let mut ty: *mut ffi_type = *p_arg.offset(i as isize);
        let mut a: *mut ::core::ffi::c_void = *avalue.offset(i as isize);
        let mut z: size_t = 0;
        match (*ty).type_0 as ::core::ffi::c_int {
            FFI_TYPE_SINT8 => {
                let fresh2 = argp;
                argp = argp.offset(1);
                *fresh2 = *(a as *mut SINT8) as ::core::ffi::c_ulong;
            }
            FFI_TYPE_UINT8 => {
                let fresh3 = argp;
                argp = argp.offset(1);
                *fresh3 = *(a as *mut UINT8) as ::core::ffi::c_ulong;
            }
            FFI_TYPE_SINT16 => {
                let fresh4 = argp;
                argp = argp.offset(1);
                *fresh4 = *(a as *mut SINT16) as ::core::ffi::c_ulong;
            }
            FFI_TYPE_UINT16 => {
                let fresh5 = argp;
                argp = argp.offset(1);
                *fresh5 = *(a as *mut UINT16) as ::core::ffi::c_ulong;
            }
            FFI_TYPE_INT | FFI_TYPE_SINT32 => {
                let fresh6 = argp;
                argp = argp.offset(1);
                *fresh6 = *(a as *mut SINT32) as ::core::ffi::c_ulong;
            }
            FFI_TYPE_UINT32 => {
                let fresh7 = argp;
                argp = argp.offset(1);
                *fresh7 = *(a as *mut UINT32) as ::core::ffi::c_ulong;
            }
            FFI_TYPE_SINT64 | FFI_TYPE_UINT64 | FFI_TYPE_POINTER => {
                let fresh8 = argp;
                argp = argp.offset(1);
                *fresh8 = *(a as *mut UINT64) as ::core::ffi::c_ulong;
            }
            FFI_TYPE_FLOAT => {
                flags |= SPARC_FLAG_FP_ARGS;
                let fresh9 = argp;
                argp = argp.offset(1);
                *fresh9 = *(a as *mut UINT32) as ::core::ffi::c_ulong;
            }
            FFI_TYPE_DOUBLE => {
                flags |= SPARC_FLAG_FP_ARGS;
                let fresh10 = argp;
                argp = argp.offset(1);
                *fresh10 = *(a as *mut UINT64) as ::core::ffi::c_ulong;
            }
            FFI_TYPE_LONGDOUBLE | FFI_TYPE_COMPLEX | FFI_TYPE_STRUCT => {
                z = (*ty).size;
                if z > 16 as size_t {
                    let fresh11 = argp;
                    argp = argp.offset(1);
                    *fresh11 = a as ::core::ffi::c_ulong;
                } else {
                    if argp as ::core::ffi::c_ulong & 15 as ::core::ffi::c_ulong != 0
                        && (*ty).alignment as ::core::ffi::c_int > 8 as ::core::ffi::c_int
                    {
                        argp = argp.offset(1);
                    }
                    memcpy(argp as *mut ::core::ffi::c_void, a, z);
                    argp = argp.offset(
                        (z.wrapping_sub(1 as size_t)
                            | (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                            .wrapping_add(1 as size_t)
                            .wrapping_div(8 as size_t) as isize,
                    );
                }
            }
            _ => {
                abort();
            }
        }
        i += 1;
    }
    return flags;
}
unsafe extern "C" fn ffi_call_int(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
    mut closure: *mut ::core::ffi::c_void,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut bytes: size_t = (*cif).bytes as size_t;
    let mut i: size_t = 0;
    let mut nargs: size_t = (*cif).nargs as size_t;
    let mut arg_types: *mut *mut ffi_type = (*cif).arg_types;
    if rvalue.is_null() && (*cif).flags & SPARC_FLAG_RET_IN_MEM as ::core::ffi::c_uint != 0 {
        bytes = bytes.wrapping_add(
            ((*(*cif).rtype).size.wrapping_sub(1 as size_t)
                | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_add(1 as size_t),
        );
    }
    i = 0 as size_t;
    while i < nargs {
        let mut at: *mut ffi_type = *arg_types.offset(i as isize);
        let mut size: ::core::ffi::c_int = (*at).size as ::core::ffi::c_int;
        if (*at).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT && size > 4 as ::core::ffi::c_int {
            alloca_allocations.push(::std::vec::from_elem(
                0,
                size as ::core::ffi::c_ulong as usize,
            ));
            let mut argcopy: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                argcopy as *mut ::core::ffi::c_void,
                *avalue.offset(i as isize),
                size as size_t,
            );
            let ref mut fresh0 = *avalue.offset(i as isize);
            *fresh0 = argcopy as *mut ::core::ffi::c_void;
        }
        i = i.wrapping_add(1);
    }
    ffi_call_v9(cif, fn_0, rvalue, avalue, bytes.wrapping_neg(), closure);
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
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_closure_loc(
    mut closure: *mut ffi_closure,
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
    mut codeloc: *mut ::core::ffi::c_void,
) -> ffi_status {
    let mut tramp: *mut ::core::ffi::c_uint =
        (&raw mut (*closure).c2rust_unnamed.tramp as *mut ::core::ffi::c_char)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_uint;
    let mut fn_0: ::core::ffi::c_ulong = 0;
    if (*cif).abi as ::core::ffi::c_uint != FFI_V9 as ::core::ffi::c_int as ::core::ffi::c_uint {
        return FFI_BAD_ABI;
    }
    fn_0 = ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, ::core::ffi::c_ulong>(
        Some(ffi_closure_v9 as unsafe extern "C" fn() -> ()),
    );
    *tramp.offset(0 as ::core::ffi::c_int as isize) = 0x83414000 as ::core::ffi::c_uint;
    *tramp.offset(1 as ::core::ffi::c_int as isize) = 0xca586010 as ::core::ffi::c_uint;
    *tramp.offset(2 as ::core::ffi::c_int as isize) = 0x81c14000 as ::core::ffi::c_uint;
    *tramp.offset(3 as ::core::ffi::c_int as isize) =
        0x1000000 as ::core::ffi::c_int as ::core::ffi::c_uint;
    *(tramp.offset(4 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uint
        as *mut ::core::ffi::c_ulong) = fn_0;
    (*closure).cif = cif;
    (*closure).fun = fun;
    (*closure).user_data = user_data;
    ffi_flush_icache(closure as *mut ::core::ffi::c_void);
    return FFI_OK;
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
    if (*cif).abi as ::core::ffi::c_uint != FFI_V9 as ::core::ffi::c_int as ::core::ffi::c_uint {
        return FFI_BAD_ABI;
    }
    (*closure).tramp = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        *mut ::core::ffi::c_void,
    >(Some(ffi_go_closure_v9 as unsafe extern "C" fn() -> ()));
    (*closure).cif = cif;
    (*closure).fun = fun;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_sparc_inner_v9(
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
    mut rvalue: *mut ::core::ffi::c_void,
    mut gpr: *mut ::core::ffi::c_ulong,
    mut fpr: *mut ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut arg_types: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut avalue: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut i: ::core::ffi::c_int = 0;
    let mut argn: ::core::ffi::c_int = 0;
    let mut argx: ::core::ffi::c_int = 0;
    let mut nargs: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut nfixedargs: ::core::ffi::c_int = 0;
    arg_types = (*cif).arg_types;
    nargs = (*cif).nargs as ::core::ffi::c_int;
    flags = (*cif).flags as ::core::ffi::c_int;
    nfixedargs = (*cif).nfixedargs as ::core::ffi::c_int;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (nargs as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
            as usize,
    ));
    avalue = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_void;
    if flags & SPARC_FLAG_RET_IN_MEM != 0 {
        rvalue = *gpr.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void;
        argn = 1 as ::core::ffi::c_int;
    } else {
        argn = 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < nargs {
        let mut named: ::core::ffi::c_int = (i < nfixedargs) as ::core::ffi::c_int;
        let mut ty: *mut ffi_type = *arg_types.offset(i as isize);
        let mut a: *mut ::core::ffi::c_void =
            gpr.offset(argn as isize) as *mut ::core::ffi::c_ulong as *mut ::core::ffi::c_void;
        let mut z: size_t = 0;
        argx = argn + 1 as ::core::ffi::c_int;
        match (*ty).type_0 as ::core::ffi::c_int {
            FFI_TYPE_COMPLEX | FFI_TYPE_STRUCT => {
                z = (*ty).size;
                if z > 16 as size_t {
                    a = *(a as *mut *mut ::core::ffi::c_void);
                } else {
                    argx = (argn as size_t).wrapping_add(
                        (z.wrapping_sub(1 as size_t)
                            | (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                            .wrapping_add(1 as size_t)
                            .wrapping_div(8 as size_t),
                    ) as ::core::ffi::c_int;
                    if named != 0 && argn < 16 as ::core::ffi::c_int {
                        let mut size_mask: ::core::ffi::c_int =
                            ffi_struct_float_mask(ty, 0 as ::core::ffi::c_int);
                        let mut argn_mask: ::core::ffi::c_int =
                            0xffff00 as ::core::ffi::c_int >> argn & 0xff00 as ::core::ffi::c_int;
                        size_mask = size_mask & 0xff as ::core::ffi::c_int | size_mask & argn_mask;
                        a = ffi_struct_float_merge(
                            size_mask,
                            gpr.offset(argn as isize) as *mut ::core::ffi::c_void,
                            fpr.offset(argn as isize) as *mut ::core::ffi::c_void,
                        );
                    }
                }
            }
            FFI_TYPE_LONGDOUBLE => {
                argn = ((argn as size_t).wrapping_sub(1 as size_t)
                    | (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(1 as size_t) as ::core::ffi::c_int;
                a = (if named != 0 && argn < 16 as ::core::ffi::c_int {
                    fpr
                } else {
                    gpr
                })
                .offset(argn as isize) as *mut ::core::ffi::c_void;
                argx = argn + 2 as ::core::ffi::c_int;
            }
            FFI_TYPE_DOUBLE => {
                if named != 0 && argn < 16 as ::core::ffi::c_int {
                    a = fpr.offset(argn as isize) as *mut ::core::ffi::c_void;
                }
            }
            FFI_TYPE_FLOAT => {
                if named != 0 && argn < 16 as ::core::ffi::c_int {
                    a = fpr.offset(argn as isize) as *mut ::core::ffi::c_void;
                }
                a = a.offset(4 as ::core::ffi::c_int as isize);
            }
            FFI_TYPE_UINT64 | FFI_TYPE_SINT64 | FFI_TYPE_POINTER => {}
            FFI_TYPE_INT | FFI_TYPE_UINT32 | FFI_TYPE_SINT32 => {
                a = a.offset(4 as ::core::ffi::c_int as isize);
            }
            FFI_TYPE_UINT16 | FFI_TYPE_SINT16 => {
                a = a.offset(6 as ::core::ffi::c_int as isize);
            }
            FFI_TYPE_UINT8 | FFI_TYPE_SINT8 => {
                a = a.offset(7 as ::core::ffi::c_int as isize);
            }
            _ => {
                abort();
            }
        }
        let ref mut fresh12 = *avalue.offset(i as isize);
        *fresh12 = a;
        i += 1;
        argn = argx;
    }
    fun.expect("non-null function pointer")(cif, rvalue, avalue, user_data);
    return flags;
}
