extern "C" {
    static mut ffi_type_double: ffi_type;
    static mut ffi_type_pointer: ffi_type;
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
    fn ffi_call_asm_stack_bridge(
        heap_frame: *mut ::core::ffi::c_void,
        context_offset: size_t,
        total_size: size_t,
        fn_0: Option<unsafe extern "C" fn() -> ()>,
        closure: *mut ::core::ffi::c_void,
    );
    fn ffi_closure_asm();
    fn ffi_go_closure_asm();
}
pub type ffi_arg = ::core::ffi::c_ulong;
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 3;
pub const FFI_LAST_ABI: ffi_abi = 7;
pub const FFI_ILP32D: ffi_abi = 6;
pub const FFI_ILP32F: ffi_abi = 5;
pub const FFI_ILP32S: ffi_abi = 4;
pub const FFI_LP64D: ffi_abi = 3;
pub const FFI_LP64F: ffi_abi = 2;
pub const FFI_LP64S: ffi_abi = 1;
pub const FFI_FIRST_ABI: ffi_abi = 0;
pub type size_t = usize;

/// LoongArch64 GNU `long double`: IEEE binary128, 16-byte size/alignment.
/// Rust stable cannot name the `f128` primitive yet; libffi only needs its layout.
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct FfiLongDouble(pub [u8; 16]);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::core::ffi::c_longlong,
    pub __clang_max_align_nonce2: FfiLongDouble,
}
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
    pub loongarch_nfixedargs: ::core::ffi::c_uint,
    pub loongarch_unused: ::core::ffi::c_uint,
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
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
pub type uint64_t = __uint64_t;
pub type __uint64_t = u64;
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
pub struct call_builder {
    pub aregs: *mut call_context,
    pub used_integer: ::core::ffi::c_int,
    pub used_float: ::core::ffi::c_int,
    pub used_stack: *mut size_t,
    pub stack: *mut size_t,
    pub next_struct_area: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct call_context {
    pub fa: [::core::ffi::c_double; 8],
    pub a: [size_t; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct float_struct_info {
    pub as_elements: ::core::ffi::c_char,
    pub type1: ::core::ffi::c_char,
    pub offset2: ::core::ffi::c_char,
    pub type2: ::core::ffi::c_char,
}
pub type int64_t = __int64_t;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type __int32_t = i32;
pub type int16_t = __int16_t;
pub type __int16_t = i16;
pub type uint16_t = __uint16_t;
pub type __uint16_t = u16;
pub type int8_t = __int8_t;
pub type __int8_t = i8;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
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
        let fresh5 = out;
        out = out.offset(1);
        *fresh5 = in_0;
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
            .wrapping_sub(1 as size_t)
            | ((*fields[1 as ::core::ffi::c_int as usize]).alignment as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(1 as size_t) as ::core::ffi::c_char;
        ret.as_elements = 1 as ::core::ffi::c_char;
    }
    return ret;
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
            let fresh6 = (*cb).used_float;
            (*cb).used_float = (*cb).used_float + 1;
            *((&raw mut (*(*cb).aregs).fa as *mut ::core::ffi::c_double).offset(fresh6 as isize)
                as *mut ::core::ffi::c_float) = *(data as *mut ::core::ffi::c_float);
            return;
        }
        FFI_TYPE_DOUBLE => {
            let fresh7 = (*cb).used_float;
            (*cb).used_float = (*cb).used_float + 1;
            (*(*cb).aregs).fa[fresh7 as usize] = *(data as *mut ::core::ffi::c_double);
            return;
        }
        _ => {}
    }
    if (*cb).used_integer == NARGREG {
        let fresh8 = (*cb).used_stack;
        (*cb).used_stack = (*cb).used_stack.offset(1);
        *fresh8 = value;
    } else {
        let fresh9 = (*cb).used_integer;
        (*cb).used_integer = (*cb).used_integer + 1;
        (*(*cb).aregs).a[fresh9 as usize] = value;
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
            let fresh1 = (*cb).used_float;
            (*cb).used_float = (*cb).used_float + 1;
            *(data as *mut ::core::ffi::c_float) = *((&raw mut (*(*cb).aregs).fa
                as *mut ::core::ffi::c_double)
                .offset(fresh1 as isize)
                as *mut ::core::ffi::c_float);
            return;
        }
        FFI_TYPE_DOUBLE => {
            let fresh2 = (*cb).used_float;
            (*cb).used_float = (*cb).used_float + 1;
            *(data as *mut ::core::ffi::c_double) = (*(*cb).aregs).fa[fresh2 as usize];
            return;
        }
        _ => {}
    }
    if (*cb).used_integer == NARGREG {
        let fresh3 = (*cb).used_stack;
        (*cb).used_stack = (*cb).used_stack.offset(1);
        value = *fresh3;
    } else {
        let fresh4 = (*cb).used_integer;
        (*cb).used_integer = (*cb).used_integer + 1;
        value = (*(*cb).aregs).a[fresh4 as usize];
    }
    match type_0 {
        FFI_TYPE_UINT8 | FFI_TYPE_SINT8 | FFI_TYPE_UINT16 | FFI_TYPE_SINT16 | FFI_TYPE_UINT32
        | FFI_TYPE_SINT32 | FFI_TYPE_UINT64 | FFI_TYPE_SINT64 | FFI_TYPE_POINTER => {
            *(data as *mut ffi_arg) = value as ffi_arg;
        }
        _ => {}
    };
}
unsafe extern "C" fn allocate_and_copy_struct_to_stack(
    mut cb: *mut call_builder,
    mut data: *mut ::core::ffi::c_void,
    mut type_0: *mut ffi_type,
) -> *mut ::core::ffi::c_void {
    let mut dest: size_t = (*cb).next_struct_area.wrapping_sub((*type_0).size);
    dest = dest & -((*type_0).alignment as ::core::ffi::c_int) as size_t;
    (*cb).next_struct_area = dest;
    return memcpy(
        ((*cb).stack as *mut ::core::ffi::c_char).offset(dest as isize) as *mut ::core::ffi::c_void,
        data,
        (*type_0).size,
    );
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
    let mut promoted: ::core::ffi::c_double = 0.;
    if var != 0 && (*type_0).type_0 as ::core::ffi::c_int == FFI_TYPE_FLOAT {
        promoted = *(data as *mut ::core::ffi::c_float) as ::core::ffi::c_double;
        type_0 = &raw mut ffi_type_double;
        data = &raw mut promoted as *mut ::core::ffi::c_void;
    }
    if (*type_0).size > (2 as ::core::ffi::c_int * __SIZEOF_POINTER__) as size_t {
        allocate_and_copy_struct_to_stack(cb, data, type_0);
        data = ((*cb).stack as *mut ::core::ffi::c_char).offset((*cb).next_struct_area as isize)
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
                (*cb).used_integer = (((*cb).used_integer as size_t).wrapping_sub(1 as size_t)
                    | (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(1 as size_t)
                    as ::core::ffi::c_int;
            }
            (*cb).used_stack = (((*cb).used_stack as size_t).wrapping_sub(1 as size_t)
                | (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as size_t)
                .wrapping_add(1 as size_t) as *mut size_t;
        }
        memcpy(
            &raw mut realign as *mut size_t as *mut ::core::ffi::c_void,
            data,
            (*type_0).size,
        );
        if (*type_0).size > 0 as size_t {
            marshal_atom(
                cb,
                FFI_TYPE_POINTER,
                &raw mut realign as *mut size_t as *mut ::core::ffi::c_void,
            );
        }
        if (*type_0).size > __SIZEOF_POINTER__ as size_t {
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
    if var != 0 && (*type_0).type_0 as ::core::ffi::c_int == FFI_TYPE_FLOAT {
        let mut m: ::core::ffi::c_int = (*cb).used_integer;
        let mut promoted: *mut ::core::ffi::c_void = (if m < NARGREG {
            (&raw mut (*(*cb).aregs).a as *mut size_t).offset(m as isize)
        } else {
            (*cb)
                .used_stack
                .offset(m as isize)
                .offset(-(NARGREG as isize))
                .offset(1 as ::core::ffi::c_int as isize)
        }) as *mut ::core::ffi::c_void;
        *(promoted as *mut ::core::ffi::c_float) =
            *(promoted as *mut ::core::ffi::c_double) as ::core::ffi::c_float;
    }
    if (*type_0).size > (2 as ::core::ffi::c_int * __SIZEOF_POINTER__) as size_t {
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
                (*cb).used_integer = (((*cb).used_integer as size_t).wrapping_sub(1 as size_t)
                    | (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(1 as size_t)
                    as ::core::ffi::c_int;
            }
            (*cb).used_stack = (((*cb).used_stack as size_t).wrapping_sub(1 as size_t)
                | (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as size_t)
                .wrapping_add(1 as size_t) as *mut size_t;
        }
        if (*type_0).size > 0 as size_t {
            unmarshal_atom(
                cb,
                FFI_TYPE_POINTER,
                &raw mut realign as *mut size_t as *mut ::core::ffi::c_void,
            );
        }
        if (*type_0).size > __SIZEOF_POINTER__ as size_t {
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
    return ((*type_0).size > (2 as ::core::ffi::c_int * __SIZEOF_POINTER__) as size_t)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep(mut cif: *mut ffi_cif) -> ffi_status {
    (*cif).loongarch_nfixedargs = (*cif).nargs;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep_var(
    mut cif: *mut ffi_cif,
    mut nfixedargs: ::core::ffi::c_uint,
    mut ntotalargs: ::core::ffi::c_uint,
) -> ffi_status {
    (*cif).loongarch_nfixedargs = nfixedargs;
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
    let mut arg_bytes: size_t = (((*cif).bytes as size_t).wrapping_sub(1 as size_t)
        | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
        .wrapping_add(1 as size_t);
    let mut extra_bytes: size_t = if (*cif).nargs <= 3 as ::core::ffi::c_uint {
        0 as size_t
    } else {
        ((2 as usize)
            .wrapping_mul(::core::mem::size_of::<size_t>() as usize)
            .wrapping_mul((*cif).nargs.wrapping_sub(3 as ::core::ffi::c_uint) as usize)
            .wrapping_sub(1 as size_t)
            | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(1 as size_t)
    };
    let mut rval_bytes: size_t = 0 as size_t;
    if rvalue.is_null()
        && (*(*cif).rtype).size > (2 as ::core::ffi::c_int * __SIZEOF_POINTER__) as size_t
    {
        rval_bytes = ((*(*cif).rtype).size.wrapping_sub(1 as size_t)
            | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(1 as size_t);
    }
    let mut alloc_size: size_t = arg_bytes
        .wrapping_add(extra_bytes)
        .wrapping_add(rval_bytes)
        .wrapping_add(::core::mem::size_of::<call_context>() as size_t);
    let mut alloc_base: size_t = 0;
    if ::core::mem::align_of::<max_align_t>() >= STKALIGN as usize {
        alloca_allocations.push(::std::vec::from_elem(0, alloc_size as usize));
        alloc_base = alloca_allocations.last_mut().unwrap().as_mut_ptr() as size_t;
    } else {
        alloca_allocations.push(::std::vec::from_elem(
            0,
            alloc_size
                .wrapping_add(16 as size_t)
                .wrapping_sub(1 as size_t) as usize,
        ));
        alloc_base = ((alloca_allocations.last_mut().unwrap().as_mut_ptr() as size_t)
            .wrapping_sub(1 as size_t)
            | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(1 as size_t);
    }
    if rval_bytes != 0 {
        rvalue = alloc_base.wrapping_add(arg_bytes) as *mut ::core::ffi::c_void;
    }
    let mut cb: call_builder = call_builder {
        aregs: ::core::ptr::null_mut::<call_context>(),
        used_integer: 0,
        used_float: 0,
        used_stack: ::core::ptr::null_mut::<size_t>(),
        stack: ::core::ptr::null_mut::<size_t>(),
        next_struct_area: 0,
    };
    cb.used_integer = 0 as ::core::ffi::c_int;
    cb.used_float = cb.used_integer;
    cb.aregs = alloc_base
        .wrapping_add(arg_bytes)
        .wrapping_add(extra_bytes)
        .wrapping_add(rval_bytes) as *mut call_context;
    cb.used_stack = alloc_base as *mut ::core::ffi::c_char as *mut size_t;
    cb.stack = (alloc_base as *mut ::core::ffi::c_char)
        .offset(extra_bytes as isize)
        .offset(rval_bytes as isize) as *mut size_t;
    cb.next_struct_area = arg_bytes;
    let mut return_by_ref: ::core::ffi::c_int =
        passed_by_ref(&raw mut cb, (*cif).rtype, 0 as ::core::ffi::c_int);
    if return_by_ref != 0 {
        let fresh0 = cb.used_integer;
        cb.used_integer = cb.used_integer + 1;
        (*cb.aregs).a[fresh0 as usize] = rvalue as size_t;
    }
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < (*cif).nargs {
        marshal(
            &raw mut cb,
            *(*cif).arg_types.offset(i as isize),
            (i as ::core::ffi::c_uint >= (*cif).loongarch_nfixedargs) as ::core::ffi::c_int,
            *avalue.offset(i as isize),
        );
        i += 1;
    }
    ffi_call_asm_stack_bridge(
        alloc_base as *mut ::core::ffi::c_void,
        arg_bytes.wrapping_add(extra_bytes).wrapping_add(rval_bytes),
        alloc_size,
        fn_0,
        closure,
    );
    cb.used_integer = 0 as ::core::ffi::c_int;
    cb.used_float = cb.used_integer;
    if return_by_ref == 0 && !rvalue.is_null() {
        unmarshal(&raw mut cb, (*cif).rtype, 0 as ::core::ffi::c_int, rvalue);
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
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ),
    >,
    user_data: *mut ::core::ffi::c_void,
    codeloc: *mut ::core::ffi::c_void,
) -> ffi_status {
    if (*cif).abi <= FFI_FIRST_ABI || (*cif).abi >= FFI_LAST_ABI {
        return FFI_BAD_ABI;
    }

    if ffi_tramp_is_present(closure.cast()) != 0 {
        ffi_tramp_set_parms(
            (*closure).c2rust_unnamed.ftramp,
            ffi_closure_asm as *mut ::core::ffi::c_void,
            closure.cast(),
        );
    } else {
        // Exact instruction template from src/loongarch/ffi.c.
        let tramp = codeloc.cast::<u32>();
        tramp.add(0).write(0x1800_000c); // pcaddi $t0, 0
        tramp.add(1).write(0x28c0_418d); // ld.d $t1, $t0, 16
        tramp.add(2).write(0x4c00_01a0); // jirl $zero, $t1, 0
        tramp.add(3).write(0x0340_0000); // nop
        tramp
            .add(4)
            .write((ffi_closure_asm as *const () as usize & 0xffff_ffff) as u32);
        tramp
            .add(5)
            .write((ffi_closure_asm as *const () as usize >> 32) as u32);
        // GCC lowers __builtin___clear_cache to this LoongArch barrier.
        ::core::arch::asm!("ibar 0", options(nostack, preserves_flags));
    }

    (*closure).cif = cif;
    (*closure).fun = fun;
    (*closure).user_data = user_data;
    FFI_OK
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
        ((*cif).nargs as usize).wrapping_mul(
            (2 as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize),
        ) as usize,
    ));
    let mut astorage: *mut ::core::ffi::c_char =
        alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    let mut rvalue: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut cb: call_builder = call_builder {
        aregs: ::core::ptr::null_mut::<call_context>(),
        used_integer: 0,
        used_float: 0,
        used_stack: ::core::ptr::null_mut::<size_t>(),
        stack: ::core::ptr::null_mut::<size_t>(),
        next_struct_area: 0,
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
        rvalue = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void;
    }
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < (*cif).nargs {
        let ref mut fresh10 = *avalue.offset(i as isize);
        *fresh10 = unmarshal(
            &raw mut cb,
            *(*cif).arg_types.offset(i as isize),
            (i as ::core::ffi::c_uint >= (*cif).loongarch_nfixedargs) as ::core::ffi::c_int,
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
    *tramp_size = 16 as size_t;
    *map_size = ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as size_t;
    return &raw mut trampoline_code_table as *mut ::core::ffi::c_void;
}
pub const __SIZEOF_POINTER__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
