extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ffi_tramp_is_present(closure: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn ffi_tramp_set_parms(
        tramp: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        code: *mut ::core::ffi::c_void,
    );
    fn ffi_call_SYSV(
        _: *mut call_frame,
        _: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_void,
        fn_0: Option<unsafe extern "C" fn() -> ()>,
        _: *mut ::core::ffi::c_void,
    );
    // C2Rust 0.22.1 cannot translate __builtin_frame_address(0). The exact
    // configured function is retained as GCC-generated s390x assembly.
    fn ffi_call_int(
        cif: *mut ffi_cif,
        fn_0: Option<unsafe extern "C" fn() -> ()>,
        rvalue: *mut ::core::ffi::c_void,
        avalue: *mut *mut ::core::ffi::c_void,
        closure: *mut ::core::ffi::c_void,
    );
    fn ffi_closure_SYSV();
    fn ffi_go_closure_SYSV();
}
pub type ffi_arg = ::core::ffi::c_ulong;
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 1;
pub const FFI_LAST_ABI: ffi_abi = 2;
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
    pub tramp: [::core::ffi::c_char; 32],
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
    pub back_chain: *mut ::core::ffi::c_void,
    pub eos: *mut ::core::ffi::c_void,
    pub gpr_args: [::core::ffi::c_ulong; 5],
    pub gpr_save: [::core::ffi::c_ulong; 9],
    pub fpr_args: [::core::ffi::c_ulonglong; 4],
}
pub type uintptr_t = usize;
pub type UINT64 = ::core::ffi::c_ulong;
pub type UINT32 = ::core::ffi::c_uint;
pub type SINT32 = ::core::ffi::c_int;
pub type UINT16 = ::core::ffi::c_ushort;
pub type SINT16 = ::core::ffi::c_short;
pub type UINT8 = ::core::ffi::c_uchar;
pub type SINT8 = ::core::ffi::c_schar;
pub const FFI_TYPE_VOID: ::core::ffi::c_int = 0;
pub const FFI_TYPE_INT: ::core::ffi::c_int = 1;
pub const FFI_TYPE_FLOAT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FFI_TYPE_DOUBLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FFI_TYPE_LONGDOUBLE: ::core::ffi::c_int = 4;
pub const FFI_TYPE_UINT8: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FFI_TYPE_SINT8: ::core::ffi::c_int = 6;
pub const FFI_TYPE_UINT16: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const FFI_TYPE_SINT16: ::core::ffi::c_int = 8;
pub const FFI_TYPE_UINT32: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const FFI_TYPE_SINT32: ::core::ffi::c_int = 10;
pub const FFI_TYPE_UINT64: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const FFI_TYPE_SINT64: ::core::ffi::c_int = 12;
pub const FFI_TYPE_STRUCT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FFI_TYPE_POINTER: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const FFI_TYPE_COMPLEX: ::core::ffi::c_int = 15;
pub const FFI_TYPE_UINT128: ::core::ffi::c_int = 16;
pub const FFI_TYPE_SINT128: ::core::ffi::c_int = 17;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const FFI390_RET_DOUBLE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FFI390_RET_FLOAT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FFI390_RET_INT64: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FFI390_RET_VOID: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FFI360_RET_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const FFI390_RET_IN_MEM: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const FFI390_RET_STRUCT: ::core::ffi::c_int = FFI390_RET_VOID | FFI390_RET_IN_MEM;
pub const FFI390_TRAMP_MAP_SHIFT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const FFI390_TRAMP_MAP_SIZE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << FFI390_TRAMP_MAP_SHIFT;
pub const FFI390_TRAMP_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MAX_GPRARGS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MAX_FPRARGS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn ffi_check_struct_type(mut arg: *mut ffi_type) -> ::core::ffi::c_int {
    let mut size: size_t = (*arg).size;
    while (*arg).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT
        && !(*(*arg).elements.offset(0 as ::core::ffi::c_int as isize)).is_null()
        && (*(*arg).elements.offset(1 as ::core::ffi::c_int as isize)).is_null()
    {
        arg = *(*arg).elements.offset(0 as ::core::ffi::c_int as isize) as *mut ffi_type;
    }
    match size {
        1 => return FFI_TYPE_UINT8,
        2 => return FFI_TYPE_UINT16,
        4 => {
            if (*arg).type_0 as ::core::ffi::c_int == FFI_TYPE_FLOAT {
                return FFI_TYPE_FLOAT;
            } else {
                return FFI_TYPE_UINT32;
            }
        }
        8 => {
            if (*arg).type_0 as ::core::ffi::c_int == FFI_TYPE_DOUBLE {
                return FFI_TYPE_DOUBLE;
            } else {
                return FFI_TYPE_UINT64;
            }
        }
        _ => {}
    }
    return FFI_TYPE_POINTER;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep(mut cif: *mut ffi_cif) -> ffi_status {
    let mut struct_size: size_t = 0 as size_t;
    let mut n_gpr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n_fpr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n_ov: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ptr: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut i: ::core::ffi::c_int = 0;
    match (*(*cif).rtype).type_0 as ::core::ffi::c_int {
        FFI_TYPE_VOID => {
            (*cif).flags = FFI390_RET_VOID as ::core::ffi::c_uint;
        }
        FFI_TYPE_STRUCT | FFI_TYPE_COMPLEX | FFI_TYPE_SINT128 | FFI_TYPE_UINT128
        | FFI_TYPE_LONGDOUBLE => {
            (*cif).flags = FFI390_RET_STRUCT as ::core::ffi::c_uint;
            n_gpr += 1;
        }
        FFI_TYPE_FLOAT => {
            (*cif).flags = FFI390_RET_FLOAT as ::core::ffi::c_uint;
        }
        FFI_TYPE_DOUBLE => {
            (*cif).flags = FFI390_RET_DOUBLE as ::core::ffi::c_uint;
        }
        FFI_TYPE_UINT64 | FFI_TYPE_SINT64 => {
            (*cif).flags = FFI390_RET_INT64 as ::core::ffi::c_uint;
        }
        FFI_TYPE_POINTER | FFI_TYPE_INT | FFI_TYPE_UINT32 | FFI_TYPE_SINT32 | FFI_TYPE_UINT16
        | FFI_TYPE_SINT16 | FFI_TYPE_UINT8 | FFI_TYPE_SINT8 => {
            (*cif).flags = FFI390_RET_INT64 as ::core::ffi::c_uint;
        }
        _ => {}
    }
    ptr = (*cif).arg_types;
    i = (*cif).nargs as ::core::ffi::c_int;
    while i > 0 as ::core::ffi::c_int {
        let mut type_0: ::core::ffi::c_int = (**ptr).type_0 as ::core::ffi::c_int;
        let mut current_block_12: u64;
        match type_0 {
            FFI_TYPE_STRUCT => {
                type_0 = ffi_check_struct_type(*ptr);
                if type_0 != FFI_TYPE_POINTER {
                    current_block_12 = 11057878835866523405;
                } else {
                    current_block_12 = 15608676824980778792;
                }
            }
            FFI_TYPE_COMPLEX | FFI_TYPE_SINT128 | FFI_TYPE_UINT128 | FFI_TYPE_LONGDOUBLE => {
                current_block_12 = 15608676824980778792;
            }
            _ => {
                current_block_12 = 11057878835866523405;
            }
        }
        match current_block_12 {
            15608676824980778792 => {
                type_0 = FFI_TYPE_POINTER;
                struct_size = struct_size.wrapping_add(
                    (**ptr).size.wrapping_add(15 as size_t) & !(15 as ::core::ffi::c_int) as size_t,
                );
            }
            _ => {}
        }
        match type_0 {
            FFI_TYPE_DOUBLE => {
                if n_fpr < MAX_FPRARGS {
                    n_fpr += 1;
                } else {
                    n_ov = (n_ov as ::core::ffi::c_ulong).wrapping_add(
                        (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                            .wrapping_div(::core::mem::size_of::<::core::ffi::c_long>() as usize)
                            as ::core::ffi::c_ulong,
                    ) as ::core::ffi::c_int as ::core::ffi::c_int;
                }
            }
            FFI_TYPE_FLOAT => {
                if n_fpr < MAX_FPRARGS {
                    n_fpr += 1;
                } else {
                    n_ov += 1;
                }
            }
            _ => {
                if n_gpr < MAX_GPRARGS {
                    n_gpr += 1;
                } else {
                    n_ov += 1;
                }
            }
        }
        i -= 1;
        ptr = ptr.offset(1);
    }
    (*cif).bytes = ((n_ov as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as usize)
        .wrapping_add(15 as usize)
        & !(15 as ::core::ffi::c_int) as usize)
        .wrapping_add(struct_size as usize) as ::core::ffi::c_uint;
    return FFI_OK;
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
pub unsafe extern "C" fn ffi_closure_helper_SYSV(
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
    mut p_gpr: *mut ::core::ffi::c_ulong,
    mut p_fpr: *mut ::core::ffi::c_ulonglong,
    mut p_ov: *mut ::core::ffi::c_ulong,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut ret_buffer: ::core::ffi::c_ulonglong = 0;
    let mut rvalue: *mut ::core::ffi::c_void = &raw mut ret_buffer as *mut ::core::ffi::c_void;
    let mut avalue: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut p_arg: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut n_gpr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n_fpr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n_ov: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ptr: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut i: ::core::ffi::c_int = 0;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        ((*cif).nargs as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
            as usize,
    ));
    avalue = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_void;
    p_arg = avalue;
    if (*cif).flags & FFI390_RET_IN_MEM as ::core::ffi::c_uint != 0 {
        let fresh0 = n_gpr;
        n_gpr = n_gpr + 1;
        rvalue = *p_gpr.offset(fresh0 as isize) as *mut ::core::ffi::c_void;
    }
    ptr = (*cif).arg_types;
    i = (*cif).nargs as ::core::ffi::c_int;
    while i > 0 as ::core::ffi::c_int {
        let mut deref_struct_pointer: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut type_0: ::core::ffi::c_int = (**ptr).type_0 as ::core::ffi::c_int;
        if type_0 == FFI_TYPE_LONGDOUBLE {
            type_0 = FFI_TYPE_STRUCT;
        }
        if type_0 == FFI_TYPE_STRUCT || type_0 == FFI_TYPE_COMPLEX {
            if type_0 == FFI_TYPE_COMPLEX {
                type_0 = FFI_TYPE_POINTER;
            } else {
                type_0 = ffi_check_struct_type(*ptr);
            }
            if type_0 == FFI_TYPE_POINTER {
                deref_struct_pointer = 1 as ::core::ffi::c_int;
            }
        }
        if type_0 == FFI_TYPE_POINTER {
            type_0 = FFI_TYPE_UINT64;
        }
        match type_0 {
            FFI_TYPE_DOUBLE => {
                if n_fpr < MAX_FPRARGS {
                    let fresh1 = n_fpr;
                    n_fpr = n_fpr + 1;
                    *p_arg = p_fpr.offset(fresh1 as isize) as *mut ::core::ffi::c_ulonglong
                        as *mut ::core::ffi::c_void;
                } else {
                    *p_arg = p_ov.offset(n_ov as isize) as *mut ::core::ffi::c_ulong
                        as *mut ::core::ffi::c_void;
                    n_ov = (n_ov as ::core::ffi::c_ulong).wrapping_add(
                        (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                            .wrapping_div(::core::mem::size_of::<::core::ffi::c_long>() as usize)
                            as ::core::ffi::c_ulong,
                    ) as ::core::ffi::c_int as ::core::ffi::c_int;
                }
            }
            FFI_TYPE_FLOAT => {
                if n_fpr < MAX_FPRARGS {
                    let fresh2 = n_fpr;
                    n_fpr = n_fpr + 1;
                    *p_arg = p_fpr.offset(fresh2 as isize) as *mut ::core::ffi::c_ulonglong
                        as *mut ::core::ffi::c_void;
                } else {
                    let fresh3 = n_ov;
                    n_ov = n_ov + 1;
                    *p_arg = (p_ov.offset(fresh3 as isize) as *mut ::core::ffi::c_ulong
                        as *mut ::core::ffi::c_char)
                        .offset(::core::mem::size_of::<::core::ffi::c_long>() as usize as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void;
                }
            }
            FFI_TYPE_UINT64 | FFI_TYPE_SINT64 => {
                if n_gpr < MAX_GPRARGS {
                    let fresh4 = n_gpr;
                    n_gpr = n_gpr + 1;
                    *p_arg = p_gpr.offset(fresh4 as isize) as *mut ::core::ffi::c_ulong
                        as *mut ::core::ffi::c_void;
                } else {
                    let fresh5 = n_ov;
                    n_ov = n_ov + 1;
                    *p_arg = p_ov.offset(fresh5 as isize) as *mut ::core::ffi::c_ulong
                        as *mut ::core::ffi::c_void;
                }
            }
            FFI_TYPE_INT | FFI_TYPE_UINT32 | FFI_TYPE_SINT32 => {
                if n_gpr < MAX_GPRARGS {
                    let fresh6 = n_gpr;
                    n_gpr = n_gpr + 1;
                    *p_arg = (p_gpr.offset(fresh6 as isize) as *mut ::core::ffi::c_ulong
                        as *mut ::core::ffi::c_char)
                        .offset(::core::mem::size_of::<::core::ffi::c_long>() as usize as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void;
                } else {
                    let fresh7 = n_ov;
                    n_ov = n_ov + 1;
                    *p_arg = (p_ov.offset(fresh7 as isize) as *mut ::core::ffi::c_ulong
                        as *mut ::core::ffi::c_char)
                        .offset(::core::mem::size_of::<::core::ffi::c_long>() as usize as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void;
                }
            }
            FFI_TYPE_UINT16 | FFI_TYPE_SINT16 => {
                if n_gpr < MAX_GPRARGS {
                    let fresh8 = n_gpr;
                    n_gpr = n_gpr + 1;
                    *p_arg = (p_gpr.offset(fresh8 as isize) as *mut ::core::ffi::c_ulong
                        as *mut ::core::ffi::c_char)
                        .offset(::core::mem::size_of::<::core::ffi::c_long>() as usize as isize)
                        .offset(-(2 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void;
                } else {
                    let fresh9 = n_ov;
                    n_ov = n_ov + 1;
                    *p_arg = (p_ov.offset(fresh9 as isize) as *mut ::core::ffi::c_ulong
                        as *mut ::core::ffi::c_char)
                        .offset(::core::mem::size_of::<::core::ffi::c_long>() as usize as isize)
                        .offset(-(2 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void;
                }
            }
            FFI_TYPE_UINT8 | FFI_TYPE_SINT8 => {
                if n_gpr < MAX_GPRARGS {
                    let fresh10 = n_gpr;
                    n_gpr = n_gpr + 1;
                    *p_arg = (p_gpr.offset(fresh10 as isize) as *mut ::core::ffi::c_ulong
                        as *mut ::core::ffi::c_char)
                        .offset(::core::mem::size_of::<::core::ffi::c_long>() as usize as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void;
                } else {
                    let fresh11 = n_ov;
                    n_ov = n_ov + 1;
                    *p_arg = (p_ov.offset(fresh11 as isize) as *mut ::core::ffi::c_ulong
                        as *mut ::core::ffi::c_char)
                        .offset(::core::mem::size_of::<::core::ffi::c_long>() as usize as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void;
                }
            }
            _ => {}
        }
        if deref_struct_pointer != 0 {
            *p_arg = *(*p_arg as *mut *mut ::core::ffi::c_void);
        }
        i -= 1;
        p_arg = p_arg.offset(1);
        ptr = ptr.offset(1);
    }
    fun.expect("non-null function pointer")(cif, rvalue, avalue, user_data);
    match (*(*cif).rtype).type_0 as ::core::ffi::c_int {
        FFI_TYPE_FLOAT => {
            *p_fpr.offset(0 as ::core::ffi::c_int as isize) =
                ((*(rvalue as *mut ::core::ffi::c_uint) as ::core::ffi::c_longlong)
                    << 32 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong;
        }
        FFI_TYPE_DOUBLE => {
            *p_fpr.offset(0 as ::core::ffi::c_int as isize) =
                *(rvalue as *mut ::core::ffi::c_ulonglong);
        }
        FFI_TYPE_UINT64 | FFI_TYPE_SINT64 => {
            *p_gpr.offset(0 as ::core::ffi::c_int as isize) =
                *(rvalue as *mut ::core::ffi::c_ulong);
        }
        FFI_TYPE_POINTER | FFI_TYPE_UINT32 | FFI_TYPE_UINT16 | FFI_TYPE_UINT8 => {
            *p_gpr.offset(0 as ::core::ffi::c_int as isize) =
                *(rvalue as *mut ::core::ffi::c_ulong);
        }
        FFI_TYPE_INT | FFI_TYPE_SINT32 | FFI_TYPE_SINT16 | FFI_TYPE_SINT8 => {
            *p_gpr.offset(0 as ::core::ffi::c_int as isize) =
                *(rvalue as *mut ::core::ffi::c_long) as ::core::ffi::c_ulong;
        }
        FFI_TYPE_VOID | FFI_TYPE_STRUCT | FFI_TYPE_COMPLEX | FFI_TYPE_LONGDOUBLE | _ => {}
    };
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
    static mut template: [::core::ffi::c_ushort; 5] = [
        0xd10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        0xeb01 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        0x100e as ::core::ffi::c_int as ::core::ffi::c_ushort,
        0x4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        0x7f1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    ];
    let mut dest: Option<unsafe extern "C" fn() -> ()> = None;
    let mut tramp: *mut ::core::ffi::c_ulong =
        &raw mut (*closure).c2rust_unnamed.tramp as *mut ::core::ffi::c_ulong;
    if (*cif).abi as ::core::ffi::c_uint != FFI_SYSV as ::core::ffi::c_int as ::core::ffi::c_uint {
        return FFI_BAD_ABI;
    }
    if ffi_tramp_is_present(closure as *mut ::core::ffi::c_void) != 0 {
        dest = Some(ffi_closure_SYSV as unsafe extern "C" fn() -> ())
            as Option<unsafe extern "C" fn() -> ()>;
        ffi_tramp_set_parms(
            (*closure).c2rust_unnamed.ftramp,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, *mut ::core::ffi::c_void>(
                dest,
            ),
            closure as *mut ::core::ffi::c_void,
        );
    } else {
        memcpy(
            tramp as *mut ::core::ffi::c_void,
            &raw const template as *const ::core::ffi::c_ushort as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_ushort; 5]>() as size_t,
        );
        *tramp.offset(2 as ::core::ffi::c_int as isize) = codeloc as ::core::ffi::c_ulong;
        *tramp.offset(3 as ::core::ffi::c_int as isize) =
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, ::core::ffi::c_ulong>(
                Some(ffi_closure_SYSV as unsafe extern "C" fn() -> ()),
            );
    }
    (*closure).cif = cif;
    (*closure).fun = fun;
    (*closure).user_data = user_data;
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
    if (*cif).abi as ::core::ffi::c_uint != FFI_SYSV as ::core::ffi::c_int as ::core::ffi::c_uint {
        return FFI_BAD_ABI;
    }
    (*closure).tramp = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        *mut ::core::ffi::c_void,
    >(Some(ffi_go_closure_SYSV as unsafe extern "C" fn() -> ()));
    (*closure).cif = cif;
    (*closure).fun = fun;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_arch(
    mut tramp_size: *mut size_t,
    mut map_size: *mut size_t,
) -> *mut ::core::ffi::c_void {
    extern "C" {
        static mut trampoline_code_table: *mut ::core::ffi::c_void;
    }
    *tramp_size = FFI390_TRAMP_SIZE as size_t;
    *map_size = FFI390_TRAMP_MAP_SIZE as size_t;
    return &raw mut trampoline_code_table as *mut ::core::ffi::c_void;
}
