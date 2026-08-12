extern "C" {
    fn __riscv_flush_icache(
        start: *mut core::ffi::c_void,
        end: *mut core::ffi::c_void,
        flags: usize,
    ) -> core::ffi::c_long;
    static mut ffi_type_pointer: ffi_type;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ffi_tramp_is_present(closure: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn ffi_tramp_set_parms(
        tramp: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        code: *mut ::core::ffi::c_void,
    );
    fn ffi_call_asm(
        stack: *mut ::core::ffi::c_void,
        regs: *mut call_context,
        fn_0: Option<unsafe extern "C" fn() -> ()>,
        closure: *mut ::core::ffi::c_void,
        stack_bytes: size_t,
    );
    fn ffi_closure_asm();
    fn ffi_go_closure_asm();
}
pub type ffi_arg = ::core::ffi::c_ulong;
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 1;
pub const FFI_LAST_ABI: ffi_abi = 5;
pub const FFI_UNUSED_3: ffi_abi = 4;
pub const FFI_UNUSED_2: ffi_abi = 3;
pub const FFI_UNUSED_1: ffi_abi = 2;
pub const FFI_SYSV: ffi_abi = 1;
pub const FFI_FIRST_ABI: ffi_abi = 0;
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct max_align_t {
    pub __ll: ::core::ffi::c_longlong,
    pub __ld: [u8; 16],
}
pub type size_t = ::core::ffi::c_ulong;
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
    pub riscv_nfixedargs: ::core::ffi::c_uint,
    pub riscv_unused: ::core::ffi::c_uint,
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
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
pub type uintptr_t = ::core::ffi::c_ulong;
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
pub struct call_builder {
    pub aregs: *mut call_context,
    pub used_integer: ::core::ffi::c_int,
    pub used_float: ::core::ffi::c_int,
    pub used_stack: *mut size_t,
    pub struct_stack: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct call_context {
    pub fa: [float_reg; 8],
    pub a: [size_t; 8],
    pub frame: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union float_reg {
    pub i: uint64_t,
    pub d: ::core::ffi::c_double,
}
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint8_t = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub i: uint32_t,
    pub f: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct float_struct_info {
    pub as_elements: ::core::ffi::c_char,
    pub type1: ::core::ffi::c_char,
    pub offset2: ::core::ffi::c_char,
    pub type2: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: uint32_t,
    pub f: ::core::ffi::c_float,
}
pub type int64_t = ::core::ffi::c_long;
pub type int32_t = ::core::ffi::c_int;
pub type int16_t = ::core::ffi::c_short;
pub type int8_t = ::core::ffi::c_schar;
pub const FFI_TYPE_VOID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const FFI_TYPE_STRUCT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FFI_TYPE_POINTER: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const FFI_TRAMPOLINE_SIZE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const RISCV_TRAMP_MAP_SHIFT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const RISCV_TRAMP_MAP_SIZE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << RISCV_TRAMP_MAP_SHIFT;
pub const RISCV_TRAMP_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NARGREG: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const STKALIGN: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MAXCOPYARG: usize =
    (2 as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize);
unsafe extern "C" fn flatten_struct(
    mut in_0: *mut ffi_type,
    mut out: *mut *mut ffi_type,
    mut out_end: *mut *mut ffi_type,
) -> *mut *mut ffi_type {
    let mut i: ::core::ffi::c_int = 0;
    if out == out_end {
        return out;
    }
    if (*in_0).type_0 as ::core::ffi::c_int != FFI_TYPE_STRUCT {
        let fresh7 = out;
        out = out.offset(1);
        *fresh7 = in_0;
    } else {
        i = 0 as ::core::ffi::c_int;
        while !(*(*in_0).elements.offset(i as isize)).is_null() {
            out = flatten_struct(
                *(*in_0).elements.offset(i as isize) as *mut ffi_type,
                out,
                out_end,
            );
            i += 1;
        }
    }
    return out;
}
unsafe extern "C" fn struct_passed_as_elements(
    mut cb: *mut call_builder,
    mut top: *mut ffi_type,
) -> float_struct_info {
    let mut ret: float_struct_info = float_struct_info {
        as_elements: 0 as ::core::ffi::c_char,
        type1: 0 as ::core::ffi::c_char,
        offset2: 0 as ::core::ffi::c_char,
        type2: 0 as ::core::ffi::c_char,
    };
    let mut fields: [*mut ffi_type; 3] = [::core::ptr::null_mut::<ffi_type>(); 3];
    let mut num_floats: ::core::ffi::c_int = 0;
    let mut num_ints: ::core::ffi::c_int = 0;
    let mut num_fields: ::core::ffi::c_int = flatten_struct(
        top,
        &raw mut fields as *mut *mut ffi_type,
        (&raw mut fields as *mut *mut ffi_type).offset(3 as ::core::ffi::c_int as isize),
    )
    .offset_from(&raw mut fields as *mut *mut ffi_type)
        as ::core::ffi::c_long as ::core::ffi::c_int;
    if num_fields == 1 as ::core::ffi::c_int {
        if (*fields[0 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
            >= FFI_TYPE_FLOAT
            && (*fields[0 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
                <= FFI_TYPE_DOUBLE
        {
            ret.as_elements = 1 as ::core::ffi::c_char;
            ret.type1 = (*fields[0 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_char;
        }
    } else if num_fields == 2 as ::core::ffi::c_int {
        num_floats = ((*fields[0 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
            >= FFI_TYPE_FLOAT
            && (*fields[0 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
                <= FFI_TYPE_DOUBLE) as ::core::ffi::c_int
            + ((*fields[1 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
                >= FFI_TYPE_FLOAT
                && (*fields[1 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
                    <= FFI_TYPE_DOUBLE) as ::core::ffi::c_int;
        num_ints = ((*fields[0 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
            >= FFI_TYPE_UINT8
            && (*fields[0 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
                <= FFI_TYPE_SINT64) as ::core::ffi::c_int
            + ((*fields[1 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
                >= FFI_TYPE_UINT8
                && (*fields[1 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
                    <= FFI_TYPE_SINT64) as ::core::ffi::c_int;
        if num_floats == 0 as ::core::ffi::c_int || num_floats + num_ints != 2 as ::core::ffi::c_int
        {
            return ret;
        }
        if (*cb).used_float + num_floats > NARGREG
            || (*cb).used_integer + (2 as ::core::ffi::c_int - num_floats) > NARGREG
        {
            return ret;
        }
        if !((*fields[0 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
            >= FFI_TYPE_FLOAT
            && (*fields[0 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
                <= FFI_TYPE_DOUBLE)
            && !((*fields[1 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
                >= FFI_TYPE_FLOAT
                && (*fields[1 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_int
                    <= FFI_TYPE_DOUBLE)
        {
            return ret;
        }
        ret.type1 = (*fields[0 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_char;
        ret.type2 = (*fields[1 as ::core::ffi::c_int as usize]).type_0 as ::core::ffi::c_char;
        ret.offset2 = ((*fields[0 as ::core::ffi::c_int as usize])
            .size
            .wrapping_sub(1 as ::core::ffi::c_ulong)
            | ((*fields[1 as ::core::ffi::c_int as usize]).alignment as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
            .wrapping_add(1 as ::core::ffi::c_ulong) as ::core::ffi::c_char;
        ret.as_elements = 1 as ::core::ffi::c_char;
    }
    return ret;
}
unsafe extern "C" fn marshal_float(mut cb: *mut call_builder, mut data: *mut ::core::ffi::c_void) {
    let mut value: C2RustUnnamed_1 = C2RustUnnamed_1 { i: 0 };
    value.f = *(data as *mut ::core::ffi::c_float);
    let fresh14 = (*cb).used_float;
    (*cb).used_float = (*cb).used_float + 1;
    (*(*cb).aregs).fa[fresh14 as usize].i =
        (0xffffffff00000000 as ::core::ffi::c_ulong | value.i as ::core::ffi::c_ulong) as uint64_t;
}
unsafe extern "C" fn unmarshal_float(
    mut cb: *mut call_builder,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut value: C2RustUnnamed_0 = C2RustUnnamed_0 { i: 0 };
    let fresh6 = (*cb).used_float;
    (*cb).used_float = (*cb).used_float + 1;
    value.i = (*(*cb).aregs).fa[fresh6 as usize].i as uint32_t;
    *(data as *mut ::core::ffi::c_float) = value.f;
}
unsafe extern "C" fn marshal_atom(
    mut cb: *mut call_builder,
    mut type_0: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut value: size_t = 0 as size_t;
    match type_0 {
        FFI_TYPE_UINT8 => {
            value = *(data as *mut uint8_t) as size_t;
        }
        FFI_TYPE_SINT8 => {
            value = *(data as *mut int8_t) as size_t;
        }
        FFI_TYPE_UINT16 => {
            value = *(data as *mut uint16_t) as size_t;
        }
        FFI_TYPE_SINT16 => {
            value = *(data as *mut int16_t) as size_t;
        }
        FFI_TYPE_UINT32 => {
            value = *(data as *mut int32_t) as size_t;
        }
        FFI_TYPE_SINT32 => {
            value = *(data as *mut int32_t) as size_t;
        }
        FFI_TYPE_UINT64 => {
            value = *(data as *mut uint64_t) as size_t;
        }
        FFI_TYPE_SINT64 => {
            value = *(data as *mut int64_t) as size_t;
        }
        FFI_TYPE_POINTER => {
            value = *(data as *mut size_t);
        }
        FFI_TYPE_FLOAT => {
            marshal_float(cb, data);
            return;
        }
        FFI_TYPE_DOUBLE => {
            let fresh8 = (*cb).used_float;
            (*cb).used_float = (*cb).used_float + 1;
            (*(*cb).aregs).fa[fresh8 as usize].d = *(data as *mut ::core::ffi::c_double);
            return;
        }
        _ => {}
    }
    if (*cb).used_integer == NARGREG {
        let fresh12 = (*cb).used_stack;
        (*cb).used_stack = (*cb).used_stack.offset(1);
        *fresh12 = value;
    } else {
        let fresh13 = (*cb).used_integer;
        (*cb).used_integer = (*cb).used_integer + 1;
        (*(*cb).aregs).a[fresh13 as usize] = value;
    };
}
unsafe extern "C" fn unmarshal_atom(
    mut cb: *mut call_builder,
    mut type_0: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut value: size_t = 0;
    match type_0 {
        FFI_TYPE_FLOAT => {
            unmarshal_float(cb, data);
            return;
        }
        FFI_TYPE_DOUBLE => {
            let fresh2 = (*cb).used_float;
            (*cb).used_float = (*cb).used_float + 1;
            *(data as *mut ::core::ffi::c_double) = (*(*cb).aregs).fa[fresh2 as usize].d;
            return;
        }
        _ => {}
    }
    if (*cb).used_integer == NARGREG {
        let fresh4 = (*cb).used_stack;
        (*cb).used_stack = (*cb).used_stack.offset(1);
        value = *fresh4;
    } else {
        let fresh5 = (*cb).used_integer;
        (*cb).used_integer = (*cb).used_integer + 1;
        value = (*(*cb).aregs).a[fresh5 as usize];
    }
    match type_0 {
        FFI_TYPE_UINT8 => {
            *(data as *mut uint8_t) = value as uint8_t;
        }
        FFI_TYPE_SINT8 => {
            *(data as *mut uint8_t) = value as uint8_t;
        }
        FFI_TYPE_UINT16 => {
            *(data as *mut uint16_t) = value as uint16_t;
        }
        FFI_TYPE_SINT16 => {
            *(data as *mut uint16_t) = value as uint16_t;
        }
        FFI_TYPE_UINT32 => {
            *(data as *mut uint32_t) = value as uint32_t;
        }
        FFI_TYPE_SINT32 => {
            *(data as *mut uint32_t) = value as uint32_t;
        }
        FFI_TYPE_UINT64 => {
            *(data as *mut uint64_t) = value as uint64_t;
        }
        FFI_TYPE_SINT64 => {
            *(data as *mut uint64_t) = value as uint64_t;
        }
        FFI_TYPE_POINTER => {
            *(data as *mut size_t) = value;
        }
        _ => {}
    };
}
unsafe extern "C" fn marshal(
    mut cb: *mut call_builder,
    mut type_0: *mut ffi_type,
    mut var: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut realign: [size_t; 2] = [0; 2];
    if var == 0 && (*type_0).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT {
        let mut fsi: float_struct_info = struct_passed_as_elements(cb, type_0);
        if fsi.as_elements != 0 {
            marshal_atom(cb, fsi.type1 as ::core::ffi::c_int, data);
            if fsi.offset2 != 0 {
                marshal_atom(
                    cb,
                    fsi.type2 as ::core::ffi::c_int,
                    (data as *mut ::core::ffi::c_char)
                        .offset(fsi.offset2 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                );
            }
            return;
        }
    }
    if var == 0
        && (*cb).used_float < NARGREG
        && ((*type_0).type_0 as ::core::ffi::c_int >= FFI_TYPE_FLOAT
            && (*type_0).type_0 as ::core::ffi::c_int <= FFI_TYPE_DOUBLE)
    {
        marshal_atom(cb, (*type_0).type_0 as ::core::ffi::c_int, data);
        return;
    }
    if (*type_0).size > (2 as ::core::ffi::c_int * __SIZEOF_POINTER__) as ::core::ffi::c_ulong {
        data = memcpy((*cb).struct_stack, data, (*type_0).size);
        (*cb).struct_stack = ((((*cb).struct_stack as *mut ::core::ffi::c_char)
            .offset((*type_0).size as isize)
            as ::core::ffi::c_ulong)
            .wrapping_sub(1 as ::core::ffi::c_ulong)
            | (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
            .wrapping_add(1 as ::core::ffi::c_ulong) as *mut size_t
            as *mut ::core::ffi::c_void;
        marshal_atom(
            cb,
            FFI_TYPE_POINTER,
            &raw mut data as *mut ::core::ffi::c_void,
        );
    } else if (*type_0).type_0 as ::core::ffi::c_int >= FFI_TYPE_UINT8
        && (*type_0).type_0 as ::core::ffi::c_int <= FFI_TYPE_SINT64
        || (*type_0).type_0 as ::core::ffi::c_int == FFI_TYPE_POINTER
    {
        marshal_atom(cb, (*type_0).type_0 as ::core::ffi::c_int, data);
    } else {
        if (*type_0).alignment as ::core::ffi::c_int > __SIZEOF_POINTER__ {
            if var != 0 {
                (*cb).used_integer = (((*cb).used_integer as ::core::ffi::c_ulong)
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                    | (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong)
                    as ::core::ffi::c_int;
            }
            (*cb).used_stack =
                (((*cb).used_stack as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong)
                    | (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong) as *mut size_t;
        }
        memcpy(
            &raw mut realign as *mut size_t as *mut ::core::ffi::c_void,
            data,
            (*type_0).size,
        );
        if (*type_0).size > 0 as ::core::ffi::c_ulong {
            marshal_atom(
                cb,
                FFI_TYPE_POINTER,
                &raw mut realign as *mut size_t as *mut ::core::ffi::c_void,
            );
        }
        if (*type_0).size > __SIZEOF_POINTER__ as ::core::ffi::c_ulong {
            marshal_atom(
                cb,
                FFI_TYPE_POINTER,
                (&raw mut realign as *mut size_t).offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
            );
        }
    };
}
unsafe extern "C" fn unmarshal(
    mut cb: *mut call_builder,
    mut type_0: *mut ffi_type,
    mut var: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut realign: [size_t; 2] = [0; 2];
    let mut pointer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if var == 0 && (*type_0).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT {
        let mut fsi: float_struct_info = struct_passed_as_elements(cb, type_0);
        if fsi.as_elements != 0 {
            unmarshal_atom(cb, fsi.type1 as ::core::ffi::c_int, data);
            if fsi.offset2 != 0 {
                unmarshal_atom(
                    cb,
                    fsi.type2 as ::core::ffi::c_int,
                    (data as *mut ::core::ffi::c_char)
                        .offset(fsi.offset2 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                );
            }
            return data;
        }
    }
    if var == 0
        && (*cb).used_float < NARGREG
        && ((*type_0).type_0 as ::core::ffi::c_int >= FFI_TYPE_FLOAT
            && (*type_0).type_0 as ::core::ffi::c_int <= FFI_TYPE_DOUBLE)
    {
        unmarshal_atom(cb, (*type_0).type_0 as ::core::ffi::c_int, data);
        return data;
    }
    if (*type_0).size > (2 as ::core::ffi::c_int * __SIZEOF_POINTER__) as ::core::ffi::c_ulong {
        unmarshal_atom(
            cb,
            FFI_TYPE_POINTER,
            &raw mut pointer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
        return pointer;
    } else if (*type_0).type_0 as ::core::ffi::c_int >= FFI_TYPE_UINT8
        && (*type_0).type_0 as ::core::ffi::c_int <= FFI_TYPE_SINT64
        || (*type_0).type_0 as ::core::ffi::c_int == FFI_TYPE_POINTER
    {
        unmarshal_atom(cb, (*type_0).type_0 as ::core::ffi::c_int, data);
        return data;
    } else {
        if (*type_0).alignment as ::core::ffi::c_int > __SIZEOF_POINTER__ {
            if var != 0 {
                (*cb).used_integer = (((*cb).used_integer as ::core::ffi::c_ulong)
                    .wrapping_sub(1 as ::core::ffi::c_ulong)
                    | (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong)
                    as ::core::ffi::c_int;
            }
            (*cb).used_stack =
                (((*cb).used_stack as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong)
                    | (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong) as *mut size_t;
        }
        if (*type_0).size > 0 as ::core::ffi::c_ulong {
            unmarshal_atom(
                cb,
                FFI_TYPE_POINTER,
                &raw mut realign as *mut size_t as *mut ::core::ffi::c_void,
            );
        }
        if (*type_0).size > __SIZEOF_POINTER__ as ::core::ffi::c_ulong {
            unmarshal_atom(
                cb,
                FFI_TYPE_POINTER,
                (&raw mut realign as *mut size_t).offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
            );
        }
        memcpy(
            data,
            &raw mut realign as *mut size_t as *const ::core::ffi::c_void,
            (*type_0).size,
        );
        return data;
    };
}
unsafe extern "C" fn passed_by_ref(
    mut cb: *mut call_builder,
    mut type_0: *mut ffi_type,
    mut var: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if var == 0 && (*type_0).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT {
        let mut fsi: float_struct_info = struct_passed_as_elements(cb, type_0);
        if fsi.as_elements != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    return ((*type_0).size > (2 as ::core::ffi::c_int * __SIZEOF_POINTER__) as ::core::ffi::c_ulong)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep(mut cif: *mut ffi_cif) -> ffi_status {
    (*cif).riscv_nfixedargs = (*cif).nargs;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep_var(
    mut cif: *mut ffi_cif,
    mut nfixedargs: ::core::ffi::c_uint,
    mut ntotalargs: ::core::ffi::c_uint,
) -> ffi_status {
    (*cif).riscv_nfixedargs = nfixedargs;
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
    let mut arg_bytes: size_t = if (*cif).nargs <= 3 as ::core::ffi::c_uint {
        0 as size_t
    } else {
        (((2 as usize)
            .wrapping_mul(::core::mem::size_of::<size_t>() as usize)
            .wrapping_mul((*cif).nargs.wrapping_sub(3 as ::core::ffi::c_uint) as usize)
            as size_t)
            .wrapping_sub(1 as size_t)
            | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(1 as size_t)
    };
    let mut struct_bytes: size_t = (((*cif).bytes as size_t).wrapping_sub(1 as size_t)
        | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
        .wrapping_add(1 as size_t);
    let mut rval_bytes: size_t = 0 as size_t;
    if rvalue.is_null()
        && (*(*cif).rtype).size
            > (2 as ::core::ffi::c_int * __SIZEOF_POINTER__) as ::core::ffi::c_ulong
    {
        rval_bytes = ((*(*cif).rtype).size.wrapping_sub(1 as ::core::ffi::c_ulong)
            | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
            .wrapping_add(1 as ::core::ffi::c_ulong) as size_t;
    }
    let mut alloc_size: size_t = arg_bytes
        .wrapping_add(rval_bytes)
        .wrapping_add(struct_bytes)
        .wrapping_add(::core::mem::size_of::<call_context>() as size_t);
    let mut alloc_base: size_t = 0;
    // C2Rust models alloca with a Vec. Reserve alignment slack explicitly;
    // ffi_call_asm switches sp to this region and restores the real sp from
    // the extended call_context frame.
    alloca_allocations.push(::std::vec::from_elem(
        0,
        alloc_size.wrapping_add(STKALIGN as size_t - 1) as usize,
    ));
    alloc_base = ((alloca_allocations.last_mut().unwrap().as_mut_ptr() as size_t)
        .wrapping_add(STKALIGN as size_t - 1))
        & !(STKALIGN as size_t - 1);
    if rval_bytes != 0 {
        rvalue = alloc_base.wrapping_add(arg_bytes) as *mut ::core::ffi::c_void;
    }
    let mut cb: call_builder = call_builder {
        aregs: ::core::ptr::null_mut::<call_context>(),
        used_integer: 0,
        used_float: 0,
        used_stack: ::core::ptr::null_mut::<size_t>(),
        struct_stack: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    cb.used_integer = 0 as ::core::ffi::c_int;
    cb.used_float = cb.used_integer;
    cb.aregs = alloc_base
        .wrapping_add(arg_bytes)
        .wrapping_add(rval_bytes)
        .wrapping_add(struct_bytes) as *mut call_context;
    cb.used_stack = alloc_base as *mut ::core::ffi::c_void as *mut size_t;
    cb.struct_stack =
        alloc_base.wrapping_add(arg_bytes).wrapping_add(rval_bytes) as *mut ::core::ffi::c_void;
    let mut return_by_ref: ::core::ffi::c_int =
        passed_by_ref(&raw mut cb, (*cif).rtype, 0 as ::core::ffi::c_int);
    if return_by_ref != 0 {
        marshal(
            &raw mut cb,
            &raw mut ffi_type_pointer,
            0 as ::core::ffi::c_int,
            &raw mut rvalue as *mut ::core::ffi::c_void,
        );
    }
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < (*cif).nargs {
        marshal(
            &raw mut cb,
            *(*cif).arg_types.offset(i as isize),
            (i as ::core::ffi::c_uint >= (*cif).riscv_nfixedargs) as ::core::ffi::c_int,
            *avalue.offset(i as isize),
        );
        i += 1;
    }
    ffi_call_asm(
        alloc_base as *mut ::core::ffi::c_void,
        cb.aregs as *mut call_context,
        fn_0,
        closure,
        arg_bytes,
    );
    cb.used_integer = 0 as ::core::ffi::c_int;
    cb.used_float = cb.used_integer;
    if return_by_ref == 0 && !rvalue.is_null() {
        if (*(*cif).rtype).type_0 as ::core::ffi::c_int >= FFI_TYPE_UINT8
            && (*(*cif).rtype).type_0 as ::core::ffi::c_int <= FFI_TYPE_SINT64
            && ((*(*cif).rtype).size as usize) < ::core::mem::size_of::<ffi_arg>() as usize
        {
            match (*(*cif).rtype).type_0 as ::core::ffi::c_int {
                FFI_TYPE_SINT8 | FFI_TYPE_SINT16 | FFI_TYPE_SINT32 => {
                    unmarshal_atom(
                        &raw mut cb,
                        if ::core::mem::size_of::<ffi_arg>() as usize > 4 as usize {
                            FFI_TYPE_SINT64
                        } else {
                            FFI_TYPE_SINT32
                        },
                        rvalue,
                    );
                }
                FFI_TYPE_UINT8 | FFI_TYPE_UINT16 | FFI_TYPE_UINT32 => {
                    unmarshal_atom(
                        &raw mut cb,
                        if ::core::mem::size_of::<ffi_arg>() as usize > 4 as usize {
                            FFI_TYPE_UINT64
                        } else {
                            FFI_TYPE_UINT32
                        },
                        rvalue,
                    );
                }
                _ => {}
            }
        } else {
            unmarshal(&raw mut cb, (*cif).rtype, 0 as ::core::ffi::c_int, rvalue);
        }
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
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_closure_loc(
    closure: *mut ffi_closure,
    cif: *mut ffi_cif,
    fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ),
    >,
    user_data: *mut core::ffi::c_void,
    codeloc: *mut core::ffi::c_void,
) -> ffi_status {
    if (*cif).abi <= FFI_FIRST_ABI || (*cif).abi >= FFI_LAST_ABI {
        return FFI_BAD_ABI;
    }
    if ffi_tramp_is_present(closure.cast()) != 0 {
        ffi_tramp_set_parms(
            (*closure).c2rust_unnamed.ftramp,
            closure_fn_address(),
            closure.cast(),
        );
    } else {
        let tramp = (*closure)
            .c2rust_unnamed
            .tramp
            .as_mut_ptr()
            .cast::<uint32_t>();
        let target = ffi_closure_asm as *const () as usize as uint64_t;
        *tramp.add(0) = 0x0000_0317;
        *tramp.add(1) = 0x0103_3383;
        *tramp.add(2) = 0x0003_8067;
        *tramp.add(3) = 0x0000_0013;
        *tramp.add(4) = target as uint32_t;
        *tramp.add(5) = (target >> 32) as uint32_t;
        __riscv_flush_icache(
            codeloc,
            codeloc
                .cast::<u8>()
                .add(FFI_TRAMPOLINE_SIZE as usize)
                .cast(),
            0,
        );
    }
    (*closure).cif = cif;
    (*closure).fun = fun;
    (*closure).user_data = user_data;
    FFI_OK
}

unsafe fn closure_fn_address() -> *mut core::ffi::c_void {
    ffi_closure_asm as *const () as *mut core::ffi::c_void
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
    if (*cif).abi as ::core::ffi::c_uint
        <= FFI_FIRST_ABI as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cif).abi as ::core::ffi::c_uint
            >= FFI_LAST_ABI as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return FFI_BAD_ABI;
    }
    (*closure).tramp = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        *mut ::core::ffi::c_void,
    >(Some(ffi_go_closure_asm as unsafe extern "C" fn() -> ()));
    (*closure).cif = cif;
    (*closure).fun = fun;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_inner(
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
    mut stack: *mut size_t,
    mut aregs: *mut call_context,
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
    alloca_allocations.push(::std::vec::from_elem(
        0,
        ((*cif).nargs as usize).wrapping_mul(MAXCOPYARG) as usize,
    ));
    let mut astorage: *mut ::core::ffi::c_char =
        alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    let mut rvalue: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut cb: call_builder = call_builder {
        aregs: ::core::ptr::null_mut::<call_context>(),
        used_integer: 0,
        used_float: 0,
        used_stack: ::core::ptr::null_mut::<size_t>(),
        struct_stack: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut return_by_ref: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    cb.aregs = aregs;
    cb.used_float = 0 as ::core::ffi::c_int;
    cb.used_integer = cb.used_float;
    cb.used_stack = stack;
    return_by_ref = passed_by_ref(&raw mut cb, (*cif).rtype, 0 as ::core::ffi::c_int);
    if return_by_ref != 0 {
        unmarshal(
            &raw mut cb,
            &raw mut ffi_type_pointer,
            0 as ::core::ffi::c_int,
            &raw mut rvalue as *mut ::core::ffi::c_void,
        );
    } else {
        alloca_allocations.push(::std::vec::from_elem(0, (*(*cif).rtype).size as usize));
        rvalue = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast();
    }
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < (*cif).nargs {
        let ref mut fresh15 = *avalue.offset(i as isize);
        *fresh15 = unmarshal(
            &raw mut cb,
            *(*cif).arg_types.offset(i as isize),
            (i as ::core::ffi::c_uint >= (*cif).riscv_nfixedargs) as ::core::ffi::c_int,
            astorage.offset((i as usize).wrapping_mul(MAXCOPYARG) as isize)
                as *mut ::core::ffi::c_void,
        );
        i += 1;
    }
    fun.expect("non-null function pointer")(cif, rvalue, avalue, user_data);
    if return_by_ref == 0 && (*(*cif).rtype).type_0 as ::core::ffi::c_int != FFI_TYPE_VOID {
        cb.used_float = 0 as ::core::ffi::c_int;
        cb.used_integer = cb.used_float;
        marshal(&raw mut cb, (*cif).rtype, 0 as ::core::ffi::c_int, rvalue);
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
    *tramp_size = RISCV_TRAMP_SIZE as size_t;
    *map_size = RISCV_TRAMP_MAP_SIZE as size_t;
    return &raw mut trampoline_code_table as *mut ::core::ffi::c_void;
}
pub const __SIZEOF_POINTER__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
