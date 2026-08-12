use core::arch::asm;

extern "C" {
    fn abort() -> !;
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn ffi_call_SYSV(
        context: *mut call_context,
        frame: *mut core::ffi::c_void,
        fn_0: Option<unsafe extern "C" fn() -> ()>,
        rvalue: *mut core::ffi::c_void,
        flags: core::ffi::c_int,
        closure: *mut core::ffi::c_void,
    );
    fn ffi_closure_SYSV();
    fn ffi_closure_SYSV_V();
}

pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
pub type uint8_t = u8;
pub type ffi_arg = core::ffi::c_ulong;
pub type ffi_abi = core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 1;
pub const FFI_LAST_ABI: ffi_abi = 3;
pub const FFI_WIN64: ffi_abi = 2;
pub const FFI_SYSV: ffi_abi = 1;
pub const FFI_FIRST_ABI: ffi_abi = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ffi_type {
    pub size: size_t,
    pub alignment: core::ffi::c_ushort,
    pub type_0: core::ffi::c_ushort,
    pub elements: *mut *mut _ffi_type,
}
pub type ffi_type = _ffi_type;
pub type ffi_status = core::ffi::c_uint;
pub const FFI_BAD_ARGTYPE: ffi_status = 3;
pub const FFI_BAD_ABI: ffi_status = 2;
pub const FFI_BAD_TYPEDEF: ffi_status = 1;
pub const FFI_OK: ffi_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_cif {
    pub abi: ffi_abi,
    pub nargs: core::ffi::c_uint,
    pub arg_types: *mut *mut ffi_type,
    pub rtype: *mut ffi_type,
    pub bytes: core::ffi::c_uint,
    pub flags: core::ffi::c_uint,
    pub aarch64_nfixedargs: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_closure {
    pub trampoline_table: *mut core::ffi::c_void,
    pub trampoline_table_entry: *mut core::ffi::c_void,
    pub cif: *mut ffi_cif,
    pub fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> (),
    >,
    pub user_data: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct call_context {
    pub v: [_v; 8],
    pub x: [UINT64; 8],
}
pub type UINT64 = core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _v {
    pub d: [_d; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _d {
    pub d: UINT64,
    pub s: [UINT32; 2],
}
pub type UINT32 = core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_state {
    pub ngrn: core::ffi::c_uint,
    pub nsrn: core::ffi::c_uint,
    pub nsaa: size_t,
    pub next_struct_area: size_t,
    pub allocating_variadic: core::ffi::c_uint,
}
pub type SINT32 = core::ffi::c_int;
pub type SINT16 = core::ffi::c_short;
pub type UINT16 = core::ffi::c_ushort;
pub type SINT8 = core::ffi::c_schar;
pub type UINT8 = core::ffi::c_uchar;
pub const __DARWIN_NULL: *mut core::ffi::c_void = core::ptr::null_mut::<core::ffi::c_void>();
pub const NULL: *mut core::ffi::c_void = __DARWIN_NULL;
pub const FFI_TYPE_VOID: core::ffi::c_int = 0;
pub const FFI_TYPE_INT: core::ffi::c_int = 1;
pub const FFI_TYPE_FLOAT: core::ffi::c_int = 2;
pub const FFI_TYPE_DOUBLE: core::ffi::c_int = 3;
pub const FFI_TYPE_UINT8: core::ffi::c_int = 5;
pub const FFI_TYPE_SINT8: core::ffi::c_int = 6;
pub const FFI_TYPE_UINT16: core::ffi::c_int = 7;
pub const FFI_TYPE_SINT16: core::ffi::c_int = 8;
pub const FFI_TYPE_UINT32: core::ffi::c_int = 9;
pub const FFI_TYPE_SINT32: core::ffi::c_int = 10;
pub const FFI_TYPE_UINT64: core::ffi::c_int = 11;
pub const FFI_TYPE_SINT64: core::ffi::c_int = 12;
pub const FFI_TYPE_STRUCT: core::ffi::c_int = 13 as core::ffi::c_int;
pub const FFI_TYPE_POINTER: core::ffi::c_int = 14;
pub const FFI_TYPE_COMPLEX: core::ffi::c_int = 15 as core::ffi::c_int;
pub const FFI_TYPE_UINT128: core::ffi::c_int = 16;
pub const FFI_TYPE_SINT128: core::ffi::c_int = 17;
pub const AARCH64_RET_VOID: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AARCH64_RET_INT64: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AARCH64_RET_INT128: core::ffi::c_int = 2 as core::ffi::c_int;
pub const AARCH64_RET_S4: core::ffi::c_int = 8 as core::ffi::c_int;
pub const AARCH64_RET_S3: core::ffi::c_int = 9;
pub const AARCH64_RET_S2: core::ffi::c_int = 10;
pub const AARCH64_RET_S1: core::ffi::c_int = 11;
pub const AARCH64_RET_D4: core::ffi::c_int = 12;
pub const AARCH64_RET_D3: core::ffi::c_int = 13;
pub const AARCH64_RET_D2: core::ffi::c_int = 14;
pub const AARCH64_RET_D1: core::ffi::c_int = 15;
pub const AARCH64_RET_UINT8: core::ffi::c_int = 20 as core::ffi::c_int;
pub const AARCH64_RET_UINT16: core::ffi::c_int = 22 as core::ffi::c_int;
pub const AARCH64_RET_UINT32: core::ffi::c_int = 24 as core::ffi::c_int;
pub const AARCH64_RET_SINT8: core::ffi::c_int = 26 as core::ffi::c_int;
pub const AARCH64_RET_SINT16: core::ffi::c_int = 28 as core::ffi::c_int;
pub const AARCH64_RET_SINT32: core::ffi::c_int = 30 as core::ffi::c_int;
pub const AARCH64_RET_IN_MEM: core::ffi::c_int =
    (1 as core::ffi::c_int) << 5 as core::ffi::c_int;
pub const AARCH64_RET_NEED_COPY: core::ffi::c_int =
    (1 as core::ffi::c_int) << 6 as core::ffi::c_int;
pub const AARCH64_FLAG_ARG_V_BIT: core::ffi::c_int = 7 as core::ffi::c_int;
pub const AARCH64_FLAG_ARG_V: core::ffi::c_int =
    (1 as core::ffi::c_int) << AARCH64_FLAG_ARG_V_BIT;
pub const AARCH64_FLAG_VARARG: core::ffi::c_int =
    (1 as core::ffi::c_int) << 8 as core::ffi::c_int;
pub const N_X_ARG_REG: core::ffi::c_int = 8 as core::ffi::c_int;
pub const N_V_ARG_REG: core::ffi::c_int = 8 as core::ffi::c_int;
pub const FFI_TYPE_LONGDOUBLE: core::ffi::c_int = 4;
pub const PAGE_MAX_SHIFT: core::ffi::c_int = 14 as core::ffi::c_int;
pub const PAGE_MAX_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << PAGE_MAX_SHIFT;
unsafe extern "C" fn is_hfa0(mut ty: *const ffi_type) -> core::ffi::c_int {
    let mut elements: *mut *mut ffi_type = (*ty).elements as *mut *mut ffi_type;
    let mut i: core::ffi::c_int = 0;
    let mut ret: core::ffi::c_int = -(1 as core::ffi::c_int);
    if !elements.is_null() {
        i = 0 as core::ffi::c_int;
        while !(*elements.offset(i as isize)).is_null() {
            ret = (**elements.offset(i as isize)).type_0 as core::ffi::c_int;
            if !(ret == FFI_TYPE_STRUCT || ret == FFI_TYPE_COMPLEX) {
                break;
            }
            ret = is_hfa0(*elements.offset(i as isize));
            if !(ret < 0 as core::ffi::c_int) {
                break;
            }
            i += 1;
        }
    }
    return ret;
}
unsafe extern "C" fn is_hfa1(
    mut ty: *const ffi_type,
    mut candidate: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut elements: *mut *mut ffi_type = (*ty).elements as *mut *mut ffi_type;
    let mut i: core::ffi::c_int = 0;
    if !elements.is_null() {
        i = 0 as core::ffi::c_int;
        while !(*elements.offset(i as isize)).is_null() {
            let mut t: core::ffi::c_int =
                (**elements.offset(i as isize)).type_0 as core::ffi::c_int;
            if t == FFI_TYPE_STRUCT || t == FFI_TYPE_COMPLEX {
                if is_hfa1(*elements.offset(i as isize), candidate) == 0 {
                    return 0 as core::ffi::c_int;
                }
            } else if t != candidate {
                return 0 as core::ffi::c_int;
            }
            i += 1;
        }
    }
    return 1 as core::ffi::c_int;
}
unsafe extern "C" fn is_vfp_type(mut ty: *const ffi_type) -> core::ffi::c_int {
    let mut elements: *mut *mut ffi_type = core::ptr::null_mut::<*mut ffi_type>();
    let mut candidate: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    let mut size: size_t = 0;
    let mut ele_count: size_t = 0;
    candidate = (*ty).type_0 as core::ffi::c_int;
    match candidate {
        FFI_TYPE_FLOAT | FFI_TYPE_DOUBLE | FFI_TYPE_LONGDOUBLE => {
            ele_count = 1 as size_t;
        }
        FFI_TYPE_COMPLEX => {
            candidate = (**(*ty).elements.offset(0 as core::ffi::c_int as isize)).type_0
                as core::ffi::c_int;
            match candidate {
                FFI_TYPE_FLOAT | FFI_TYPE_DOUBLE | FFI_TYPE_LONGDOUBLE => {
                    ele_count = 2 as size_t;
                }
                _ => return 0 as core::ffi::c_int,
            }
        }
        FFI_TYPE_STRUCT => {
            size = (*ty).size;
            if size < 4 as size_t || size > 64 as size_t {
                return 0 as core::ffi::c_int;
            }
            elements = (*ty).elements as *mut *mut ffi_type;
            candidate =
                (**elements.offset(0 as core::ffi::c_int as isize)).type_0 as core::ffi::c_int;
            if candidate == FFI_TYPE_STRUCT || candidate == FFI_TYPE_COMPLEX {
                i = 0 as core::ffi::c_int;
                loop {
                    candidate = is_hfa0(*elements.offset(i as isize));
                    if candidate >= 0 as core::ffi::c_int {
                        break;
                    }
                    i += 1;
                }
            }
            match candidate {
                FFI_TYPE_FLOAT => {
                    ele_count =
                        size.wrapping_div(core::mem::size_of::<core::ffi::c_float>() as size_t);
                    if size
                        != ele_count
                            .wrapping_mul(core::mem::size_of::<core::ffi::c_float>() as size_t)
                    {
                        return 0 as core::ffi::c_int;
                    }
                }
                FFI_TYPE_DOUBLE => {
                    ele_count = size
                        .wrapping_div(core::mem::size_of::<core::ffi::c_double>() as size_t);
                    if size
                        != ele_count
                            .wrapping_mul(core::mem::size_of::<core::ffi::c_double>() as size_t)
                    {
                        return 0 as core::ffi::c_int;
                    }
                }
                FFI_TYPE_LONGDOUBLE => {
                    ele_count = size
                        .wrapping_div(core::mem::size_of::<core::ffi::c_double>() as size_t);
                    if size
                        != ele_count
                            .wrapping_mul(core::mem::size_of::<core::ffi::c_double>() as size_t)
                    {
                        return 0 as core::ffi::c_int;
                    }
                }
                _ => return 0 as core::ffi::c_int,
            }
            if ele_count > 4 as size_t {
                return 0 as core::ffi::c_int;
            }
            i = 0 as core::ffi::c_int;
            while !(*elements.offset(i as isize)).is_null() {
                let mut t: core::ffi::c_int =
                    (**elements.offset(i as isize)).type_0 as core::ffi::c_int;
                if t == FFI_TYPE_STRUCT || t == FFI_TYPE_COMPLEX {
                    if is_hfa1(*elements.offset(i as isize), candidate) == 0 {
                        return 0 as core::ffi::c_int;
                    }
                } else if t != candidate {
                    return 0 as core::ffi::c_int;
                }
                i += 1;
            }
        }
        _ => return 0 as core::ffi::c_int,
    }
    return candidate * 4 as core::ffi::c_int
        + (4 as core::ffi::c_int - ele_count as core::ffi::c_int);
}
unsafe extern "C" fn arg_init(mut state: *mut arg_state, mut size: size_t) {
    (*state).ngrn = 0 as core::ffi::c_uint;
    (*state).nsrn = 0 as core::ffi::c_uint;
    (*state).nsaa = 0 as size_t;
    (*state).next_struct_area = size;
    (*state).allocating_variadic = 0 as core::ffi::c_uint;
}
unsafe extern "C" fn allocate_to_stack(
    mut state: *mut arg_state,
    mut stack: *mut core::ffi::c_void,
    mut alignment: size_t,
    mut size: size_t,
) -> *mut core::ffi::c_void {
    let mut nsaa: size_t = (*state).nsaa;
    if (*state).allocating_variadic != 0 && alignment < 8 as size_t {
        alignment = 8 as size_t;
    }
    nsaa = (nsaa.wrapping_sub(1 as size_t) | alignment.wrapping_sub(1 as size_t))
        .wrapping_add(1 as size_t);
    (*state).nsaa = nsaa.wrapping_add(size);
    return (stack as *mut core::ffi::c_char).offset(nsaa as isize) as *mut core::ffi::c_void;
}
unsafe extern "C" fn allocate_and_copy_struct_to_stack(
    mut state: *mut arg_state,
    mut stack: *mut core::ffi::c_void,
    mut alignment: size_t,
    mut size: size_t,
    mut value: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut dest: size_t = (*state).next_struct_area.wrapping_sub(size);
    dest = dest & alignment.wrapping_neg();
    (*state).next_struct_area = dest;
    return memcpy(
        (stack as *mut core::ffi::c_char).offset(dest as isize) as *mut core::ffi::c_void,
        value,
        size,
    );
}
unsafe extern "C" fn extend_integer_type(
    mut source: *mut core::ffi::c_void,
    mut type_0: core::ffi::c_int,
) -> ffi_arg {
    match type_0 {
        FFI_TYPE_UINT8 => {
            let mut u8: UINT8 = 0;
            memcpy(
                &raw mut u8 as *mut core::ffi::c_void,
                source,
                core::mem::size_of::<UINT8>() as size_t,
            );
            return u8 as ffi_arg;
        }
        FFI_TYPE_SINT8 => {
            let mut s8: SINT8 = 0;
            memcpy(
                &raw mut s8 as *mut core::ffi::c_void,
                source,
                core::mem::size_of::<SINT8>() as size_t,
            );
            return s8 as ffi_arg;
        }
        FFI_TYPE_UINT16 => {
            let mut u16: UINT16 = 0;
            memcpy(
                &raw mut u16 as *mut core::ffi::c_void,
                source,
                core::mem::size_of::<UINT16>() as size_t,
            );
            return u16 as ffi_arg;
        }
        FFI_TYPE_SINT16 => {
            let mut s16: SINT16 = 0;
            memcpy(
                &raw mut s16 as *mut core::ffi::c_void,
                source,
                core::mem::size_of::<SINT16>() as size_t,
            );
            return s16 as ffi_arg;
        }
        FFI_TYPE_UINT32 => {
            let mut u32: UINT32 = 0;
            memcpy(
                &raw mut u32 as *mut core::ffi::c_void,
                source,
                core::mem::size_of::<UINT32>() as size_t,
            );
            return u32 as ffi_arg;
        }
        FFI_TYPE_INT | FFI_TYPE_SINT32 => {
            let mut s32: SINT32 = 0;
            memcpy(
                &raw mut s32 as *mut core::ffi::c_void,
                source,
                core::mem::size_of::<SINT32>() as size_t,
            );
            return s32 as ffi_arg;
        }
        FFI_TYPE_UINT64 | FFI_TYPE_SINT64 => {
            let mut u64: UINT64 = 0;
            memcpy(
                &raw mut u64 as *mut core::ffi::c_void,
                source,
                core::mem::size_of::<UINT64>() as size_t,
            );
            return u64 as ffi_arg;
        }
        FFI_TYPE_POINTER => {
            let mut uptr: uintptr_t = 0;
            memcpy(
                &raw mut uptr as *mut core::ffi::c_void,
                source,
                core::mem::size_of::<uintptr_t>() as size_t,
            );
            return uptr as ffi_arg;
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn extend_hfa_type(
    mut dest: *mut core::ffi::c_void,
    mut src: *mut core::ffi::c_void,
    mut h: core::ffi::c_int,
) {
    let mut f: ssize_t = (h - AARCH64_RET_S4) as ssize_t;
    let mut x0: *mut core::ffi::c_void = core::ptr::null_mut::<core::ffi::c_void>();
    asm!(
        "adr\t{0}, 0f\n", "\tadd\t{0}, {0}, {1}\n", "\tbr\t{0}\n", "0:\thint #36\n",
        "\tldp\ts16, s17, [{3}]\n", "\tldp\ts18, s19, [{3}, #8]\n", "\tb\t4f\n",
        "\thint #36\n", "\tldp\ts16, s17, [{3}]\n", "\tldr\ts18, [{3}, #8]\n",
        "\tb\t3f\n", "\thint #36\n", "\tldp\ts16, s17, [{3}]\n", "\tb\t2f\n", "\tnop\n",
        "\thint #36\n", "\tldr\ts16, [{3}]\n", "\tb\t1f\n", "\tnop\n", "\thint #36\n",
        "\tldp\td16, d17, [{3}]\n", "\tldp\td18, d19, [{3}, #16]\n", "\tb\t4f\n",
        "\thint #36\n", "\tldp\td16, d17, [{3}]\n", "\tldr\td18, [{3}, #16]\n",
        "\tb\t3f\n", "\thint #36\n", "\tldp\td16, d17, [{3}]\n", "\tb\t2f\n", "\tnop\n",
        "\thint #36\n", "\tldr\td16, [{3}]\n", "\tb\t1f\n", "\tnop\n", "\thint #36\n",
        "\tldp\tq16, q17, [{3}]\n", "\tldp\tq18, q19, [{3}, #32]\n", "\tb\t4f\n",
        "\thint #36\n", "\tldp\tq16, q17, [{3}]\n", "\tldr\tq18, [{3}, #32]\n",
        "\tb\t3f\n", "\thint #36\n", "\tldp\tq16, q17, [{3}]\n", "\tb\t2f\n", "\tnop\n",
        "\thint #36\n", "\tldr\tq16, [{3}]\n", "\tb\t1f\n", "4:\tstr\tq19, [{2}, #48]\n",
        "3:\tstr\tq18, [{2}, #32]\n", "2:\tstr\tq17, [{2}, #16]\n",
        "1:\tstr\tq16, [{2}]\n", out(reg) x0, inlateout(reg) f * 16 as ssize_t => _,
        inlateout(reg) dest => _, inlateout(reg) src => _, out("v16") _, out("v17") _,
        out("v18") _, out("v19") _, options(preserves_flags)
    );
}
unsafe extern "C" fn compress_hfa_type(
    mut dest: *mut core::ffi::c_void,
    mut reg: *mut core::ffi::c_void,
    mut h: core::ffi::c_int,
) -> *mut core::ffi::c_void {
    match h {
        AARCH64_RET_S1 => {
            if !(dest == reg) {
                *(dest as *mut core::ffi::c_float) = *(reg as *mut core::ffi::c_float);
            }
        }
        AARCH64_RET_S2 => {
            asm!(
                "ldp q16, q17, [{1}]\n", "\tst2 {{ v16.s, v17.s }}[0], [{0}]\n",
                inlateout(reg) dest => _, inlateout(reg) reg => _, out("v16") _,
                out("v17") _, options(preserves_flags)
            );
        }
        AARCH64_RET_S3 => {
            asm!(
                "ldp q16, q17, [{1}]\n", "\tldr q18, [{1}, #32]\n",
                "\tst3 {{ v16.s, v17.s, v18.s }}[0], [{0}]\n", inlateout(reg) dest => _,
                inlateout(reg) reg => _, out("v16") _, out("v17") _, out("v18") _,
                options(preserves_flags)
            );
        }
        AARCH64_RET_S4 => {
            asm!(
                "ldp q16, q17, [{1}]\n", "\tldp q18, q19, [{1}, #32]\n",
                "\tst4 {{ v16.s, v17.s, v18.s, v19.s }}[0], [{0}]\n", inlateout(reg) dest
                => _, inlateout(reg) reg => _, out("v16") _, out("v17") _, out("v18") _,
                out("v19") _, options(preserves_flags)
            );
        }
        AARCH64_RET_D1 => {
            if !(dest == reg) {
                *(dest as *mut core::ffi::c_double) = *(reg as *mut core::ffi::c_double);
            }
        }
        AARCH64_RET_D2 => {
            asm!(
                "ldp q16, q17, [{1}]\n", "\tst2 {{ v16.d, v17.d }}[0], [{0}]\n",
                inlateout(reg) dest => _, inlateout(reg) reg => _, out("v16") _,
                out("v17") _, options(preserves_flags)
            );
        }
        AARCH64_RET_D3 => {
            asm!(
                "ldp q16, q17, [{1}]\n", "\tldr q18, [{1}, #32]\n",
                "\tst3 {{ v16.d, v17.d, v18.d }}[0], [{0}]\n", inlateout(reg) dest => _,
                inlateout(reg) reg => _, out("v16") _, out("v17") _, out("v18") _,
                options(preserves_flags)
            );
        }
        AARCH64_RET_D4 => {
            asm!(
                "ldp q16, q17, [{1}]\n", "\tldp q18, q19, [{1}, #32]\n",
                "\tst4 {{ v16.d, v17.d, v18.d, v19.d }}[0], [{0}]\n", inlateout(reg) dest
                => _, inlateout(reg) reg => _, out("v16") _, out("v17") _, out("v18") _,
                out("v19") _, options(preserves_flags)
            );
        }
        _ => {
            if dest != reg {
                return memcpy(
                    dest,
                    reg,
                    (16 as core::ffi::c_int
                        * (4 as core::ffi::c_int - (h & 3 as core::ffi::c_int)))
                        as size_t,
                );
            }
        }
    }
    return dest;
}
unsafe extern "C" fn allocate_int_to_reg_or_stack(
    mut context: *mut call_context,
    mut state: *mut arg_state,
    mut stack: *mut core::ffi::c_void,
    mut size: size_t,
) -> *mut core::ffi::c_void {
    if (*state).ngrn < N_X_ARG_REG as core::ffi::c_uint {
        let fresh3 = (*state).ngrn;
        (*state).ngrn = (*state).ngrn.wrapping_add(1);
        return (&raw mut (*context).x as *mut UINT64).offset(fresh3 as isize) as *mut UINT64
            as *mut core::ffi::c_void;
    }
    (*state).ngrn = N_X_ARG_REG as core::ffi::c_uint;
    return allocate_to_stack(state, stack, size, size);
}
unsafe extern "C" fn allocate_int128_to_reg_or_stack(
    mut context: *mut call_context,
    mut state: *mut arg_state,
    mut stack: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut ngrn: core::ffi::c_uint = (*state).ngrn;
    let mut ret: *mut core::ffi::c_void = core::ptr::null_mut::<core::ffi::c_void>();
    if ngrn < N_X_ARG_REG as core::ffi::c_uint {
        ret = (&raw mut (*context).x as *mut UINT64).offset(ngrn as isize) as *mut UINT64
            as *mut core::ffi::c_void;
        ngrn = ngrn.wrapping_add(2 as core::ffi::c_uint);
    } else {
        ret = allocate_to_stack(state, stack, 16 as size_t, 16 as size_t);
        ngrn = N_X_ARG_REG as core::ffi::c_uint;
    }
    (*state).ngrn = ngrn;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep(mut cif: *mut ffi_cif) -> ffi_status {
    let mut rtype: *mut ffi_type = (*cif).rtype;
    let mut bytes: size_t = (*cif).bytes as size_t;
    let mut flags: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    let mut n: core::ffi::c_int = 0;
    match (*rtype).type_0 as core::ffi::c_int {
        FFI_TYPE_VOID => {
            flags = AARCH64_RET_VOID;
        }
        FFI_TYPE_UINT8 => {
            flags = AARCH64_RET_UINT8;
        }
        FFI_TYPE_UINT16 => {
            flags = AARCH64_RET_UINT16;
        }
        FFI_TYPE_UINT32 => {
            flags = AARCH64_RET_UINT32;
        }
        FFI_TYPE_SINT8 => {
            flags = AARCH64_RET_SINT8;
        }
        FFI_TYPE_SINT16 => {
            flags = AARCH64_RET_SINT16;
        }
        FFI_TYPE_INT | FFI_TYPE_SINT32 => {
            flags = AARCH64_RET_SINT32;
        }
        FFI_TYPE_SINT64 | FFI_TYPE_UINT64 => {
            flags = AARCH64_RET_INT64;
        }
        FFI_TYPE_POINTER => {
            flags = if core::mem::size_of::<*mut core::ffi::c_void>() as usize == 4 as usize {
                AARCH64_RET_UINT32
            } else {
                AARCH64_RET_INT64
            };
        }
        FFI_TYPE_UINT128 | FFI_TYPE_SINT128 => {
            flags = AARCH64_RET_INT128;
        }
        FFI_TYPE_FLOAT | FFI_TYPE_DOUBLE | FFI_TYPE_LONGDOUBLE | FFI_TYPE_STRUCT
        | FFI_TYPE_COMPLEX => {
            flags = is_vfp_type(rtype);
            if flags == 0 as core::ffi::c_int {
                let mut s: size_t = (*rtype).size;
                if s > 16 as size_t {
                    flags = AARCH64_RET_VOID | AARCH64_RET_IN_MEM;
                    bytes = bytes.wrapping_add(8 as size_t);
                } else if s == 16 as size_t {
                    flags = AARCH64_RET_INT128;
                } else if s == 8 as size_t {
                    flags = AARCH64_RET_INT64;
                } else {
                    flags = AARCH64_RET_INT128 | AARCH64_RET_NEED_COPY;
                }
            }
        }
        _ => {
            abort();
        }
    }
    i = 0 as core::ffi::c_int;
    n = (*cif).nargs as core::ffi::c_int;
    while i < n {
        if is_vfp_type(*(*cif).arg_types.offset(i as isize)) != 0 {
            flags |= AARCH64_FLAG_ARG_V;
            break;
        } else {
            i += 1;
        }
    }
    (*cif).bytes = (bytes.wrapping_sub(1 as size_t)
        | (16 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
        .wrapping_add(1 as size_t) as core::ffi::c_uint;
    (*cif).flags = flags as core::ffi::c_uint;
    (*cif).aarch64_nfixedargs = 0 as core::ffi::c_uint;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep_var(
    mut cif: *mut ffi_cif,
    mut nfixedargs: core::ffi::c_uint,
    mut ntotalargs: core::ffi::c_uint,
) -> ffi_status {
    let mut status: ffi_status = ffi_prep_cif_machdep(cif);
    (*cif).aarch64_nfixedargs = nfixedargs;
    return status;
}
unsafe extern "C" fn ffi_call_int(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut orig_rvalue: *mut core::ffi::c_void,
    mut avalue: *mut *mut core::ffi::c_void,
    mut closure: *mut core::ffi::c_void,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut context: *mut call_context = core::ptr::null_mut::<call_context>();
    let mut stack: *mut core::ffi::c_void = core::ptr::null_mut::<core::ffi::c_void>();
    let mut frame: *mut core::ffi::c_void = core::ptr::null_mut::<core::ffi::c_void>();
    let mut rvalue: *mut core::ffi::c_void = core::ptr::null_mut::<core::ffi::c_void>();
    let mut state: arg_state = arg_state {
        ngrn: 0,
        nsrn: 0,
        nsaa: 0,
        next_struct_area: 0,
        allocating_variadic: 0,
    };
    let mut stack_bytes: size_t = 0;
    let mut rtype_size: size_t = 0;
    let mut rsize: size_t = 0;
    let mut i: core::ffi::c_int = 0;
    let mut nargs: core::ffi::c_int = 0;
    let mut flags: core::ffi::c_int = 0;
    let mut isvariadic: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut rtype: *mut ffi_type = core::ptr::null_mut::<ffi_type>();
    flags = (*cif).flags as core::ffi::c_int;
    rtype = (*cif).rtype;
    rtype_size = (*rtype).size;
    stack_bytes = (*cif).bytes as size_t;
    if flags & AARCH64_FLAG_VARARG != 0 {
        isvariadic = 1 as core::ffi::c_int;
        flags &= !AARCH64_FLAG_VARARG;
    }
    rsize = 0 as size_t;
    if flags & AARCH64_RET_IN_MEM != 0 {
        if orig_rvalue.is_null() {
            rsize = rtype_size;
        }
    } else if orig_rvalue.is_null() {
        flags &= AARCH64_FLAG_ARG_V;
    } else if flags & AARCH64_RET_NEED_COPY != 0 {
        rsize = 16 as size_t;
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (core::mem::size_of::<call_context>() as usize)
            .wrapping_add(stack_bytes as usize)
            .wrapping_add(40 as usize)
            .wrapping_add(rsize as usize) as usize,
    ));
    context = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut call_context;
    stack = context.offset(1 as core::ffi::c_int as isize) as *mut core::ffi::c_void;
    frame = (stack as uintptr_t).wrapping_add(stack_bytes as uintptr_t) as *mut core::ffi::c_void;
    rvalue = if rsize != 0 {
        (frame as uintptr_t).wrapping_add(40 as core::ffi::c_int as uintptr_t)
            as *mut core::ffi::c_void
    } else {
        orig_rvalue
    };
    arg_init(&raw mut state, stack_bytes);
    i = 0 as core::ffi::c_int;
    nargs = (*cif).nargs as core::ffi::c_int;
    while i < nargs {
        let mut ty: *mut ffi_type = *(*cif).arg_types.offset(i as isize);
        let mut s: size_t = (*ty).size;
        let mut a: *mut core::ffi::c_void = *avalue.offset(i as isize);
        let mut h: core::ffi::c_int = 0;
        let mut t: core::ffi::c_int = 0;
        let mut dest: *mut core::ffi::c_void = core::ptr::null_mut::<core::ffi::c_void>();
        t = (*ty).type_0 as core::ffi::c_int;
        let mut current_block_57: u64;
        match t {
            FFI_TYPE_VOID => {
                current_block_57 = 6243635450180130569;
            }
            FFI_TYPE_INT | FFI_TYPE_UINT8 | FFI_TYPE_SINT8 | FFI_TYPE_UINT16 | FFI_TYPE_SINT16
            | FFI_TYPE_UINT32 | FFI_TYPE_SINT32 | FFI_TYPE_UINT64 | FFI_TYPE_SINT64
            | FFI_TYPE_POINTER => {
                current_block_57 = 12308861024312953424;
            }
            FFI_TYPE_UINT128 | FFI_TYPE_SINT128 => {
                dest = allocate_int128_to_reg_or_stack(context, &raw mut state, stack);
                memcpy(dest, a, 16 as size_t);
                current_block_57 = 6243635450180130569;
            }
            FFI_TYPE_FLOAT | FFI_TYPE_DOUBLE | FFI_TYPE_LONGDOUBLE | FFI_TYPE_STRUCT
            | FFI_TYPE_COMPLEX => {
                h = is_vfp_type(ty);
                if h != 0 {
                    let mut elems: core::ffi::c_int =
                        4 as core::ffi::c_int - (h & 3 as core::ffi::c_int);
                    if (*cif).abi as core::ffi::c_uint
                        == FFI_WIN64 as core::ffi::c_int as core::ffi::c_uint
                        && isvariadic != 0
                    {
                        if state.ngrn.wrapping_add(elems as core::ffi::c_uint)
                            <= N_X_ARG_REG as core::ffi::c_uint
                        {
                            dest = (&raw mut (*context).x as *mut UINT64)
                                .offset(state.ngrn as isize)
                                as *mut UINT64
                                as *mut core::ffi::c_void;
                            state.ngrn = state.ngrn.wrapping_add(elems as core::ffi::c_uint);
                            extend_hfa_type(dest, a, h);
                            current_block_57 = 6243635450180130569;
                        } else {
                            state.nsrn = N_X_ARG_REG as core::ffi::c_uint;
                            dest = allocate_to_stack(
                                &raw mut state,
                                stack,
                                (*ty).alignment as size_t,
                                s,
                            );
                            current_block_57 = 16203797167131938757;
                        }
                    } else if state.nsrn.wrapping_add(elems as core::ffi::c_uint)
                        <= N_V_ARG_REG as core::ffi::c_uint
                    {
                        dest = (&raw mut (*context).v as *mut _v).offset(state.nsrn as isize)
                            as *mut _v as *mut core::ffi::c_void;
                        state.nsrn = state.nsrn.wrapping_add(elems as core::ffi::c_uint);
                        extend_hfa_type(dest, a, h);
                        current_block_57 = 6243635450180130569;
                    } else {
                        state.nsrn = N_V_ARG_REG as core::ffi::c_uint;
                        dest =
                            allocate_to_stack(&raw mut state, stack, (*ty).alignment as size_t, s);
                        current_block_57 = 16203797167131938757;
                    }
                } else if s > 16 as size_t {
                    dest = allocate_and_copy_struct_to_stack(
                        &raw mut state,
                        stack,
                        (*ty).alignment as size_t,
                        s,
                        *avalue.offset(i as isize),
                    );
                    a = &raw mut dest as *mut core::ffi::c_void;
                    t = FFI_TYPE_POINTER;
                    s = core::mem::size_of::<*mut core::ffi::c_void>() as usize as size_t;
                    current_block_57 = 12308861024312953424;
                } else {
                    let mut n: size_t = s.wrapping_add(7 as size_t).wrapping_div(8 as size_t);
                    if (state.ngrn as size_t).wrapping_add(n) <= N_X_ARG_REG as size_t {
                        dest = (&raw mut (*context).x as *mut UINT64).offset(state.ngrn as isize)
                            as *mut UINT64
                            as *mut core::ffi::c_void;
                        state.ngrn = state.ngrn.wrapping_add(n as core::ffi::c_uint);
                    } else {
                        state.ngrn = N_X_ARG_REG as core::ffi::c_uint;
                        dest =
                            allocate_to_stack(&raw mut state, stack, (*ty).alignment as size_t, s);
                    }
                    current_block_57 = 16203797167131938757;
                }
                match current_block_57 {
                    12308861024312953424 => {}
                    6243635450180130569 => {}
                    _ => {
                        memcpy(dest, a, s);
                        current_block_57 = 6243635450180130569;
                    }
                }
            }
            _ => {
                abort();
            }
        }
        match current_block_57 {
            12308861024312953424 => {
                let mut ext: ffi_arg = extend_integer_type(a, t);
                if state.ngrn < N_X_ARG_REG as core::ffi::c_uint {
                    let fresh2 = state.ngrn;
                    state.ngrn = state.ngrn.wrapping_add(1);
                    (*context).x[fresh2 as usize] = ext as UINT64;
                } else {
                    let mut d: *mut core::ffi::c_void =
                        allocate_to_stack(&raw mut state, stack, (*ty).alignment as size_t, s);
                    state.ngrn = N_X_ARG_REG as core::ffi::c_uint;
                    memcpy(d, a, s);
                }
            }
            _ => {}
        }
        if (i + 1 as core::ffi::c_int) as core::ffi::c_uint == (*cif).aarch64_nfixedargs {
            state.ngrn = N_X_ARG_REG as core::ffi::c_uint;
            state.nsrn = N_V_ARG_REG as core::ffi::c_uint;
            state.allocating_variadic = 1 as core::ffi::c_uint;
        }
        i += 1;
    }
    ffi_call_SYSV(context, frame, fn_0, rvalue, flags, closure);
    if flags & AARCH64_RET_NEED_COPY != 0 {
        memcpy(orig_rvalue, rvalue, rtype_size);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ffi_call(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut core::ffi::c_void,
    mut avalue: *mut *mut core::ffi::c_void,
) {
    ffi_call_int(cif, fn_0, rvalue, avalue, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_closure_loc(
    mut closure: *mut ffi_closure,
    mut cif: *mut ffi_cif,
    mut fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> (),
    >,
    mut user_data: *mut core::ffi::c_void,
    mut codeloc: *mut core::ffi::c_void,
) -> ffi_status {
    if (*cif).abi as core::ffi::c_uint != FFI_SYSV as core::ffi::c_int as core::ffi::c_uint
        && (*cif).abi as core::ffi::c_uint
            != FFI_WIN64 as core::ffi::c_int as core::ffi::c_uint
    {
        return FFI_BAD_ABI;
    }
    let mut start: Option<unsafe extern "C" fn() -> ()> = None;
    if (*cif).flags & AARCH64_FLAG_ARG_V as core::ffi::c_uint != 0 {
        start = Some(ffi_closure_SYSV_V as unsafe extern "C" fn() -> ())
            as Option<unsafe extern "C" fn() -> ()>;
    } else {
        start = Some(ffi_closure_SYSV as unsafe extern "C" fn() -> ())
            as Option<unsafe extern "C" fn() -> ()>;
    }
    let mut config: *mut *mut core::ffi::c_void = (codeloc as *mut uint8_t)
        .offset(-(PAGE_MAX_SIZE as isize))
        as *mut *mut core::ffi::c_void;
    let ref mut fresh0 = *config.offset(0 as core::ffi::c_int as isize);
    *fresh0 = closure as *mut core::ffi::c_void;
    let ref mut fresh1 = *config.offset(1 as core::ffi::c_int as isize);
    *fresh1 = core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        *mut core::ffi::c_void,
    >(start);
    (*closure).cif = cif;
    (*closure).fun = fun;
    (*closure).user_data = user_data;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_SYSV_inner(
    mut cif: *mut ffi_cif,
    mut fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> (),
    >,
    mut user_data: *mut core::ffi::c_void,
    mut context: *mut call_context,
    mut stack: *mut core::ffi::c_void,
    mut rvalue: *mut core::ffi::c_void,
    mut struct_rvalue: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    alloca_allocations.push(::std::vec::from_elem(
        0,
        ((*cif).nargs as usize)
            .wrapping_mul(core::mem::size_of::<*mut core::ffi::c_void>() as usize)
            as usize,
    ));
    let mut avalue: *mut *mut core::ffi::c_void =
        alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut core::ffi::c_void;
    let mut i: core::ffi::c_int = 0;
    let mut h: core::ffi::c_int = 0;
    let mut nargs: core::ffi::c_int = 0;
    let mut flags: core::ffi::c_int = 0;
    let mut isvariadic: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut state: arg_state = arg_state {
        ngrn: 0,
        nsrn: 0,
        nsaa: 0,
        next_struct_area: 0,
        allocating_variadic: 0,
    };
    arg_init(&raw mut state, (*cif).bytes as size_t);
    flags = (*cif).flags as core::ffi::c_int;
    if flags & AARCH64_FLAG_VARARG != 0 {
        isvariadic = 1 as core::ffi::c_int;
        flags &= !AARCH64_FLAG_VARARG;
    }
    i = 0 as core::ffi::c_int;
    nargs = (*cif).nargs as core::ffi::c_int;
    while i < nargs {
        let mut ty: *mut ffi_type = *(*cif).arg_types.offset(i as isize);
        let mut t: core::ffi::c_int = (*ty).type_0 as core::ffi::c_int;
        let mut n: size_t = 0;
        let mut s: size_t = (*ty).size;
        match t {
            FFI_TYPE_VOID => {}
            FFI_TYPE_INT | FFI_TYPE_UINT8 | FFI_TYPE_SINT8 | FFI_TYPE_UINT16 | FFI_TYPE_SINT16
            | FFI_TYPE_UINT32 | FFI_TYPE_SINT32 | FFI_TYPE_UINT64 | FFI_TYPE_SINT64
            | FFI_TYPE_POINTER => {
                let ref mut fresh4 = *avalue.offset(i as isize);
                *fresh4 = allocate_int_to_reg_or_stack(context, &raw mut state, stack, s);
            }
            FFI_TYPE_UINT128 | FFI_TYPE_SINT128 => {
                let ref mut fresh5 = *avalue.offset(i as isize);
                *fresh5 = allocate_int128_to_reg_or_stack(context, &raw mut state, stack);
            }
            FFI_TYPE_FLOAT | FFI_TYPE_DOUBLE | FFI_TYPE_LONGDOUBLE | FFI_TYPE_STRUCT
            | FFI_TYPE_COMPLEX => {
                h = is_vfp_type(ty);
                if h != 0 {
                    n = (4 as core::ffi::c_int - (h & 3 as core::ffi::c_int)) as size_t;
                    if (*cif).abi as core::ffi::c_uint
                        == FFI_WIN64 as core::ffi::c_int as core::ffi::c_uint
                        && isvariadic != 0
                    {
                        if (state.ngrn as size_t).wrapping_add(n) <= N_X_ARG_REG as size_t {
                            let mut reg: *mut core::ffi::c_void =
                                (&raw mut (*context).x as *mut UINT64).offset(state.ngrn as isize)
                                    as *mut UINT64
                                    as *mut core::ffi::c_void;
                            state.ngrn = state.ngrn.wrapping_add(n as core::ffi::c_uint);
                            let ref mut fresh6 = *avalue.offset(i as isize);
                            *fresh6 = compress_hfa_type(reg, reg, h);
                        } else {
                            state.ngrn = N_X_ARG_REG as core::ffi::c_uint;
                            state.nsrn = N_V_ARG_REG as core::ffi::c_uint;
                            let ref mut fresh7 = *avalue.offset(i as isize);
                            *fresh7 = allocate_to_stack(
                                &raw mut state,
                                stack,
                                (*ty).alignment as size_t,
                                s,
                            );
                        }
                    } else if (state.nsrn as size_t).wrapping_add(n) <= N_V_ARG_REG as size_t {
                        let mut reg_0: *mut core::ffi::c_void =
                            (&raw mut (*context).v as *mut _v).offset(state.nsrn as isize)
                                as *mut _v as *mut core::ffi::c_void;
                        state.nsrn = state.nsrn.wrapping_add(n as core::ffi::c_uint);
                        let ref mut fresh8 = *avalue.offset(i as isize);
                        *fresh8 = compress_hfa_type(reg_0, reg_0, h);
                    } else {
                        state.nsrn = N_V_ARG_REG as core::ffi::c_uint;
                        let ref mut fresh9 = *avalue.offset(i as isize);
                        *fresh9 =
                            allocate_to_stack(&raw mut state, stack, (*ty).alignment as size_t, s);
                    }
                } else if s > 16 as size_t {
                    let ref mut fresh10 = *avalue.offset(i as isize);
                    *fresh10 = *(allocate_int_to_reg_or_stack(
                        context,
                        &raw mut state,
                        stack,
                        core::mem::size_of::<*mut core::ffi::c_void>() as size_t,
                    ) as *mut *mut core::ffi::c_void);
                } else {
                    n = s.wrapping_add(7 as size_t).wrapping_div(8 as size_t);
                    if (state.ngrn as size_t).wrapping_add(n) <= N_X_ARG_REG as size_t {
                        let ref mut fresh11 = *avalue.offset(i as isize);
                        *fresh11 = (&raw mut (*context).x as *mut UINT64)
                            .offset(state.ngrn as isize)
                            as *mut UINT64
                            as *mut core::ffi::c_void;
                        state.ngrn = state.ngrn.wrapping_add(n as core::ffi::c_uint);
                    } else {
                        state.ngrn = N_X_ARG_REG as core::ffi::c_uint;
                        let ref mut fresh12 = *avalue.offset(i as isize);
                        *fresh12 =
                            allocate_to_stack(&raw mut state, stack, (*ty).alignment as size_t, s);
                    }
                }
            }
            _ => {
                abort();
            }
        }
        if (i + 1 as core::ffi::c_int) as core::ffi::c_uint == (*cif).aarch64_nfixedargs {
            state.ngrn = N_X_ARG_REG as core::ffi::c_uint;
            state.nsrn = N_V_ARG_REG as core::ffi::c_uint;
            state.allocating_variadic = 1 as core::ffi::c_uint;
        }
        i += 1;
    }
    if flags & AARCH64_RET_IN_MEM != 0 {
        rvalue = struct_rvalue;
    }
    fun.expect("non-null function pointer")(cif, rvalue, avalue, user_data);
    return flags;
}
