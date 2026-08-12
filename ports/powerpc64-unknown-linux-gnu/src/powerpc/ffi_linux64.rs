extern "C" {
    static mut ffi_type_longdouble: ffi_type;
    fn ffi_tramp_set_parms(
        tramp: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        code: *mut ::core::ffi::c_void,
    );
    fn abort() -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ffi_tramp_is_present(closure: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn ffi_closure_LINUX64();
}
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_LAST_ABI: ffi_abi = 16;
pub const FFI_DEFAULT_ABI: ffi_abi = 10;
pub const FFI_LINUX_LONG_DOUBLE_IEEE128: ffi_abi = 4;
pub const FFI_LINUX_LONG_DOUBLE_128: ffi_abi = 2;
pub const FFI_LINUX_STRUCT_ALIGN: ffi_abi = 1;
pub const FFI_LINUX: ffi_abi = 8;
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
pub struct extended_cif {
    pub cif: *mut ffi_cif,
    pub rvalue: *mut ::core::ffi::c_void,
    pub avalue: *mut *mut ::core::ffi::c_void,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const FLAG_VEC_ARGUMENTS: C2RustUnnamed_0 = 268435456;
pub const FLAG_RETVAL_REFERENCE: C2RustUnnamed_0 = 134217728;
pub const FLAG_4_GPR_ARGUMENTS: C2RustUnnamed_0 = 67108864;
pub const FLAG_FP_ARGUMENTS: C2RustUnnamed_0 = 33554432;
pub const FLAG_ARG_NEEDS_PSAVE: C2RustUnnamed_0 = 16777216;
pub const FLAG_ARG_NEEDS_COPY: C2RustUnnamed_0 = 16777216;
pub const FLAG_COMPAT: C2RustUnnamed_0 = 8388608;
pub const FLAG_RETURNS_128BITS: C2RustUnnamed_0 = 32;
pub const FLAG_RETURNS_64BITS: C2RustUnnamed_0 = 16;
pub const FLAG_RETURNS_VEC: C2RustUnnamed_0 = 8;
pub const FLAG_RETURNS_FP: C2RustUnnamed_0 = 4;
pub const FLAG_RETURNS_NOTHING: C2RustUnnamed_0 = 2;
pub const FLAG_RETURNS_SMST: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ffi_dblfl {
    pub f: ::core::ffi::c_float,
    pub d: ::core::ffi::c_double,
}
pub type float128 = [::core::ffi::c_char; 16];
pub const NUM_GPR_ARG_REGISTERS64: C2RustUnnamed_2 = 8;
pub const NUM_VEC_ARG_REGISTERS64: C2RustUnnamed_2 = 12;
pub const NUM_FPR_ARG_REGISTERS64: C2RustUnnamed_2 = 13;
pub const ASM_NEEDS_REGISTERS64: C2RustUnnamed_3 = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub union valp {
    pub c: *mut ::core::ffi::c_char,
    pub ul: *mut ::core::ffi::c_ulong,
    pub f: *mut ::core::ffi::c_float,
    pub d: *mut ::core::ffi::c_double,
    pub f128: *mut float128,
    pub p: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub v: *mut *mut ::core::ffi::c_void,
    pub c: *mut *mut ::core::ffi::c_char,
    pub sc: *mut *mut ::core::ffi::c_schar,
    pub uc: *mut *mut ::core::ffi::c_uchar,
    pub ss: *mut *mut ::core::ffi::c_short,
    pub us: *mut *mut ::core::ffi::c_ushort,
    pub si: *mut *mut ::core::ffi::c_int,
    pub ui: *mut *mut ::core::ffi::c_uint,
    pub ul: *mut *mut ::core::ffi::c_ulong,
    pub f: *mut *mut ::core::ffi::c_float,
    pub d: *mut *mut ::core::ffi::c_double,
    pub f128: *mut *mut float128,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const FFI_TYPE_VOID: ::core::ffi::c_int = 0;
pub const FFI_TYPE_INT: ::core::ffi::c_int = 1;
pub const FFI_TYPE_FLOAT: ::core::ffi::c_int = 2;
pub const FFI_TYPE_DOUBLE: ::core::ffi::c_int = 3;
pub const FFI_TYPE_LONGDOUBLE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
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
pub const PPC_LD_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PPC_LD_R3: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PPC_LD_R3R4: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PPC_LD_F32: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PPC_LD_F64: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PPC_LD_F128: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PPC_LD_U8: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PPC_LD_S8: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const PPC_LD_U16: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PPC_LD_S16: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const PPC_LD_U32: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PPC_LD_S32: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const PPC_LD_PTR: ::core::ffi::c_int = PPC_LD_R3;
pub const PPC_LD_I64: ::core::ffi::c_int = PPC_LD_R3;
pub const PPC64_LD_VECTOR: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const PPC64_LD_VECTOR_HOMOG: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const PPC64_LD_FLOAT_HOMOG: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const PPC64_LD_DOUBLE_HOMOG: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const PPC64_LD_STRUCT_3: ::core::ffi::c_int = PPC_LD_U32;
pub const PPC64_LD_STRUCT_5: ::core::ffi::c_int = PPC_LD_I64;
pub const PPC64_LD_STRUCT_6: ::core::ffi::c_int = PPC_LD_I64;
pub const PPC64_LD_STRUCT_7: ::core::ffi::c_int = PPC_LD_I64;
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_types_linux64(mut abi: ffi_abi) {
    if abi as ::core::ffi::c_uint
        & (FFI_LINUX as ::core::ffi::c_int | FFI_LINUX_LONG_DOUBLE_128 as ::core::ffi::c_int)
            as ::core::ffi::c_uint
        == FFI_LINUX as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ffi_type_longdouble.size = 8 as size_t;
        ffi_type_longdouble.alignment = 8 as ::core::ffi::c_ushort;
    } else {
        ffi_type_longdouble.size = 16 as size_t;
        ffi_type_longdouble.alignment = 16 as ::core::ffi::c_ushort;
    };
}
unsafe extern "C" fn discover_homogeneous_aggregate(
    mut abi: ffi_abi,
    mut t: *const ffi_type,
    mut elnum: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    match (*t).type_0 as ::core::ffi::c_int {
        FFI_TYPE_LONGDOUBLE => {
            if abi as ::core::ffi::c_uint
                & FFI_LINUX_LONG_DOUBLE_128 as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
            {
                *elnum = 1 as ::core::ffi::c_uint;
                return FFI_TYPE_DOUBLE as ::core::ffi::c_uint;
            } else if abi as ::core::ffi::c_uint
                & FFI_LINUX_LONG_DOUBLE_IEEE128 as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
            {
                *elnum = 2 as ::core::ffi::c_uint;
                return FFI_TYPE_LONGDOUBLE as ::core::ffi::c_uint;
            }
        }
        FFI_TYPE_FLOAT | FFI_TYPE_DOUBLE => {}
        FFI_TYPE_STRUCT => {
            let mut base_elt: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            let mut total_elnum: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            let mut el: *mut *mut ffi_type = (*t).elements as *mut *mut ffi_type;
            while !(*el).is_null() {
                let mut el_elt: ::core::ffi::c_uint = 0;
                let mut el_elnum: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                el_elt = discover_homogeneous_aggregate(abi, *el, &raw mut el_elnum);
                if el_elt == 0 as ::core::ffi::c_uint || base_elt != 0 && base_elt != el_elt {
                    return 0 as ::core::ffi::c_uint;
                }
                base_elt = el_elt;
                total_elnum = total_elnum.wrapping_add(el_elnum);
                if total_elnum > 1 as ::core::ffi::c_uint {
                    return 0 as ::core::ffi::c_uint;
                }
                el = el.offset(1);
            }
            *elnum = total_elnum;
            return base_elt;
        }
        _ => return 0 as ::core::ffi::c_uint,
    }
    *elnum = 1 as ::core::ffi::c_uint;
    return (*t).type_0 as ::core::ffi::c_int as ::core::ffi::c_uint;
}
unsafe extern "C" fn ffi_prep_cif_linux64_core(mut cif: *mut ffi_cif) -> ffi_status {
    let mut ptr: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut bytes: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    let mut fparg_count: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut intarg_count: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut vecarg_count: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut flags: ::core::ffi::c_uint = (*cif).flags;
    let mut elt: ::core::ffi::c_uint = 0;
    let mut elnum: ::core::ffi::c_uint = 0;
    let mut rtype: ::core::ffi::c_uint = 0;
    if (*cif).abi as ::core::ffi::c_uint
        & FFI_LINUX_LONG_DOUBLE_IEEE128 as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        return FFI_BAD_ABI;
    }
    bytes = ((6 as ::core::ffi::c_int + ASM_NEEDS_REGISTERS64 as ::core::ffi::c_int) as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as usize)
        as ::core::ffi::c_uint;
    bytes = (bytes as ::core::ffi::c_ulong).wrapping_add(
        ((2 as ::core::ffi::c_int * NUM_GPR_ARG_REGISTERS64 as ::core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as usize)
            as ::core::ffi::c_ulong,
    ) as ::core::ffi::c_uint as ::core::ffi::c_uint;
    rtype = (*(*cif).rtype).type_0 as ::core::ffi::c_uint;
    let mut current_block_13: u64;
    match rtype {
        4 => {
            if (*cif).abi as ::core::ffi::c_uint
                & FFI_LINUX_LONG_DOUBLE_IEEE128 as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                flags |= FLAG_RETURNS_VEC as ::core::ffi::c_int as ::core::ffi::c_uint;
                current_block_13 = 6009453772311597924;
            } else {
                if (*cif).abi as ::core::ffi::c_uint
                    & FFI_LINUX_LONG_DOUBLE_128 as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    flags |= FLAG_RETURNS_128BITS as ::core::ffi::c_int as ::core::ffi::c_uint;
                }
                current_block_13 = 15709411293161541035;
            }
        }
        3 => {
            current_block_13 = 15709411293161541035;
        }
        2 => {
            current_block_13 = 10755683262200018870;
        }
        11 | 12 | 14 => {
            flags |= FLAG_RETURNS_64BITS as ::core::ffi::c_int as ::core::ffi::c_uint;
            current_block_13 = 6009453772311597924;
        }
        13 => {
            intarg_count = intarg_count.wrapping_add(1);
            flags |= FLAG_RETVAL_REFERENCE as ::core::ffi::c_int as ::core::ffi::c_uint;
            current_block_13 = 12376787134064811136;
        }
        0 => {
            current_block_13 = 12376787134064811136;
        }
        _ => {
            current_block_13 = 6009453772311597924;
        }
    }
    match current_block_13 {
        15709411293161541035 => {
            flags |= FLAG_RETURNS_64BITS as ::core::ffi::c_int as ::core::ffi::c_uint;
            current_block_13 = 10755683262200018870;
        }
        12376787134064811136 => {
            flags |= FLAG_RETURNS_NOTHING as ::core::ffi::c_int as ::core::ffi::c_uint;
            current_block_13 = 6009453772311597924;
        }
        _ => {}
    }
    match current_block_13 {
        10755683262200018870 => {
            flags |= FLAG_RETURNS_FP as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
        _ => {}
    }
    ptr = (*cif).arg_types;
    i = (*cif).nargs;
    while i > 0 as ::core::ffi::c_uint {
        let mut align: ::core::ffi::c_uint = 0;
        let mut current_block_52: u64;
        match (**ptr).type_0 as ::core::ffi::c_int {
            FFI_TYPE_LONGDOUBLE => {
                if (*cif).abi as ::core::ffi::c_uint
                    & FFI_LINUX_LONG_DOUBLE_IEEE128 as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    vecarg_count = vecarg_count.wrapping_add(1);
                    intarg_count = intarg_count.wrapping_add(3 as ::core::ffi::c_uint)
                        & !(0x1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
                    if vecarg_count
                        > NUM_VEC_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        flags |= FLAG_ARG_NEEDS_PSAVE as ::core::ffi::c_int as ::core::ffi::c_uint;
                    }
                    current_block_52 = 4741994311446740739;
                } else {
                    if (*cif).abi as ::core::ffi::c_uint
                        & FFI_LINUX_LONG_DOUBLE_128 as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0 as ::core::ffi::c_uint
                    {
                        fparg_count = fparg_count.wrapping_add(1);
                        intarg_count = intarg_count.wrapping_add(1);
                    }
                    current_block_52 = 3846739063131686548;
                }
            }
            FFI_TYPE_DOUBLE | FFI_TYPE_FLOAT => {
                current_block_52 = 3846739063131686548;
            }
            FFI_TYPE_STRUCT => {
                if (*cif).abi as ::core::ffi::c_uint
                    & FFI_LINUX_STRUCT_ALIGN as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    align = (**ptr).alignment as ::core::ffi::c_uint;
                    if align > 16 as ::core::ffi::c_uint {
                        align = 16 as ::core::ffi::c_uint;
                    }
                    align = align.wrapping_div(8 as ::core::ffi::c_uint);
                    if align > 1 as ::core::ffi::c_uint {
                        intarg_count = ((intarg_count as size_t).wrapping_sub(1 as size_t)
                            | align.wrapping_sub(1 as ::core::ffi::c_uint) as size_t)
                            .wrapping_add(1 as size_t)
                            as ::core::ffi::c_uint;
                    }
                }
                intarg_count = (intarg_count as size_t).wrapping_add(
                    (**ptr)
                        .size
                        .wrapping_add(7 as size_t)
                        .wrapping_div(8 as size_t),
                ) as ::core::ffi::c_uint as ::core::ffi::c_uint;
                elt = discover_homogeneous_aggregate((*cif).abi, *ptr, &raw mut elnum);
                if elt == FFI_TYPE_LONGDOUBLE as ::core::ffi::c_uint
                    && (*cif).abi as ::core::ffi::c_uint
                        & FFI_LINUX_LONG_DOUBLE_IEEE128 as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0 as ::core::ffi::c_uint
                {
                    vecarg_count = vecarg_count.wrapping_add(elnum);
                    if vecarg_count
                        > NUM_VEC_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        flags |= FLAG_ARG_NEEDS_PSAVE as ::core::ffi::c_int as ::core::ffi::c_uint;
                    }
                } else if elt != 0 {
                    fparg_count = fparg_count.wrapping_add(elnum);
                    if fparg_count
                        > NUM_FPR_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        flags |= FLAG_ARG_NEEDS_PSAVE as ::core::ffi::c_int as ::core::ffi::c_uint;
                    }
                } else if intarg_count
                    > NUM_GPR_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    flags |= FLAG_ARG_NEEDS_PSAVE as ::core::ffi::c_int as ::core::ffi::c_uint;
                }
                current_block_52 = 4741994311446740739;
            }
            FFI_TYPE_POINTER | FFI_TYPE_UINT64 | FFI_TYPE_SINT64 | FFI_TYPE_INT
            | FFI_TYPE_UINT32 | FFI_TYPE_SINT32 | FFI_TYPE_UINT16 | FFI_TYPE_SINT16
            | FFI_TYPE_UINT8 | FFI_TYPE_SINT8 => {
                intarg_count = intarg_count.wrapping_add(1);
                if intarg_count
                    > NUM_GPR_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    flags |= FLAG_ARG_NEEDS_PSAVE as ::core::ffi::c_int as ::core::ffi::c_uint;
                }
                current_block_52 = 4741994311446740739;
            }
            _ => {
                current_block_52 = 4741994311446740739;
            }
        }
        match current_block_52 {
            3846739063131686548 => {
                fparg_count = fparg_count.wrapping_add(1);
                intarg_count = intarg_count.wrapping_add(1);
                if fparg_count
                    > NUM_FPR_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    flags |= FLAG_ARG_NEEDS_PSAVE as ::core::ffi::c_int as ::core::ffi::c_uint;
                }
            }
            _ => {}
        }
        i = i.wrapping_sub(1);
        ptr = ptr.offset(1);
    }
    if fparg_count != 0 as ::core::ffi::c_uint {
        flags |= FLAG_FP_ARGUMENTS as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    if intarg_count > 4 as ::core::ffi::c_uint {
        flags |= FLAG_4_GPR_ARGUMENTS as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    if vecarg_count != 0 as ::core::ffi::c_uint {
        flags |= FLAG_VEC_ARGUMENTS as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    if fparg_count != 0 as ::core::ffi::c_uint {
        bytes = (bytes as ::core::ffi::c_ulong).wrapping_add(
            (NUM_FPR_ARG_REGISTERS64 as ::core::ffi::c_int as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize)
                as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_uint as ::core::ffi::c_uint;
    }
    if vecarg_count != 0 as ::core::ffi::c_uint {
        bytes = bytes.wrapping_add(15 as ::core::ffi::c_uint)
            & !(0xf as ::core::ffi::c_int) as ::core::ffi::c_uint;
        bytes = (bytes as ::core::ffi::c_ulong).wrapping_add(
            (NUM_VEC_ARG_REGISTERS64 as ::core::ffi::c_int as usize)
                .wrapping_mul(::core::mem::size_of::<float128>() as usize)
                as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_uint as ::core::ffi::c_uint;
    }
    if intarg_count > NUM_GPR_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint {
        bytes = (bytes as ::core::ffi::c_ulong).wrapping_add(
            (intarg_count
                .wrapping_sub(NUM_GPR_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint)
                as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as usize)
                as ::core::ffi::c_ulong,
        ) as ::core::ffi::c_uint as ::core::ffi::c_uint;
    }
    bytes = bytes.wrapping_add(15 as ::core::ffi::c_uint)
        & !(0xf as ::core::ffi::c_int) as ::core::ffi::c_uint;
    (*cif).flags = flags;
    (*cif).bytes = bytes;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_linux64(mut cif: *mut ffi_cif) -> ffi_status {
    if (*cif).abi as ::core::ffi::c_uint & FFI_LINUX as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        (*cif).nfixedargs = (*cif).nargs;
    } else if (*cif).abi as ::core::ffi::c_uint
        == FFI_COMPAT_LINUX64 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cif).flags |= FLAG_COMPAT as ::core::ffi::c_int as ::core::ffi::c_uint;
        (*cif).abi = (FFI_LINUX as ::core::ffi::c_int
            | FFI_LINUX_LONG_DOUBLE_128 as ::core::ffi::c_int) as ffi_abi;
    } else {
        return FFI_BAD_ABI;
    }
    return ffi_prep_cif_linux64_core(cif);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_linux64_var(
    mut cif: *mut ffi_cif,
    mut nfixedargs: ::core::ffi::c_uint,
    mut ntotalargs: ::core::ffi::c_uint,
) -> ffi_status {
    if (*cif).abi as ::core::ffi::c_uint & FFI_LINUX as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        (*cif).nfixedargs = nfixedargs;
    } else if (*cif).abi as ::core::ffi::c_uint
        == FFI_COMPAT_LINUX64 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cif).flags |= FLAG_COMPAT as ::core::ffi::c_int as ::core::ffi::c_uint;
        (*cif).abi = (FFI_LINUX as ::core::ffi::c_int
            | FFI_LINUX_LONG_DOUBLE_128 as ::core::ffi::c_int) as ffi_abi;
    } else {
        return FFI_BAD_ABI;
    }
    return ffi_prep_cif_linux64_core(cif);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_args64(
    mut ecif: *mut extended_cif,
    stack: *mut ::core::ffi::c_ulong,
) {
    let mut current_block: u64;
    let bytes: ::core::ffi::c_ulong = (*(*ecif).cif).bytes as ::core::ffi::c_ulong;
    let flags: ::core::ffi::c_ulong = (*(*ecif).cif).flags as ::core::ffi::c_ulong;
    let mut stacktop: valp = valp {
        c: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut gpr_base: valp = valp {
        c: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut gpr_end: valp = valp {
        c: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut rest: valp = valp {
        c: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut next_arg: valp = valp {
        c: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut fpr_base: valp = valp {
        c: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut fparg_count: ::core::ffi::c_uint = 0;
    let mut vec_base: valp = valp {
        c: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut vecarg_count: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    let mut words: ::core::ffi::c_uint = 0;
    let mut nargs: ::core::ffi::c_uint = 0;
    let mut nfixedargs: ::core::ffi::c_uint = 0;
    let mut ptr: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut double_tmp: ::core::ffi::c_double = 0.;
    let mut p_argv: C2RustUnnamed_1 = C2RustUnnamed_1 {
        v: ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    };
    let mut gprvalue: ::core::ffi::c_ulong = 0;
    let mut align: ::core::ffi::c_ulong = 0;
    stacktop.c = (stack as *mut ::core::ffi::c_char).offset(bytes as isize);
    gpr_base.ul = stacktop
        .ul
        .offset(-(ASM_NEEDS_REGISTERS64 as ::core::ffi::c_int as isize))
        .offset(-(NUM_GPR_ARG_REGISTERS64 as ::core::ffi::c_int as isize));
    gpr_end.ul = gpr_base
        .ul
        .offset(NUM_GPR_ARG_REGISTERS64 as ::core::ffi::c_int as isize);
    rest.ul = stack
        .offset(6 as ::core::ffi::c_int as isize)
        .offset(NUM_GPR_ARG_REGISTERS64 as ::core::ffi::c_int as isize);
    fpr_base.d = gpr_base
        .d
        .offset(-(NUM_FPR_ARG_REGISTERS64 as ::core::ffi::c_int as isize));
    fparg_count = 0 as ::core::ffi::c_uint;
    if (*(*ecif).cif).flags & FLAG_FP_ARGUMENTS as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        vec_base.p = fpr_base.p & !(0xf as ::core::ffi::c_int) as size_t;
    } else {
        vec_base.p = gpr_base.p;
    }
    vec_base.f128 = vec_base
        .f128
        .offset(-(NUM_VEC_ARG_REGISTERS64 as ::core::ffi::c_int as isize));
    vecarg_count = 0 as ::core::ffi::c_uint;
    next_arg.ul = gpr_base.ul;
    if flags & FLAG_RETVAL_REFERENCE as ::core::ffi::c_int as ::core::ffi::c_ulong != 0 {
        let fresh0 = next_arg.ul;
        next_arg.ul = next_arg.ul.offset(1);
        *fresh0 = (*ecif).rvalue as *mut ::core::ffi::c_char as ::core::ffi::c_ulong;
    }
    p_argv.v = (*ecif).avalue;
    nargs = (*(*ecif).cif).nargs;
    nfixedargs = -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    if flags & FLAG_COMPAT as ::core::ffi::c_int as ::core::ffi::c_ulong
        == 0 as ::core::ffi::c_ulong
    {
        nfixedargs = (*(*ecif).cif).nfixedargs;
    }
    ptr = (*(*ecif).cif).arg_types;
    i = 0 as ::core::ffi::c_uint;
    while i < nargs {
        let mut elt: ::core::ffi::c_uint = 0;
        let mut elnum: ::core::ffi::c_uint = 0;
        match (**ptr).type_0 as ::core::ffi::c_int {
            FFI_TYPE_LONGDOUBLE => {
                if (*(*ecif).cif).abi as ::core::ffi::c_uint
                    & FFI_LINUX_LONG_DOUBLE_IEEE128 as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    next_arg.p = (next_arg.p.wrapping_sub(1 as size_t)
                        | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                        .wrapping_add(1 as size_t);
                    if next_arg.ul == gpr_end.ul {
                        next_arg.ul = rest.ul;
                    }
                    if vecarg_count
                        < NUM_VEC_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint
                        && i < nfixedargs
                    {
                        let fresh1 = vec_base.f128;
                        vec_base.f128 = vec_base.f128.offset(1);
                        memcpy(
                            fresh1 as *mut ::core::ffi::c_void,
                            *p_argv.f128 as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<float128>() as size_t,
                        );
                    } else {
                        memcpy(
                            next_arg.f128 as *mut ::core::ffi::c_void,
                            *p_argv.f128 as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<float128>() as size_t,
                        );
                    }
                    next_arg.f128 = next_arg.f128.offset(1);
                    if next_arg.f128 == gpr_end.f128 {
                        next_arg.f128 = rest.f128;
                    }
                    vecarg_count = vecarg_count.wrapping_add(1);
                    current_block = 10043043949733653460;
                } else if (*(*ecif).cif).abi as ::core::ffi::c_uint
                    & FFI_LINUX_LONG_DOUBLE_128 as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    double_tmp = *(*p_argv.d).offset(0 as ::core::ffi::c_int as isize);
                    if fparg_count
                        < NUM_FPR_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint
                        && i < nfixedargs
                    {
                        let fresh2 = fpr_base.d;
                        fpr_base.d = fpr_base.d.offset(1);
                        *fresh2 = double_tmp;
                        if flags & FLAG_COMPAT as ::core::ffi::c_int as ::core::ffi::c_ulong
                            != 0 as ::core::ffi::c_ulong
                        {
                            *next_arg.d = double_tmp;
                        }
                    } else {
                        *next_arg.d = double_tmp;
                    }
                    next_arg.ul = next_arg.ul.offset(1);
                    if next_arg.ul == gpr_end.ul {
                        next_arg.ul = rest.ul;
                    }
                    fparg_count = fparg_count.wrapping_add(1);
                    double_tmp = *(*p_argv.d).offset(1 as ::core::ffi::c_int as isize);
                    if fparg_count
                        < NUM_FPR_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint
                        && i < nfixedargs
                    {
                        let fresh3 = fpr_base.d;
                        fpr_base.d = fpr_base.d.offset(1);
                        *fresh3 = double_tmp;
                        if flags & FLAG_COMPAT as ::core::ffi::c_int as ::core::ffi::c_ulong
                            != 0 as ::core::ffi::c_ulong
                        {
                            *next_arg.d = double_tmp;
                        }
                    } else {
                        *next_arg.d = double_tmp;
                    }
                    next_arg.ul = next_arg.ul.offset(1);
                    if next_arg.ul == gpr_end.ul {
                        next_arg.ul = rest.ul;
                    }
                    fparg_count = fparg_count.wrapping_add(1);
                    current_block = 10043043949733653460;
                } else {
                    current_block = 11883447716370441420;
                }
            }
            FFI_TYPE_DOUBLE => {
                current_block = 11883447716370441420;
            }
            FFI_TYPE_FLOAT => {
                current_block = 1822693298316003543;
            }
            FFI_TYPE_STRUCT => {
                if (*(*ecif).cif).abi as ::core::ffi::c_uint
                    & FFI_LINUX_STRUCT_ALIGN as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    align = (**ptr).alignment as ::core::ffi::c_ulong;
                    if align > 16 as ::core::ffi::c_ulong {
                        align = 16 as ::core::ffi::c_ulong;
                    }
                    if align > 1 as ::core::ffi::c_ulong {
                        next_arg.p = (next_arg.p.wrapping_sub(1 as size_t)
                            | (align as size_t).wrapping_sub(1 as size_t))
                        .wrapping_add(1 as size_t);
                        if next_arg.ul == gpr_end.ul {
                            next_arg.ul = rest.ul;
                        }
                    }
                }
                elt = discover_homogeneous_aggregate((*(*ecif).cif).abi, *ptr, &raw mut elnum);
                if elt != 0 {
                    if elt == FFI_TYPE_FLOAT as ::core::ffi::c_uint {
                        current_block = 1822693298316003543;
                    } else {
                        current_block = 11883447716370441420;
                    }
                } else {
                    words = (**ptr)
                        .size
                        .wrapping_add(7 as size_t)
                        .wrapping_div(8 as size_t)
                        as ::core::ffi::c_uint;
                    if next_arg.ul >= gpr_base.ul && next_arg.ul.offset(words as isize) > gpr_end.ul
                    {
                        let mut first: size_t =
                            gpr_end.c.offset_from(next_arg.c) as ::core::ffi::c_long as size_t;
                        memcpy(
                            next_arg.c as *mut ::core::ffi::c_void,
                            *p_argv.c as *const ::core::ffi::c_void,
                            first,
                        );
                        memcpy(
                            rest.c as *mut ::core::ffi::c_void,
                            (*p_argv.c).offset(first as isize) as *const ::core::ffi::c_void,
                            (**ptr).size.wrapping_sub(first),
                        );
                        next_arg.c = rest
                            .c
                            .offset(words.wrapping_mul(8 as ::core::ffi::c_uint) as isize)
                            .offset(-(first as isize));
                    } else {
                        let mut where_0: *mut ::core::ffi::c_char = next_arg.c;
                        memcpy(
                            where_0 as *mut ::core::ffi::c_void,
                            *p_argv.c as *const ::core::ffi::c_void,
                            (**ptr).size,
                        );
                        next_arg.ul = next_arg.ul.offset(words as isize);
                        if next_arg.ul == gpr_end.ul {
                            next_arg.ul = rest.ul;
                        }
                    }
                    current_block = 10043043949733653460;
                }
            }
            FFI_TYPE_UINT8 => {
                gprvalue = **p_argv.uc as ::core::ffi::c_ulong;
                current_block = 5894510895438022481;
            }
            FFI_TYPE_SINT8 => {
                gprvalue = **p_argv.sc as ::core::ffi::c_ulong;
                current_block = 5894510895438022481;
            }
            FFI_TYPE_UINT16 => {
                gprvalue = **p_argv.us as ::core::ffi::c_ulong;
                current_block = 5894510895438022481;
            }
            FFI_TYPE_SINT16 => {
                gprvalue = **p_argv.ss as ::core::ffi::c_ulong;
                current_block = 5894510895438022481;
            }
            FFI_TYPE_UINT32 => {
                gprvalue = **p_argv.ui as ::core::ffi::c_ulong;
                current_block = 5894510895438022481;
            }
            FFI_TYPE_INT | FFI_TYPE_SINT32 => {
                gprvalue = **p_argv.si as ::core::ffi::c_ulong;
                current_block = 5894510895438022481;
            }
            FFI_TYPE_UINT64 | FFI_TYPE_SINT64 | FFI_TYPE_POINTER => {
                gprvalue = **p_argv.ul;
                current_block = 5894510895438022481;
            }
            _ => {
                current_block = 10043043949733653460;
            }
        }
        match current_block {
            11883447716370441420 => {
                double_tmp = **p_argv.d;
                if fparg_count
                    < NUM_FPR_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint
                    && i < nfixedargs
                {
                    let fresh4 = fpr_base.d;
                    fpr_base.d = fpr_base.d.offset(1);
                    *fresh4 = double_tmp;
                    if flags & FLAG_COMPAT as ::core::ffi::c_int as ::core::ffi::c_ulong
                        != 0 as ::core::ffi::c_ulong
                    {
                        *next_arg.d = double_tmp;
                    }
                } else {
                    *next_arg.d = double_tmp;
                }
                next_arg.ul = next_arg.ul.offset(1);
                if next_arg.ul == gpr_end.ul {
                    next_arg.ul = rest.ul;
                }
                fparg_count = fparg_count.wrapping_add(1);
            }
            1822693298316003543 => {
                double_tmp = **p_argv.f as ::core::ffi::c_double;
                if fparg_count
                    < NUM_FPR_ARG_REGISTERS64 as ::core::ffi::c_int as ::core::ffi::c_uint
                    && i < nfixedargs
                {
                    let fresh5 = fpr_base.d;
                    fpr_base.d = fpr_base.d.offset(1);
                    *fresh5 = double_tmp;
                    if flags & FLAG_COMPAT as ::core::ffi::c_int as ::core::ffi::c_ulong
                        != 0 as ::core::ffi::c_ulong
                    {
                        *next_arg.f.offset(0 as ::core::ffi::c_int as isize) =
                            double_tmp as ::core::ffi::c_float;
                    }
                } else {
                    *next_arg.f.offset(0 as ::core::ffi::c_int as isize) =
                        double_tmp as ::core::ffi::c_float;
                }
                next_arg.ul = next_arg.ul.offset(1);
                if next_arg.ul == gpr_end.ul {
                    next_arg.ul = rest.ul;
                }
                fparg_count = fparg_count.wrapping_add(1);
            }
            5894510895438022481 => {
                let fresh6 = next_arg.ul;
                next_arg.ul = next_arg.ul.offset(1);
                *fresh6 = gprvalue;
                if next_arg.ul == gpr_end.ul {
                    next_arg.ul = rest.ul;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        ptr = ptr.offset(1);
        p_argv.v = p_argv.v.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_closure_loc_linux64(
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
    if ((*cif).abi as ::core::ffi::c_uint) < FFI_LINUX as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cif).abi as ::core::ffi::c_uint
            >= FFI_LAST_ABI as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return FFI_BAD_ABI;
    }
    if ffi_tramp_is_present(closure as *mut ::core::ffi::c_void) != 0 {
        let mut dest: Option<unsafe extern "C" fn() -> ()> =
            Some(ffi_closure_LINUX64 as unsafe extern "C" fn() -> ());
        ffi_tramp_set_parms(
            (*closure).c2rust_unnamed.ftramp,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, *mut ::core::ffi::c_void>(
                dest,
            ),
            closure as *mut ::core::ffi::c_void,
        );
    } else {
        let mut tramp: *mut *mut ::core::ffi::c_void =
            (&raw mut (*closure).c2rust_unnamed.tramp as *mut ::core::ffi::c_char)
                .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *mut *mut ::core::ffi::c_void;
        memcpy(
            tramp.offset(0 as ::core::ffi::c_int as isize) as *mut *mut ::core::ffi::c_void
                as *mut ::core::ffi::c_void,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                *mut *mut ::core::ffi::c_void,
            >(Some(ffi_closure_LINUX64 as unsafe extern "C" fn() -> ()))
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
        );
        let ref mut fresh7 = *tramp.offset(1 as ::core::ffi::c_int as isize);
        *fresh7 = codeloc;
        memcpy(
            tramp.offset(2 as ::core::ffi::c_int as isize) as *mut *mut ::core::ffi::c_void
                as *mut ::core::ffi::c_void,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                *mut *mut ::core::ffi::c_void,
            >(Some(ffi_closure_LINUX64 as unsafe extern "C" fn() -> ()))
            .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
        );
    }
    (*closure).cif = cif;
    (*closure).fun = fun;
    (*closure).user_data = user_data;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_helper_LINUX64(
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
    mut pst: *mut ::core::ffi::c_ulong,
    mut pfr: *mut ffi_dblfl,
    mut pvec: *mut float128,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut avalue: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut arg_types: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut i: ::core::ffi::c_ulong = 0;
    let mut avn: ::core::ffi::c_ulong = 0;
    let mut nfixedargs: ::core::ffi::c_ulong = 0;
    let mut end_pfr: *mut ffi_dblfl =
        pfr.offset(NUM_FPR_ARG_REGISTERS64 as ::core::ffi::c_int as isize);
    let mut end_pvec: *mut float128 =
        pvec.offset(NUM_VEC_ARG_REGISTERS64 as ::core::ffi::c_int as isize);
    let mut align: ::core::ffi::c_ulong = 0;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        ((*cif).nargs as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
            as usize,
    ));
    avalue = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_void;
    if (*(*cif).rtype).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT
        && (*cif).flags & FLAG_RETURNS_SMST as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
    {
        rvalue = *pst as *mut ::core::ffi::c_void;
        pst = pst.offset(1);
    }
    i = 0 as ::core::ffi::c_ulong;
    avn = (*cif).nargs as ::core::ffi::c_ulong;
    nfixedargs = -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint as ::core::ffi::c_ulong;
    if (*cif).flags & FLAG_COMPAT as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
        nfixedargs = (*cif).nfixedargs as ::core::ffi::c_ulong;
    }
    arg_types = (*cif).arg_types;
    while i < avn {
        let mut elt: ::core::ffi::c_uint = 0;
        let mut elnum: ::core::ffi::c_uint = 0;
        let mut current_block_55: u64;
        match (**arg_types.offset(i as isize)).type_0 as ::core::ffi::c_int {
            FFI_TYPE_SINT8 | FFI_TYPE_UINT8 | FFI_TYPE_SINT16 | FFI_TYPE_UINT16
            | FFI_TYPE_SINT32 | FFI_TYPE_UINT32 | FFI_TYPE_SINT64 | FFI_TYPE_UINT64
            | FFI_TYPE_POINTER => {
                let ref mut fresh8 = *avalue.offset(i as isize);
                *fresh8 = pst as *mut ::core::ffi::c_void;
                pst = pst.offset(1);
                current_block_55 = 2116367355679836638;
            }
            FFI_TYPE_STRUCT => {
                if (*cif).abi as ::core::ffi::c_uint
                    & FFI_LINUX_STRUCT_ALIGN as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    align = (**arg_types.offset(i as isize)).alignment as ::core::ffi::c_ulong;
                    if align > 16 as ::core::ffi::c_ulong {
                        align = 16 as ::core::ffi::c_ulong;
                    }
                    if align > 1 as ::core::ffi::c_ulong {
                        pst = ((pst as size_t).wrapping_sub(1 as size_t)
                            | (align as size_t).wrapping_sub(1 as size_t))
                        .wrapping_add(1 as size_t)
                            as *mut ::core::ffi::c_ulong;
                    }
                }
                elt = discover_homogeneous_aggregate(
                    (*cif).abi,
                    *arg_types.offset(i as isize),
                    &raw mut elnum,
                );
                if elt != 0 {
                    if elt == FFI_TYPE_FLOAT as ::core::ffi::c_uint {
                        current_block_55 = 7861815771053118806;
                    } else {
                        current_block_55 = 3309200060982467099;
                    }
                } else {
                    let ref mut fresh9 = *avalue.offset(i as isize);
                    *fresh9 = pst as *mut ::core::ffi::c_void;
                    pst = pst.offset(
                        (**arg_types.offset(i as isize))
                            .size
                            .wrapping_add(7 as size_t)
                            .wrapping_div(8 as size_t) as isize,
                    );
                    current_block_55 = 2116367355679836638;
                }
            }
            FFI_TYPE_LONGDOUBLE => {
                if (*cif).abi as ::core::ffi::c_uint
                    & FFI_LINUX_LONG_DOUBLE_IEEE128 as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    if pst as ::core::ffi::c_ulong & 0xf as ::core::ffi::c_ulong
                        != 0 as ::core::ffi::c_ulong
                    {
                        pst = pst.offset(1);
                    }
                    if pvec < end_pvec && i < nfixedargs {
                        let fresh10 = pvec;
                        pvec = pvec.offset(1);
                        let ref mut fresh11 = *avalue.offset(i as isize);
                        *fresh11 = fresh10 as *mut ::core::ffi::c_void;
                    } else {
                        let ref mut fresh12 = *avalue.offset(i as isize);
                        *fresh12 = pst as *mut ::core::ffi::c_void;
                    }
                    pst = pst.offset(2 as ::core::ffi::c_int as isize);
                    current_block_55 = 2116367355679836638;
                } else if (*cif).abi as ::core::ffi::c_uint
                    & FFI_LINUX_LONG_DOUBLE_128 as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    if pfr.offset(1 as ::core::ffi::c_int as isize) < end_pfr
                        && i.wrapping_add(1 as ::core::ffi::c_ulong) < nfixedargs
                    {
                        let ref mut fresh13 = *avalue.offset(i as isize);
                        *fresh13 = pfr as *mut ::core::ffi::c_void;
                        pfr = pfr.offset(2 as ::core::ffi::c_int as isize);
                    } else {
                        if pfr < end_pfr && i < nfixedargs {
                            *pst = *(pfr as *mut ::core::ffi::c_ulong);
                            pfr = pfr.offset(1);
                        }
                        let ref mut fresh14 = *avalue.offset(i as isize);
                        *fresh14 = pst as *mut ::core::ffi::c_void;
                    }
                    pst = pst.offset(2 as ::core::ffi::c_int as isize);
                    current_block_55 = 2116367355679836638;
                } else {
                    current_block_55 = 3309200060982467099;
                }
            }
            FFI_TYPE_DOUBLE => {
                current_block_55 = 3309200060982467099;
            }
            FFI_TYPE_FLOAT => {
                current_block_55 = 7861815771053118806;
            }
            _ => {
                current_block_55 = 2116367355679836638;
            }
        }
        match current_block_55 {
            7861815771053118806 => {
                if pfr < end_pfr && i < nfixedargs {
                    (*pfr).f = (*pfr).d as ::core::ffi::c_float;
                    let ref mut fresh17 = *avalue.offset(i as isize);
                    *fresh17 = pfr as *mut ::core::ffi::c_void;
                    pfr = pfr.offset(1);
                } else {
                    let ref mut fresh18 = *avalue.offset(i as isize);
                    *fresh18 = pst as *mut ::core::ffi::c_void;
                }
                pst = pst.offset(1);
            }
            3309200060982467099 => {
                if pfr < end_pfr && i < nfixedargs {
                    let ref mut fresh15 = *avalue.offset(i as isize);
                    *fresh15 = pfr as *mut ::core::ffi::c_void;
                    pfr = pfr.offset(1);
                } else {
                    let ref mut fresh16 = *avalue.offset(i as isize);
                    *fresh16 = pst as *mut ::core::ffi::c_void;
                }
                pst = pst.offset(1);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    Some(fun.expect("non-null function pointer")).expect("non-null function pointer")(
        cif, rvalue, avalue, user_data,
    );
    match (*(*cif).rtype).type_0 as ::core::ffi::c_int {
        FFI_TYPE_VOID => return PPC_LD_NONE,
        FFI_TYPE_FLOAT => return PPC_LD_F32,
        FFI_TYPE_DOUBLE => return PPC_LD_F64,
        FFI_TYPE_LONGDOUBLE => {
            if (*cif).flags & FLAG_RETURNS_VEC as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                return PPC64_LD_VECTOR;
            }
            return PPC_LD_F128;
        }
        FFI_TYPE_UINT8 => return PPC_LD_U8,
        FFI_TYPE_SINT8 => return PPC_LD_S8,
        FFI_TYPE_UINT16 => return PPC_LD_U16,
        FFI_TYPE_SINT16 => return PPC_LD_S16,
        FFI_TYPE_UINT32 => return PPC_LD_U32,
        FFI_TYPE_INT | FFI_TYPE_SINT32 => return PPC_LD_S32,
        FFI_TYPE_POINTER => return PPC_LD_PTR,
        FFI_TYPE_UINT64 | FFI_TYPE_SINT64 => return PPC_LD_I64,
        FFI_TYPE_STRUCT => {
            if (*cif).flags & FLAG_RETURNS_SMST as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                if (*cif).flags
                    & (FLAG_RETURNS_FP as ::core::ffi::c_int
                        | FLAG_RETURNS_VEC as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                    == 0 as ::core::ffi::c_uint
                {
                    match (*(*cif).rtype).size {
                        0 => return PPC_LD_NONE,
                        1 => return PPC_LD_U8,
                        2 => return PPC_LD_U16,
                        3 => return PPC64_LD_STRUCT_3,
                        4 => return PPC_LD_U32,
                        5 => return PPC64_LD_STRUCT_5,
                        6 => return PPC64_LD_STRUCT_6,
                        7 => return PPC64_LD_STRUCT_7,
                        8 => return PPC_LD_R3R4,
                        _ => {}
                    }
                } else {
                    if (*cif).flags & FLAG_RETURNS_VEC as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0 as ::core::ffi::c_uint
                    {
                        return PPC64_LD_VECTOR_HOMOG;
                    }
                    if (*cif).flags
                        & FLAG_RETURNS_64BITS as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0 as ::core::ffi::c_uint
                    {
                        return PPC64_LD_DOUBLE_HOMOG;
                    }
                    return PPC64_LD_FLOAT_HOMOG;
                }
            } else {
                return PPC_LD_NONE;
            }
        }
        _ => {}
    }
    abort();
}
