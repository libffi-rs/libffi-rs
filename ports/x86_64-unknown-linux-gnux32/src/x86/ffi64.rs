extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ffi_tramp_is_present(closure: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn abort() -> !;
    fn ffi_tramp_set_parms(
        tramp: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        code: *mut ::core::ffi::c_void,
    );
    fn ffi_call_unix64_from_rust(
        args: *mut ::core::ffi::c_void,
        bytes: ::core::ffi::c_ulong,
        flags: ::core::ffi::c_uint,
        raddr: *mut ::core::ffi::c_void,
        fnaddr: Option<unsafe extern "C" fn() -> ()>,
    );
    fn ffi_closure_unix64();
    fn ffi_closure_unix64_sse();
    fn ffi_closure_unix64_alt();
    fn ffi_closure_unix64_sse_alt();
    fn ffi_go_closure_unix64();
    fn ffi_go_closure_unix64_sse();
}
pub type __int128_t = i128;
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 2;
pub const FFI_LAST_ABI: ffi_abi = 5;
pub const FFI_GNUW64: ffi_abi = 4;
pub const FFI_EFI64: ffi_abi = 3;
pub const FFI_WIN64: ffi_abi = 3;
pub const FFI_UNIX64: ffi_abi = 2;
pub const FFI_FIRST_ABI: ffi_abi = 1;
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
pub type UINT64 = ::core::ffi::c_ulonglong;
pub type uintptr_t = usize;
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
pub struct register_args {
    pub gpr: [UINT64; 6],
    pub sse: [big_int_union; 8],
    pub rax: UINT64,
    pub r10: UINT64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union big_int_union {
    pub i32_0: UINT32,
    pub i64_0: UINT64,
    pub i128_0: __int128_t,
}
pub type UINT32 = ::core::ffi::c_uint;
pub const X86_64_SSESF_CLASS: x86_64_reg_class = 4;
pub const X86_64_SSEDF_CLASS: x86_64_reg_class = 5;
pub const X86_64_SSE_CLASS: x86_64_reg_class = 3;
pub type SINT64 = ::core::ffi::c_longlong;
pub type SINT32 = ::core::ffi::c_int;
pub type SINT16 = ::core::ffi::c_short;
pub type SINT8 = ::core::ffi::c_schar;
pub const X86_64_INTEGERSI_CLASS: x86_64_reg_class = 2;
pub const X86_64_INTEGER_CLASS: x86_64_reg_class = 1;
pub const X86_64_SSEUP_CLASS: x86_64_reg_class = 6;
pub const X86_64_NO_CLASS: x86_64_reg_class = 0;
pub type x86_64_reg_class = ::core::ffi::c_uint;
pub const X86_64_MEMORY_CLASS: x86_64_reg_class = 10;
pub const X86_64_COMPLEX_X87_CLASS: x86_64_reg_class = 9;
pub const X86_64_X87UP_CLASS: x86_64_reg_class = 8;
pub const X86_64_X87_CLASS: x86_64_reg_class = 7;
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
pub const FFI_TYPE_STRUCT: ::core::ffi::c_int = 13;
pub const FFI_TYPE_POINTER: ::core::ffi::c_int = 14;
pub const FFI_TYPE_COMPLEX: ::core::ffi::c_int = 15;
pub const FFI_TYPE_UINT128: ::core::ffi::c_int = 16;
pub const FFI_TYPE_SINT128: ::core::ffi::c_int = 17;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const UNIX64_RET_VOID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UNIX64_RET_UINT8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const UNIX64_RET_UINT16: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const UNIX64_RET_UINT32: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const UNIX64_RET_SINT8: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const UNIX64_RET_SINT16: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const UNIX64_RET_SINT32: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const UNIX64_RET_INT64: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const UNIX64_RET_XMM32: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const UNIX64_RET_XMM64: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const UNIX64_RET_X87: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const UNIX64_RET_X87_2: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const UNIX64_RET_ST_XMM0_RAX: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const UNIX64_RET_ST_RAX_XMM0: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const UNIX64_RET_ST_XMM0_XMM1: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const UNIX64_RET_ST_RAX_RDX: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const UNIX64_FLAG_RET_IN_MEM: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
pub const UNIX64_FLAG_XMM_ARGS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int;
pub const UNIX64_SIZE_SHIFT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const UNIX64_TRAMP_MAP_SHIFT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const UNIX64_TRAMP_MAP_SIZE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << UNIX64_TRAMP_MAP_SHIFT;
pub const UNIX64_TRAMP_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MAX_GPR_REGS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MAX_SSE_REGS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
unsafe extern "C" fn merge_classes(
    mut class1: x86_64_reg_class,
    mut class2: x86_64_reg_class,
) -> x86_64_reg_class {
    if class1 as ::core::ffi::c_uint == class2 as ::core::ffi::c_uint {
        return class1;
    }
    if class1 as ::core::ffi::c_uint == X86_64_NO_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return class2;
    }
    if class2 as ::core::ffi::c_uint == X86_64_NO_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return class1;
    }
    if class1 as ::core::ffi::c_uint
        == X86_64_MEMORY_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
        || class2 as ::core::ffi::c_uint
            == X86_64_MEMORY_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return X86_64_MEMORY_CLASS;
    }
    if class1 as ::core::ffi::c_uint
        == X86_64_INTEGERSI_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
        && class2 as ::core::ffi::c_uint
            == X86_64_SSESF_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
        || class2 as ::core::ffi::c_uint
            == X86_64_INTEGERSI_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
            && class1 as ::core::ffi::c_uint
                == X86_64_SSESF_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return X86_64_INTEGERSI_CLASS;
    }
    if class1 as ::core::ffi::c_uint
        == X86_64_INTEGER_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
        || class1 as ::core::ffi::c_uint
            == X86_64_INTEGERSI_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
        || class2 as ::core::ffi::c_uint
            == X86_64_INTEGER_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
        || class2 as ::core::ffi::c_uint
            == X86_64_INTEGERSI_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return X86_64_INTEGER_CLASS;
    }
    if class1 as ::core::ffi::c_uint
        == X86_64_X87_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
        || class1 as ::core::ffi::c_uint
            == X86_64_X87UP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
        || class1 as ::core::ffi::c_uint
            == X86_64_COMPLEX_X87_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
        || class2 as ::core::ffi::c_uint
            == X86_64_X87_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
        || class2 as ::core::ffi::c_uint
            == X86_64_X87UP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
        || class2 as ::core::ffi::c_uint
            == X86_64_COMPLEX_X87_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return X86_64_MEMORY_CLASS;
    }
    return X86_64_SSE_CLASS;
}
unsafe extern "C" fn classify_argument(
    mut type_0: *mut ffi_type,
    mut classes: *mut x86_64_reg_class,
    mut byte_offset: size_t,
) -> size_t {
    's_390: {
        let mut UNITS_PER_WORD: size_t = 0;
        let mut words: size_t = 0;
        let mut ptr: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
        let mut i: ::core::ffi::c_uint = 0;
        let mut subclasses: [x86_64_reg_class; 4] = [X86_64_NO_CLASS; 4];
        let mut current_block_80: u64;
        match (*type_0).type_0 as ::core::ffi::c_int {
            FFI_TYPE_UINT8 | FFI_TYPE_SINT8 | FFI_TYPE_UINT16 | FFI_TYPE_SINT16
            | FFI_TYPE_UINT32 | FFI_TYPE_SINT32 | FFI_TYPE_UINT64 | FFI_TYPE_SINT64
            | FFI_TYPE_UINT128 | FFI_TYPE_SINT128 | FFI_TYPE_POINTER => {
                current_block_80 = 9317908997127676919;
            }
            FFI_TYPE_FLOAT => {
                current_block_80 = 3040524815560281372;
            }
            FFI_TYPE_DOUBLE => {
                *classes.offset(0 as ::core::ffi::c_int as isize) = X86_64_SSEDF_CLASS;
                return 1 as size_t;
            }
            FFI_TYPE_LONGDOUBLE => {
                *classes.offset(0 as ::core::ffi::c_int as isize) = X86_64_X87_CLASS;
                *classes.offset(1 as ::core::ffi::c_int as isize) = X86_64_X87UP_CLASS;
                return 2 as size_t;
            }
            FFI_TYPE_STRUCT => {
                UNITS_PER_WORD = 8 as size_t;
                words = (*type_0)
                    .size
                    .wrapping_add(byte_offset)
                    .wrapping_add(UNITS_PER_WORD)
                    .wrapping_sub(1 as size_t)
                    .wrapping_div(UNITS_PER_WORD);
                ptr = ::core::ptr::null_mut::<*mut ffi_type>();
                i = 0;
                subclasses = [X86_64_NO_CLASS; 4];
                if (*type_0).size > 32 as size_t {
                    return 0 as size_t;
                }
                i = 0 as ::core::ffi::c_uint;
                while (i as size_t) < words {
                    *classes.offset(i as isize) = X86_64_NO_CLASS;
                    i = i.wrapping_add(1);
                }
                if words == 0 {
                    current_block_80 = 1983324216492344030;
                } else {
                    ptr = (*type_0).elements as *mut *mut ffi_type;
                    while !(*ptr).is_null() {
                        let mut num: size_t = 0;
                        let mut pos: size_t = 0;
                        byte_offset = (byte_offset.wrapping_sub(1 as size_t)
                            | ((**ptr).alignment as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as size_t)
                            .wrapping_add(1 as size_t);
                        num = classify_argument(
                            *ptr,
                            &raw mut subclasses as *mut x86_64_reg_class,
                            byte_offset.wrapping_rem(8 as size_t),
                        );
                        if num == 0 as size_t {
                            return 0 as size_t;
                        }
                        pos = byte_offset.wrapping_div(8 as size_t);
                        i = 0 as ::core::ffi::c_uint;
                        while (i as size_t) < num && (i as size_t).wrapping_add(pos) < words {
                            let mut pos_0: size_t = byte_offset.wrapping_div(8 as size_t);
                            *classes.offset((i as size_t).wrapping_add(pos_0) as isize) =
                                merge_classes(
                                    subclasses[i as usize],
                                    *classes.offset((i as size_t).wrapping_add(pos_0) as isize),
                                );
                            i = i.wrapping_add(1);
                        }
                        byte_offset = byte_offset.wrapping_add((**ptr).size);
                        ptr = ptr.offset(1);
                    }
                    if words > 2 as size_t {
                        if *classes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                            != X86_64_SSE_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            return 0 as size_t;
                        }
                        i = 1 as ::core::ffi::c_uint;
                        while (i as size_t) < words {
                            if *classes.offset(i as isize) as ::core::ffi::c_uint
                                != X86_64_SSEUP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                return 0 as size_t;
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                    i = 0 as ::core::ffi::c_uint;
                    while (i as size_t) < words {
                        if *classes.offset(i as isize) as ::core::ffi::c_uint
                            == X86_64_MEMORY_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            return 0 as size_t;
                        }
                        if i > 1 as ::core::ffi::c_uint
                            && *classes.offset(i as isize) as ::core::ffi::c_uint
                                == X86_64_SSEUP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                            && *classes.offset(i.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
                                as ::core::ffi::c_uint
                                != X86_64_SSE_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                            && *classes.offset(i.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
                                as ::core::ffi::c_uint
                                != X86_64_SSEUP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            *classes.offset(i as isize) = X86_64_SSE_CLASS;
                        }
                        if i > 1 as ::core::ffi::c_uint
                            && *classes.offset(i as isize) as ::core::ffi::c_uint
                                == X86_64_X87UP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                            && *classes.offset(i.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
                                as ::core::ffi::c_uint
                                != X86_64_X87_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            return 0 as size_t;
                        }
                        i = i.wrapping_add(1);
                    }
                    return words;
                }
            }
            FFI_TYPE_VOID => {
                current_block_80 = 1983324216492344030;
            }
            FFI_TYPE_COMPLEX => {
                let mut inner: *mut ffi_type =
                    *(*type_0).elements.offset(0 as ::core::ffi::c_int as isize) as *mut ffi_type;
                match (*inner).type_0 as ::core::ffi::c_int {
                    FFI_TYPE_INT | FFI_TYPE_UINT8 | FFI_TYPE_SINT8 | FFI_TYPE_UINT16
                    | FFI_TYPE_SINT16 | FFI_TYPE_UINT32 | FFI_TYPE_SINT32 | FFI_TYPE_UINT64
                    | FFI_TYPE_SINT64 => {
                        current_block_80 = 9317908997127676919;
                    }
                    FFI_TYPE_SINT128 | FFI_TYPE_UINT128 => {
                        current_block_80 = 15277662988523915720;
                        match current_block_80 {
                            15277662988523915720 => return 0 as size_t,
                            554624747196869196 => {
                                *classes.offset(0 as ::core::ffi::c_int as isize) =
                                    X86_64_SSE_CLASS;
                                if byte_offset.wrapping_rem(8 as size_t) != 0 {
                                    *classes.offset(1 as ::core::ffi::c_int as isize) =
                                        X86_64_SSESF_CLASS;
                                    return 2 as size_t;
                                }
                                return 1 as size_t;
                            }
                            6962451677225946820 => {
                                *classes.offset(0 as ::core::ffi::c_int as isize) =
                                    X86_64_COMPLEX_X87_CLASS;
                                return 1 as size_t;
                            }
                            _ => {
                                let ref mut fresh4 =
                                    *classes.offset(1 as ::core::ffi::c_int as isize);
                                *fresh4 = X86_64_SSEDF_CLASS;
                                *classes.offset(0 as ::core::ffi::c_int as isize) = *fresh4;
                                return 2 as size_t;
                            }
                        }
                    }
                    FFI_TYPE_FLOAT => {
                        current_block_80 = 554624747196869196;
                        match current_block_80 {
                            15277662988523915720 => return 0 as size_t,
                            554624747196869196 => {
                                *classes.offset(0 as ::core::ffi::c_int as isize) =
                                    X86_64_SSE_CLASS;
                                if byte_offset.wrapping_rem(8 as size_t) != 0 {
                                    *classes.offset(1 as ::core::ffi::c_int as isize) =
                                        X86_64_SSESF_CLASS;
                                    return 2 as size_t;
                                }
                                return 1 as size_t;
                            }
                            6962451677225946820 => {
                                *classes.offset(0 as ::core::ffi::c_int as isize) =
                                    X86_64_COMPLEX_X87_CLASS;
                                return 1 as size_t;
                            }
                            _ => {
                                let ref mut fresh4 =
                                    *classes.offset(1 as ::core::ffi::c_int as isize);
                                *fresh4 = X86_64_SSEDF_CLASS;
                                *classes.offset(0 as ::core::ffi::c_int as isize) = *fresh4;
                                return 2 as size_t;
                            }
                        }
                    }
                    FFI_TYPE_DOUBLE => {
                        current_block_80 = 13844092875727763097;
                        match current_block_80 {
                            15277662988523915720 => return 0 as size_t,
                            554624747196869196 => {
                                *classes.offset(0 as ::core::ffi::c_int as isize) =
                                    X86_64_SSE_CLASS;
                                if byte_offset.wrapping_rem(8 as size_t) != 0 {
                                    *classes.offset(1 as ::core::ffi::c_int as isize) =
                                        X86_64_SSESF_CLASS;
                                    return 2 as size_t;
                                }
                                return 1 as size_t;
                            }
                            6962451677225946820 => {
                                *classes.offset(0 as ::core::ffi::c_int as isize) =
                                    X86_64_COMPLEX_X87_CLASS;
                                return 1 as size_t;
                            }
                            _ => {
                                let ref mut fresh4 =
                                    *classes.offset(1 as ::core::ffi::c_int as isize);
                                *fresh4 = X86_64_SSEDF_CLASS;
                                *classes.offset(0 as ::core::ffi::c_int as isize) = *fresh4;
                                return 2 as size_t;
                            }
                        }
                    }
                    FFI_TYPE_LONGDOUBLE => {
                        current_block_80 = 6962451677225946820;
                        match current_block_80 {
                            15277662988523915720 => return 0 as size_t,
                            554624747196869196 => {
                                *classes.offset(0 as ::core::ffi::c_int as isize) =
                                    X86_64_SSE_CLASS;
                                if byte_offset.wrapping_rem(8 as size_t) != 0 {
                                    *classes.offset(1 as ::core::ffi::c_int as isize) =
                                        X86_64_SSESF_CLASS;
                                    return 2 as size_t;
                                }
                                return 1 as size_t;
                            }
                            6962451677225946820 => {
                                *classes.offset(0 as ::core::ffi::c_int as isize) =
                                    X86_64_COMPLEX_X87_CLASS;
                                return 1 as size_t;
                            }
                            _ => {
                                let ref mut fresh4 =
                                    *classes.offset(1 as ::core::ffi::c_int as isize);
                                *fresh4 = X86_64_SSEDF_CLASS;
                                *classes.offset(0 as ::core::ffi::c_int as isize) = *fresh4;
                                return 2 as size_t;
                            }
                        }
                    }
                    _ => {
                        current_block_80 = 5793491756164225964;
                    }
                }
            }
            _ => {
                current_block_80 = 5793491756164225964;
            }
        }
        match current_block_80 {
            1983324216492344030 => {
                *classes.offset(0 as ::core::ffi::c_int as isize) = X86_64_NO_CLASS;
                return 1 as size_t;
            }
            9317908997127676919 => {
                let mut size: size_t = byte_offset.wrapping_add((*type_0).size);
                if size <= 4 as size_t {
                    *classes.offset(0 as ::core::ffi::c_int as isize) = X86_64_INTEGERSI_CLASS;
                    return 1 as size_t;
                } else if size <= 8 as size_t {
                    *classes.offset(0 as ::core::ffi::c_int as isize) = X86_64_INTEGER_CLASS;
                    return 1 as size_t;
                } else if size <= 12 as size_t {
                    *classes.offset(0 as ::core::ffi::c_int as isize) = X86_64_INTEGER_CLASS;
                    *classes.offset(1 as ::core::ffi::c_int as isize) = X86_64_INTEGERSI_CLASS;
                    return 2 as size_t;
                } else if size <= 16 as size_t {
                    let ref mut fresh3 = *classes.offset(1 as ::core::ffi::c_int as isize);
                    *fresh3 = X86_64_INTEGER_CLASS;
                    *classes.offset(0 as ::core::ffi::c_int as isize) = *fresh3;
                    return 2 as size_t;
                }
            }
            5793491756164225964 => {
                break 's_390;
            }
            _ => {}
        }
        if byte_offset.wrapping_rem(8 as size_t) == 0 {
            *classes.offset(0 as ::core::ffi::c_int as isize) = X86_64_SSESF_CLASS;
        } else {
            *classes.offset(0 as ::core::ffi::c_int as isize) = X86_64_SSE_CLASS;
        }
        return 1 as size_t;
    }
    abort();
}
unsafe extern "C" fn examine_argument(
    mut type_0: *mut ffi_type,
    mut classes: *mut x86_64_reg_class,
    mut in_return: bool,
    mut pngpr: *mut ::core::ffi::c_int,
    mut pnsse: *mut ::core::ffi::c_int,
) -> size_t {
    let mut n: size_t = 0;
    let mut i: ::core::ffi::c_uint = 0;
    let mut ngpr: ::core::ffi::c_int = 0;
    let mut nsse: ::core::ffi::c_int = 0;
    n = classify_argument(type_0, classes as *mut x86_64_reg_class, 0 as size_t);
    if n == 0 as size_t {
        return 0 as size_t;
    }
    nsse = 0 as ::core::ffi::c_int;
    ngpr = nsse;
    i = 0 as ::core::ffi::c_uint;
    while (i as size_t) < n {
        match *classes.offset(i as isize) as ::core::ffi::c_uint {
            1 | 2 => {
                ngpr += 1;
            }
            3 | 4 | 5 => {
                nsse += 1;
            }
            0 | 6 => {}
            7 | 8 | 9 => {
                return (in_return as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as size_t;
            }
            _ => {
                abort();
            }
        }
        i = i.wrapping_add(1);
    }
    *pngpr = ngpr;
    *pnsse = nsse;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep(mut cif: *mut ffi_cif) -> ffi_status {
    let mut gprcount: ::core::ffi::c_int = 0;
    let mut ssecount: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut avn: ::core::ffi::c_int = 0;
    let mut ngpr: ::core::ffi::c_int = 0;
    let mut nsse: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_uint = 0;
    let mut classes: [x86_64_reg_class; 4] = [X86_64_NO_CLASS; 4];
    let mut bytes: size_t = 0;
    let mut n: size_t = 0;
    let mut rtype_size: size_t = 0;
    let mut rtype: *mut ffi_type = ::core::ptr::null_mut::<ffi_type>();
    if (*cif).abi as ::core::ffi::c_uint != FFI_UNIX64 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return FFI_BAD_ABI;
    }
    ssecount = 0 as ::core::ffi::c_int;
    gprcount = ssecount;
    rtype = (*cif).rtype;
    rtype_size = (*rtype).size;
    match (*rtype).type_0 as ::core::ffi::c_int {
        FFI_TYPE_VOID => {
            flags = UNIX64_RET_VOID as ::core::ffi::c_uint;
        }
        FFI_TYPE_UINT8 => {
            flags = UNIX64_RET_UINT8 as ::core::ffi::c_uint;
        }
        FFI_TYPE_SINT8 => {
            flags = UNIX64_RET_SINT8 as ::core::ffi::c_uint;
        }
        FFI_TYPE_UINT16 => {
            flags = UNIX64_RET_UINT16 as ::core::ffi::c_uint;
        }
        FFI_TYPE_SINT16 => {
            flags = UNIX64_RET_SINT16 as ::core::ffi::c_uint;
        }
        FFI_TYPE_UINT32 => {
            flags = UNIX64_RET_UINT32 as ::core::ffi::c_uint;
        }
        FFI_TYPE_INT | FFI_TYPE_SINT32 => {
            flags = UNIX64_RET_SINT32 as ::core::ffi::c_uint;
        }
        FFI_TYPE_UINT64 | FFI_TYPE_SINT64 => {
            flags = UNIX64_RET_INT64 as ::core::ffi::c_uint;
        }
        FFI_TYPE_UINT128 | FFI_TYPE_SINT128 => {
            flags = (UNIX64_RET_ST_RAX_RDX | (16 as ::core::ffi::c_int) << UNIX64_SIZE_SHIFT)
                as ::core::ffi::c_uint;
        }
        FFI_TYPE_POINTER => {
            flags = (if ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize == 4 as usize {
                UNIX64_RET_UINT32
            } else {
                UNIX64_RET_INT64
            }) as ::core::ffi::c_uint;
        }
        FFI_TYPE_FLOAT => {
            flags = UNIX64_RET_XMM32 as ::core::ffi::c_uint;
        }
        FFI_TYPE_DOUBLE => {
            flags = UNIX64_RET_XMM64 as ::core::ffi::c_uint;
        }
        FFI_TYPE_LONGDOUBLE => {
            flags = UNIX64_RET_X87 as ::core::ffi::c_uint;
        }
        FFI_TYPE_STRUCT => {
            n = examine_argument(
                (*cif).rtype,
                &raw mut classes as *mut x86_64_reg_class,
                1 as ::core::ffi::c_int != 0,
                &raw mut ngpr,
                &raw mut nsse,
            );
            if n == 0 as size_t {
                gprcount += 1;
                flags = (UNIX64_RET_VOID | UNIX64_FLAG_RET_IN_MEM) as ::core::ffi::c_uint;
            } else {
                let mut sse0: bool = classes[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_uint
                    >= X86_64_SSE_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                    && classes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                        <= X86_64_SSEUP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint;
                if rtype_size == 4 as size_t && sse0 as ::core::ffi::c_int != 0 {
                    flags = UNIX64_RET_XMM32 as ::core::ffi::c_uint;
                } else if rtype_size == 8 as size_t {
                    flags = (if sse0 as ::core::ffi::c_int != 0 {
                        UNIX64_RET_XMM64
                    } else {
                        UNIX64_RET_INT64
                    }) as ::core::ffi::c_uint;
                } else {
                    let mut sse1: bool = n == 2 as size_t
                        && (classes[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                            >= X86_64_SSE_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                            && classes[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                                <= X86_64_SSEUP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint);
                    if sse0 as ::core::ffi::c_int != 0 && sse1 as ::core::ffi::c_int != 0 {
                        flags = UNIX64_RET_ST_XMM0_XMM1 as ::core::ffi::c_uint;
                    } else if sse0 {
                        flags = UNIX64_RET_ST_XMM0_RAX as ::core::ffi::c_uint;
                    } else if sse1 {
                        flags = UNIX64_RET_ST_RAX_XMM0 as ::core::ffi::c_uint;
                    } else {
                        flags = UNIX64_RET_ST_RAX_RDX as ::core::ffi::c_uint;
                    }
                    flags |= (rtype_size << UNIX64_SIZE_SHIFT) as ::core::ffi::c_uint;
                }
            }
        }
        FFI_TYPE_COMPLEX => {
            match (**(*rtype).elements.offset(0 as ::core::ffi::c_int as isize)).type_0
                as ::core::ffi::c_int
            {
                FFI_TYPE_UINT8 | FFI_TYPE_SINT8 | FFI_TYPE_UINT16 | FFI_TYPE_SINT16
                | FFI_TYPE_INT | FFI_TYPE_UINT32 | FFI_TYPE_SINT32 | FFI_TYPE_UINT64
                | FFI_TYPE_SINT64 => {
                    flags = UNIX64_RET_ST_RAX_RDX as ::core::ffi::c_uint
                        | (rtype_size as ::core::ffi::c_uint) << UNIX64_SIZE_SHIFT;
                }
                FFI_TYPE_FLOAT => {
                    flags = UNIX64_RET_XMM64 as ::core::ffi::c_uint;
                }
                FFI_TYPE_DOUBLE => {
                    flags = (UNIX64_RET_ST_XMM0_XMM1
                        | (16 as ::core::ffi::c_int) << UNIX64_SIZE_SHIFT)
                        as ::core::ffi::c_uint;
                }
                FFI_TYPE_LONGDOUBLE => {
                    flags = UNIX64_RET_X87_2 as ::core::ffi::c_uint;
                }
                FFI_TYPE_SINT128 | FFI_TYPE_UINT128 => {
                    gprcount += 1;
                    flags = (UNIX64_RET_VOID | UNIX64_FLAG_RET_IN_MEM) as ::core::ffi::c_uint;
                }
                _ => return FFI_BAD_TYPEDEF,
            }
        }
        _ => return FFI_BAD_TYPEDEF,
    }
    bytes = 0 as size_t;
    i = 0 as ::core::ffi::c_int;
    avn = (*cif).nargs as ::core::ffi::c_int;
    while i < avn {
        if examine_argument(
            *(*cif).arg_types.offset(i as isize),
            &raw mut classes as *mut x86_64_reg_class,
            0 as ::core::ffi::c_int != 0,
            &raw mut ngpr,
            &raw mut nsse,
        ) == 0 as size_t
            || gprcount + ngpr > MAX_GPR_REGS
            || ssecount + nsse > MAX_SSE_REGS
        {
            let mut align: ::core::ffi::c_long =
                (**(*cif).arg_types.offset(i as isize)).alignment as ::core::ffi::c_long;
            if align < 8 as ::core::ffi::c_long {
                align = 8 as ::core::ffi::c_long;
            }
            bytes = (bytes.wrapping_sub(1 as size_t) as ::core::ffi::c_ulong
                | (align - 1 as ::core::ffi::c_long) as ::core::ffi::c_ulong)
                .wrapping_add(1 as ::core::ffi::c_ulong) as size_t;
            bytes = bytes.wrapping_add((**(*cif).arg_types.offset(i as isize)).size);
        } else {
            gprcount += ngpr;
            ssecount += nsse;
        }
        i += 1;
    }
    if ssecount != 0 {
        flags |= UNIX64_FLAG_XMM_ARGS as ::core::ffi::c_uint;
    }
    (*cif).flags = flags;
    (*cif).bytes = (bytes.wrapping_sub(1 as size_t)
        | (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
        .wrapping_add(1 as size_t) as ::core::ffi::c_uint;
    return FFI_OK;
}
unsafe extern "C" fn ffi_call_int(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
    mut closure: *mut ::core::ffi::c_void,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut classes: [x86_64_reg_class; 4] = [X86_64_NO_CLASS; 4];
    let mut stack: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut argp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut arg_types: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut gprcount: ::core::ffi::c_int = 0;
    let mut ssecount: ::core::ffi::c_int = 0;
    let mut ngpr: ::core::ffi::c_int = 0;
    let mut nsse: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut avn: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut reg_args: *mut register_args = ::core::ptr::null_mut::<register_args>();
    flags = (*cif).flags as ::core::ffi::c_int;
    if rvalue.is_null() {
        if flags & UNIX64_FLAG_RET_IN_MEM != 0 {
            alloca_allocations.push(::std::vec::from_elem(0, (*(*cif).rtype).size as usize));
            rvalue = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast();
        } else {
            flags = UNIX64_RET_VOID;
        }
    }
    arg_types = (*cif).arg_types;
    avn = (*cif).nargs as ::core::ffi::c_int;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<register_args>() as usize)
            .wrapping_add((*cif).bytes as usize)
            .wrapping_add((4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize)
            as usize,
    ));
    stack = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    reg_args = stack as *mut register_args;
    argp = stack.offset(::core::mem::size_of::<register_args>() as usize as isize);
    (*reg_args).r10 = closure as uintptr_t as UINT64;
    ssecount = 0 as ::core::ffi::c_int;
    gprcount = ssecount;
    if flags & UNIX64_FLAG_RET_IN_MEM != 0 {
        let fresh0 = gprcount;
        gprcount = gprcount + 1;
        (*reg_args).gpr[fresh0 as usize] = rvalue as ::core::ffi::c_ulong as UINT64;
    }
    i = 0 as ::core::ffi::c_int;
    while i < avn {
        let mut n: size_t = 0;
        let mut size: size_t = (**arg_types.offset(i as isize)).size;
        n = examine_argument(
            *arg_types.offset(i as isize),
            &raw mut classes as *mut x86_64_reg_class,
            0 as ::core::ffi::c_int != 0,
            &raw mut ngpr,
            &raw mut nsse,
        );
        if n == 0 as size_t || gprcount + ngpr > MAX_GPR_REGS || ssecount + nsse > MAX_SSE_REGS {
            let mut align: ::core::ffi::c_long =
                (**arg_types.offset(i as isize)).alignment as ::core::ffi::c_long;
            if align < 8 as ::core::ffi::c_long {
                align = 8 as ::core::ffi::c_long;
            }
            argp = ((argp as size_t).wrapping_sub(1 as size_t) as ::core::ffi::c_ulong
                | (align - 1 as ::core::ffi::c_long) as ::core::ffi::c_ulong)
                .wrapping_add(1 as ::core::ffi::c_ulong)
                as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char;
            memcpy(
                argp as *mut ::core::ffi::c_void,
                *avalue.offset(i as isize),
                size,
            );
            argp = argp.offset(size as isize);
        } else {
            let mut a: *mut ::core::ffi::c_char =
                *avalue.offset(i as isize) as *mut ::core::ffi::c_char;
            let mut j: ::core::ffi::c_uint = 0;
            j = 0 as ::core::ffi::c_uint;
            while (j as size_t) < n {
                match classes[j as usize] as ::core::ffi::c_uint {
                    0 => {}
                    6 => {
                        memcpy(
                            ((&raw mut (*reg_args).sse as *mut big_int_union)
                                .offset((ssecount - 1 as ::core::ffi::c_int) as isize)
                                as *mut big_int_union
                                as *mut ::core::ffi::c_char)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_void,
                            a as *const ::core::ffi::c_void,
                            if size < 8 as size_t {
                                size
                            } else {
                                8 as size_t
                            },
                        );
                    }
                    1 | 2 => {
                        match (**arg_types.offset(i as isize)).type_0 as ::core::ffi::c_int {
                            FFI_TYPE_SINT8 => {
                                (*reg_args).gpr[gprcount as usize] =
                                    *(a as *mut SINT8) as SINT64 as UINT64;
                            }
                            FFI_TYPE_SINT16 => {
                                (*reg_args).gpr[gprcount as usize] =
                                    *(a as *mut SINT16) as SINT64 as UINT64;
                            }
                            FFI_TYPE_SINT32 => {
                                (*reg_args).gpr[gprcount as usize] =
                                    *(a as *mut SINT32) as SINT64 as UINT64;
                            }
                            _ => {
                                (*reg_args).gpr[gprcount as usize] = 0 as UINT64;
                                memcpy(
                                    (&raw mut (*reg_args).gpr as *mut UINT64)
                                        .offset(gprcount as isize)
                                        as *mut UINT64
                                        as *mut ::core::ffi::c_void,
                                    a as *const ::core::ffi::c_void,
                                    if size <= 8 as size_t {
                                        size
                                    } else {
                                        8 as size_t
                                    },
                                );
                            }
                        }
                        gprcount += 1;
                    }
                    3 | 5 => {
                        let fresh1 = ssecount;
                        ssecount = ssecount + 1;
                        memcpy(
                            &raw mut (*(&raw mut (*reg_args).sse as *mut big_int_union)
                                .offset(fresh1 as isize))
                            .i64_0 as *mut ::core::ffi::c_void,
                            a as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<UINT64>() as size_t,
                        );
                    }
                    4 => {
                        let fresh2 = ssecount;
                        ssecount = ssecount + 1;
                        memcpy(
                            &raw mut (*(&raw mut (*reg_args).sse as *mut big_int_union)
                                .offset(fresh2 as isize))
                            .i32_0 as *mut ::core::ffi::c_void,
                            a as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<UINT32>() as size_t,
                        );
                    }
                    _ => {
                        abort();
                    }
                }
                j = j.wrapping_add(1);
                a = a.offset(8 as ::core::ffi::c_int as isize);
                size = size.wrapping_sub(8 as size_t);
            }
        }
        i += 1;
    }
    (*reg_args).rax = ssecount as UINT64;
    ffi_call_unix64_from_rust(
        stack as *mut ::core::ffi::c_void,
        ((*cif).bytes as usize).wrapping_add(::core::mem::size_of::<register_args>() as usize)
            as ::core::ffi::c_ulong,
        flags as ::core::ffi::c_uint,
        rvalue,
        fn_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ffi_call(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut arg_types: *mut *mut ffi_type = (*cif).arg_types;
    let mut i: ::core::ffi::c_int = 0;
    let mut nargs: ::core::ffi::c_int = (*cif).nargs as ::core::ffi::c_int;
    let max_reg_struct_size: ::core::ffi::c_int = if (*cif).abi as ::core::ffi::c_uint
        == FFI_GNUW64 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        8 as ::core::ffi::c_int
    } else {
        16 as ::core::ffi::c_int
    };
    i = 0 as ::core::ffi::c_int;
    while i < nargs {
        let mut at: *mut ffi_type = *arg_types.offset(i as isize);
        let mut size: ::core::ffi::c_int = (*at).size as ::core::ffi::c_int;
        if (*at).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT && size > max_reg_struct_size {
            alloca_allocations.push(::std::vec::from_elem(
                0,
                size as ::core::ffi::c_uint as usize,
            ));
            let mut argcopy: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                argcopy as *mut ::core::ffi::c_void,
                *avalue.offset(i as isize),
                size as size_t,
            );
            let ref mut fresh5 = *avalue.offset(i as isize);
            *fresh5 = argcopy as *mut ::core::ffi::c_void;
        }
        i += 1;
    }
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
    static mut trampoline: [::core::ffi::c_uchar; 24] = [
        0xf3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1e as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xfa as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x8d as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xf5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    let mut dest: Option<unsafe extern "C" fn() -> ()> = None;
    let mut tramp: *mut ::core::ffi::c_char =
        &raw mut (*closure).c2rust_unnamed.tramp as *mut ::core::ffi::c_char;
    if (*cif).abi as ::core::ffi::c_uint != FFI_UNIX64 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return FFI_BAD_ABI;
    }
    if (*cif).flags & UNIX64_FLAG_XMM_ARGS as ::core::ffi::c_uint != 0 {
        dest = Some(ffi_closure_unix64_sse as unsafe extern "C" fn() -> ())
            as Option<unsafe extern "C" fn() -> ()>;
    } else {
        dest = Some(ffi_closure_unix64 as unsafe extern "C" fn() -> ())
            as Option<unsafe extern "C" fn() -> ()>;
    }
    if ffi_tramp_is_present(closure as *mut ::core::ffi::c_void) != 0 {
        if dest == Some(ffi_closure_unix64_sse as unsafe extern "C" fn() -> ()) {
            dest = Some(ffi_closure_unix64_sse_alt as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>;
        } else {
            dest = Some(ffi_closure_unix64_alt as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>;
        }
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
            &raw const trampoline as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 24]>() as size_t,
        );
        *(tramp.offset(::core::mem::size_of::<[::core::ffi::c_uchar; 24]>() as usize as isize)
            as *mut UINT64) =
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, uintptr_t>(dest)
                as UINT64;
    }
    (*closure).cif = cif;
    (*closure).fun = fun;
    (*closure).user_data = user_data;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_unix64_inner(
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
    mut reg_args: *mut register_args,
    mut argp: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut avalue: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut arg_types: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut i: ::core::ffi::c_long = 0;
    let mut avn: ::core::ffi::c_long = 0;
    let mut gprcount: ::core::ffi::c_int = 0;
    let mut ssecount: ::core::ffi::c_int = 0;
    let mut ngpr: ::core::ffi::c_int = 0;
    let mut nsse: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    avn = (*cif).nargs as ::core::ffi::c_long;
    flags = (*cif).flags as ::core::ffi::c_int;
    alloca_allocations.push(
        ::std::vec::from_elem(
            0,
            (avn as ::core::ffi::c_ulong).wrapping_mul(::core::mem::size_of::<
                *mut ::core::ffi::c_void,
            >() as ::core::ffi::c_ulong) as ::core::ffi::c_uint as usize,
        ),
    );
    avalue = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_void;
    ssecount = 0 as ::core::ffi::c_int;
    gprcount = ssecount;
    if flags & UNIX64_FLAG_RET_IN_MEM != 0 {
        let fresh6 = gprcount;
        gprcount = gprcount + 1;
        let mut r: *mut ::core::ffi::c_void =
            (*reg_args).gpr[fresh6 as usize] as uintptr_t as *mut ::core::ffi::c_void;
        let ref mut fresh7 = *(rvalue as *mut *mut ::core::ffi::c_void);
        *fresh7 = r;
        rvalue = r;
        flags = if ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize == 4 as usize {
            UNIX64_RET_UINT32
        } else {
            UNIX64_RET_INT64
        };
    }
    arg_types = (*cif).arg_types;
    i = 0 as ::core::ffi::c_long;
    while i < avn {
        let mut classes: [x86_64_reg_class; 4] = [X86_64_NO_CLASS; 4];
        let mut n: size_t = 0;
        n = examine_argument(
            *arg_types.offset(i as isize),
            &raw mut classes as *mut x86_64_reg_class,
            0 as ::core::ffi::c_int != 0,
            &raw mut ngpr,
            &raw mut nsse,
        );
        if n == 0 as size_t || gprcount + ngpr > MAX_GPR_REGS || ssecount + nsse > MAX_SSE_REGS {
            let mut align: ::core::ffi::c_long =
                (**arg_types.offset(i as isize)).alignment as ::core::ffi::c_long;
            if align < 8 as ::core::ffi::c_long {
                align = 8 as ::core::ffi::c_long;
            }
            argp = ((argp as size_t).wrapping_sub(1 as size_t) as ::core::ffi::c_ulong
                | (align - 1 as ::core::ffi::c_long) as ::core::ffi::c_ulong)
                .wrapping_add(1 as ::core::ffi::c_ulong)
                as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char;
            let ref mut fresh8 = *avalue.offset(i as isize);
            *fresh8 = argp as *mut ::core::ffi::c_void;
            argp = argp.offset((**arg_types.offset(i as isize)).size as isize);
        } else if n == 1 as size_t
            || n == 2 as size_t
                && !(classes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                    >= X86_64_SSE_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                    && classes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                        <= X86_64_SSEUP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                    || classes[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                        >= X86_64_SSE_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                        && classes[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                            <= X86_64_SSEUP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            if classes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                >= X86_64_SSE_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                && classes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint
                    <= X86_64_SSEUP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let ref mut fresh9 = *avalue.offset(i as isize);
                *fresh9 = (&raw mut (*reg_args).sse as *mut big_int_union).offset(ssecount as isize)
                    as *mut big_int_union as *mut ::core::ffi::c_void;
                ssecount = (ssecount as size_t).wrapping_add(n) as ::core::ffi::c_int
                    as ::core::ffi::c_int;
            } else {
                let ref mut fresh10 = *avalue.offset(i as isize);
                *fresh10 = (&raw mut (*reg_args).gpr as *mut UINT64).offset(gprcount as isize)
                    as *mut UINT64 as *mut ::core::ffi::c_void;
                gprcount = (gprcount as size_t).wrapping_add(n) as ::core::ffi::c_int
                    as ::core::ffi::c_int;
            }
        } else {
            alloca_allocations.push(::std::vec::from_elem(
                0,
                16 as ::core::ffi::c_int as ::core::ffi::c_uint as usize,
            ));
            let mut a: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            let mut j: ::core::ffi::c_uint = 0;
            let ref mut fresh11 = *avalue.offset(i as isize);
            *fresh11 = a as *mut ::core::ffi::c_void;
            j = 0 as ::core::ffi::c_uint;
            while (j as size_t) < n {
                if classes[j as usize] as ::core::ffi::c_uint
                    == X86_64_SSEUP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    memcpy(
                        a as *mut ::core::ffi::c_void,
                        ((&raw mut (*reg_args).sse as *mut big_int_union)
                            .offset((ssecount - 1 as ::core::ffi::c_int) as isize)
                            as *mut big_int_union
                            as *mut ::core::ffi::c_char)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                } else if classes[j as usize] as ::core::ffi::c_uint
                    >= X86_64_SSE_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                    && classes[j as usize] as ::core::ffi::c_uint
                        <= X86_64_SSEUP_CLASS as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let fresh12 = ssecount;
                    ssecount = ssecount + 1;
                    memcpy(
                        a as *mut ::core::ffi::c_void,
                        (&raw mut (*reg_args).sse as *mut big_int_union).offset(fresh12 as isize)
                            as *mut big_int_union
                            as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                } else {
                    let fresh13 = gprcount;
                    gprcount = gprcount + 1;
                    memcpy(
                        a as *mut ::core::ffi::c_void,
                        (&raw mut (*reg_args).gpr as *mut UINT64).offset(fresh13 as isize)
                            as *mut UINT64 as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                }
                j = j.wrapping_add(1);
                a = a.offset(8 as ::core::ffi::c_int as isize);
            }
        }
        i += 1;
    }
    fun.expect("non-null function pointer")(cif, rvalue, avalue, user_data);
    return flags;
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
    if (*cif).abi as ::core::ffi::c_uint != FFI_UNIX64 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return FFI_BAD_ABI;
    }
    (*closure).tramp =
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, *mut ::core::ffi::c_void>(
            if (*cif).flags & UNIX64_FLAG_XMM_ARGS as ::core::ffi::c_uint != 0 {
                Some(ffi_go_closure_unix64_sse as unsafe extern "C" fn() -> ())
            } else {
                Some(ffi_go_closure_unix64 as unsafe extern "C" fn() -> ())
            },
        );
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
    *map_size = UNIX64_TRAMP_MAP_SIZE as size_t;
    *tramp_size = UNIX64_TRAMP_SIZE as size_t;
    return &raw mut trampoline_code_table as *mut ::core::ffi::c_void;
}
