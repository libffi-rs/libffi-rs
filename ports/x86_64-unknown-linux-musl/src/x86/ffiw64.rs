extern "C" {
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ffi_tramp_is_present(closure: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn ffi_call_win64(
        stack: *mut ::core::ffi::c_void,
        _: *mut win64_call_frame,
        closure: *mut ::core::ffi::c_void,
    );
    fn ffi_closure_win64();
    fn ffi_closure_win64_alt();
    fn ffi_go_closure_win64();
    fn ffi_tramp_set_parms(
        tramp: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        code: *mut ::core::ffi::c_void,
    );
}
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 2;
pub const FFI_LAST_ABI: ffi_abi = 5;
pub const FFI_GNUW64: ffi_abi = 4;
pub const FFI_EFI64: ffi_abi = 3;
pub const FFI_WIN64: ffi_abi = 3;
pub const FFI_UNIX64: ffi_abi = 2;
pub const FFI_FIRST_ABI: ffi_abi = 1;
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
pub type UINT8 = ::core::ffi::c_uchar;
pub type UINT16 = ::core::ffi::c_ushort;
pub type UINT32 = ::core::ffi::c_uint;
pub type UINT64 = ::core::ffi::c_ulong;
pub type uintptr_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct win64_call_frame {
    pub rbp: UINT64,
    pub retaddr: UINT64,
    pub fn_0: UINT64,
    pub flags: UINT64,
    pub rvalue: UINT64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct win64_closure_frame {
    pub rvalue: [UINT64; 2],
    pub fargs: [UINT64; 4],
    pub retaddr: UINT64,
    pub args: [UINT64; 0],
}
pub const FFI_TYPE_VOID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FFI_TYPE_FLOAT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FFI_TYPE_DOUBLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FFI_TYPE_LONGDOUBLE: ::core::ffi::c_int = 4;
pub const FFI_TYPE_UINT64: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const FFI_TYPE_STRUCT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FFI_TYPE_COMPLEX: ::core::ffi::c_int = 15;
pub const FFI_TYPE_UINT128: ::core::ffi::c_int = 16;
pub const FFI_TYPE_SINT128: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const FFI_TYPE_LAST: ::core::ffi::c_int = FFI_TYPE_SINT128;
pub const FFI_TYPE_SMALL_STRUCT_1B: ::core::ffi::c_int = FFI_TYPE_LAST + 1 as ::core::ffi::c_int;
pub const FFI_TYPE_SMALL_STRUCT_2B: ::core::ffi::c_int = FFI_TYPE_LAST + 2 as ::core::ffi::c_int;
pub const FFI_TYPE_SMALL_STRUCT_4B: ::core::ffi::c_int = FFI_TYPE_LAST + 3 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep_efi64(mut cif: *mut ffi_cif) -> ffi_status {
    let mut flags: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    match (*cif).abi as ::core::ffi::c_uint {
        3 | 4 => {}
        _ => return FFI_BAD_ABI,
    }
    flags = (*(*cif).rtype).type_0 as ::core::ffi::c_int;
    let mut current_block_10: u64;
    match flags {
        FFI_TYPE_LONGDOUBLE => {
            if (*cif).abi as ::core::ffi::c_uint
                == FFI_GNUW64 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                flags = FFI_TYPE_STRUCT;
            }
            current_block_10 = 1054647088692577877;
        }
        FFI_TYPE_COMPLEX => {
            flags = FFI_TYPE_STRUCT;
            current_block_10 = 15525655650990252292;
        }
        FFI_TYPE_STRUCT => {
            current_block_10 = 15525655650990252292;
        }
        _ => {
            current_block_10 = 1054647088692577877;
        }
    }
    match current_block_10 {
        15525655650990252292 => match (*(*cif).rtype).size {
            8 => {
                flags = FFI_TYPE_UINT64;
            }
            4 => {
                flags = FFI_TYPE_SMALL_STRUCT_4B;
            }
            2 => {
                flags = FFI_TYPE_SMALL_STRUCT_2B;
            }
            1 => {
                flags = FFI_TYPE_SMALL_STRUCT_1B;
            }
            _ => {}
        },
        _ => {}
    }
    (*cif).flags = flags as ::core::ffi::c_uint;
    n = (*cif).nargs as ::core::ffi::c_int;
    n += (flags == FFI_TYPE_STRUCT) as ::core::ffi::c_int;
    if n < 4 as ::core::ffi::c_int {
        n = 4 as ::core::ffi::c_int;
    }
    (*cif).bytes = (n * 8 as ::core::ffi::c_int) as ::core::ffi::c_uint;
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
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut stack: *mut UINT64 = ::core::ptr::null_mut::<UINT64>();
    let mut rsize: size_t = 0;
    let mut frame: *mut win64_call_frame = ::core::ptr::null_mut::<win64_call_frame>();
    let mut arg_types: *mut *mut ffi_type = (*cif).arg_types;
    let mut nargs: ::core::ffi::c_int = (*cif).nargs as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nargs {
        let mut at: *mut ffi_type = *arg_types.offset(i as isize);
        let mut size: ::core::ffi::c_int = (*at).size as ::core::ffi::c_int;
        let mut needcopy: bool = false_0 != 0;
        match (*at).type_0 as ::core::ffi::c_int {
            FFI_TYPE_UINT128 | FFI_TYPE_SINT128 => {
                needcopy = true_0 != 0;
            }
            FFI_TYPE_STRUCT => match size {
                1 | 2 | 4 | 8 => {}
                _ => {
                    needcopy = true_0 != 0;
                }
            },
            _ => {}
        }
        if needcopy {
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
        i += 1;
    }
    flags = (*cif).flags as ::core::ffi::c_int;
    rsize = 0 as size_t;
    if rvalue.is_null() {
        if flags == FFI_TYPE_STRUCT {
            rsize = (*(*cif).rtype).size;
        } else {
            flags = FFI_TYPE_VOID;
        }
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        ((*cif).bytes as usize)
            .wrapping_add(::core::mem::size_of::<win64_call_frame>() as usize)
            .wrapping_add(rsize as usize) as usize,
    ));
    stack = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut UINT64;
    frame =
        (stack as *mut ::core::ffi::c_char).offset((*cif).bytes as isize) as *mut win64_call_frame;
    if rsize != 0 {
        rvalue = frame.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void;
    }
    (*frame).fn_0 =
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, uintptr_t>(fn_0) as UINT64;
    (*frame).flags = flags as UINT64;
    (*frame).rvalue = rvalue as uintptr_t as UINT64;
    j = 0 as ::core::ffi::c_int;
    if flags == FFI_TYPE_STRUCT {
        *stack.offset(0 as ::core::ffi::c_int as isize) = rvalue as uintptr_t as UINT64;
        j = 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    n = (*cif).nargs as ::core::ffi::c_int;
    while i < n {
        match (**(*cif).arg_types.offset(i as isize)).size {
            8 => {
                *stack.offset(j as isize) = *(*avalue.offset(i as isize) as *mut UINT64);
            }
            4 => {
                *stack.offset(j as isize) = *(*avalue.offset(i as isize) as *mut UINT32) as UINT64;
            }
            2 => {
                *stack.offset(j as isize) = *(*avalue.offset(i as isize) as *mut UINT16) as UINT64;
            }
            1 => {
                *stack.offset(j as isize) = *(*avalue.offset(i as isize) as *mut UINT8) as UINT64;
            }
            _ => {
                *stack.offset(j as isize) = *avalue.offset(i as isize) as uintptr_t as UINT64;
            }
        }
        i += 1;
        j += 1;
    }
    ffi_call_win64(stack as *mut ::core::ffi::c_void, frame, closure);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_call_efi64(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
) {
    ffi_call_int(cif, fn_0, rvalue, avalue, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_call_go_efi64(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
    mut closure: *mut ::core::ffi::c_void,
) {
    ffi_call_int(cif, fn_0, rvalue, avalue, closure);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_closure_loc_efi64(
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
    let mut tramp: *mut ::core::ffi::c_char =
        &raw mut (*closure).c2rust_unnamed.tramp as *mut ::core::ffi::c_char;
    match (*cif).abi as ::core::ffi::c_uint {
        3 | 4 => {}
        _ => return FFI_BAD_ABI,
    }
    if ffi_tramp_is_present(closure as *mut ::core::ffi::c_void) != 0 {
        ffi_tramp_set_parms(
            (*closure).c2rust_unnamed.ftramp,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, *mut ::core::ffi::c_void>(
                Some(ffi_closure_win64_alt as unsafe extern "C" fn() -> ()),
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
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, uintptr_t>(Some(
                ffi_closure_win64 as unsafe extern "C" fn() -> (),
            )) as UINT64;
    }
    (*closure).cif = cif;
    (*closure).fun = fun;
    (*closure).user_data = user_data;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_go_closure_efi64(
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
    match (*cif).abi as ::core::ffi::c_uint {
        3 | 4 => {}
        _ => return FFI_BAD_ABI,
    }
    (*closure).tramp = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        *mut ::core::ffi::c_void,
    >(Some(ffi_go_closure_win64 as unsafe extern "C" fn() -> ()));
    (*closure).cif = cif;
    (*closure).fun = fun;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_win64_inner(
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
    mut frame: *mut win64_closure_frame,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut avalue: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut rvalue: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut nreg: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        ((*cif).nargs as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
            as usize,
    ));
    avalue = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_void;
    rvalue = &raw mut (*frame).rvalue as *mut UINT64 as *mut ::core::ffi::c_void;
    nreg = 0 as ::core::ffi::c_int;
    flags = (*cif).flags as ::core::ffi::c_int;
    if flags == FFI_TYPE_STRUCT {
        rvalue = *(&raw mut (*frame).args as *mut UINT64).offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void;
        (*frame).rvalue[0 as ::core::ffi::c_int as usize] =
            *(&raw mut (*frame).args as *mut UINT64).offset(0 as ::core::ffi::c_int as isize);
        nreg = 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    n = (*cif).nargs as ::core::ffi::c_int;
    while i < n {
        let mut size: size_t = (**(*cif).arg_types.offset(i as isize)).size;
        let mut type_0: size_t = (**(*cif).arg_types.offset(i as isize)).type_0 as size_t;
        let mut a: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if type_0 == FFI_TYPE_DOUBLE as size_t || type_0 == FFI_TYPE_FLOAT as size_t {
            if nreg < 4 as ::core::ffi::c_int {
                a = (&raw mut (*frame).fargs as *mut UINT64).offset(nreg as isize) as *mut UINT64
                    as *mut ::core::ffi::c_void;
            } else {
                a = (&raw mut (*frame).args as *mut UINT64).offset(nreg as isize) as *mut UINT64
                    as *mut ::core::ffi::c_void;
            }
        } else if size == 1 as size_t
            || size == 2 as size_t
            || size == 4 as size_t
            || size == 8 as size_t
        {
            a = (&raw mut (*frame).args as *mut UINT64).offset(nreg as isize) as *mut UINT64
                as *mut ::core::ffi::c_void;
        } else {
            a = *(&raw mut (*frame).args as *mut UINT64).offset(nreg as isize)
                as *mut ::core::ffi::c_void;
        }
        let ref mut fresh1 = *avalue.offset(i as isize);
        *fresh1 = a;
        i += 1;
        nreg += 1;
    }
    fun.expect("non-null function pointer")(cif, rvalue, avalue, user_data);
    return flags;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
