extern "C" {
    fn ffi_call_i386_from_rust(_: *mut call_frame, _: *mut ::core::ffi::c_char);
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
    fn ffi_closure_i386();
    fn ffi_closure_STDCALL();
    fn ffi_closure_REGISTER();
    fn ffi_closure_i386_alt();
    fn ffi_closure_STDCALL_alt();
    fn ffi_closure_REGISTER_alt();
    fn ffi_go_closure_EAX();
    fn ffi_go_closure_ECX();
    fn ffi_go_closure_STDCALL();
    fn ffi_closure_raw_SYSV();
    fn ffi_closure_raw_THISCALL();
}
pub type ffi_arg = ::core::ffi::c_ulong;
pub type ffi_sarg = ::core::ffi::c_long;
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 1;
pub const FFI_LAST_ABI: ffi_abi = 9;
pub const FFI_MS_CDECL: ffi_abi = 8;
pub const FFI_REGISTER: ffi_abi = 7;
pub const FFI_PASCAL: ffi_abi = 6;
pub const FFI_STDCALL: ffi_abi = 5;
pub const FFI_FASTCALL: ffi_abi = 4;
pub const FFI_THISCALL: ffi_abi = 3;
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
pub union ffi_raw {
    pub sint: ffi_sarg,
    pub uint: ffi_arg,
    pub flt: ::core::ffi::c_float,
    pub data: [::core::ffi::c_char; 4],
    pub ptr: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct call_frame {
    pub ebp: *mut ::core::ffi::c_void,
    pub retaddr: *mut ::core::ffi::c_void,
    pub fn_0: Option<unsafe extern "C" fn() -> ()>,
    pub flags: ::core::ffi::c_int,
    pub rvalue: *mut ::core::ffi::c_void,
    pub regs: [::core::ffi::c_uint; 3],
}
pub type UINT32 = ::core::ffi::c_uint;
pub type UINT16 = ::core::ffi::c_ushort;
pub type SINT16 = ::core::ffi::c_short;
pub type UINT8 = ::core::ffi::c_uchar;
pub type SINT8 = ::core::ffi::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct abi_params {
    pub dir: ::core::ffi::c_int,
    pub static_chain: ::core::ffi::c_int,
    pub nregs: ::core::ffi::c_int,
    pub regs: [::core::ffi::c_int; 3],
}
pub type uintptr_t = usize;
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
    pub tramp: [::core::ffi::c_char; 16],
    pub ftramp: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_raw_closure {
    pub tramp: [::core::ffi::c_char; 16],
    pub cif: *mut ffi_cif,
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
pub struct closure_frame {
    pub rettemp: [::core::ffi::c_uint; 4],
    pub regs: [::core::ffi::c_uint; 3],
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
pub const FFI_TYPE_UINT64: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const FFI_TYPE_SINT64: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const FFI_TYPE_STRUCT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FFI_TYPE_POINTER: ::core::ffi::c_int = 14;
pub const FFI_TYPE_COMPLEX: ::core::ffi::c_int = 15;
pub const FFI_SIZEOF_ARG: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const X86_RET_FLOAT: ::core::ffi::c_int = 0;
pub const X86_RET_DOUBLE: ::core::ffi::c_int = 1;
pub const X86_RET_LDOUBLE: ::core::ffi::c_int = 2;
pub const X86_RET_SINT8: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const X86_RET_SINT16: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const X86_RET_UINT8: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const X86_RET_UINT16: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const X86_RET_INT64: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const X86_RET_INT32: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const X86_RET_VOID: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const X86_RET_STRUCTPOP: ::core::ffi::c_int = 10;
pub const X86_RET_STRUCTARG: ::core::ffi::c_int = 11;
pub const X86_RET_STRUCT_2B: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const X86_RET_POP_SHIFT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const R_EAX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const R_EDX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const R_ECX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const X86_TRAMP_MAP_SHIFT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const X86_TRAMP_MAP_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << X86_TRAMP_MAP_SHIFT;
pub const X86_TRAMP_SIZE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep(mut cif: *mut ffi_cif) -> ffi_status {
    let mut bytes: size_t = 0 as size_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut cabi: ::core::ffi::c_int = (*cif).abi as ::core::ffi::c_int;
    match cabi {
        1 | 5 | 3 | 4 | 8 | 6 | 7 => {}
        _ => return FFI_BAD_ABI,
    }
    let mut current_block_20: u64;
    match (*(*cif).rtype).type_0 as ::core::ffi::c_int {
        FFI_TYPE_VOID => {
            flags = X86_RET_VOID;
            current_block_20 = 11743904203796629665;
        }
        FFI_TYPE_FLOAT => {
            flags = X86_RET_FLOAT;
            current_block_20 = 11743904203796629665;
        }
        FFI_TYPE_DOUBLE => {
            flags = X86_RET_DOUBLE;
            current_block_20 = 11743904203796629665;
        }
        FFI_TYPE_LONGDOUBLE => {
            flags = X86_RET_LDOUBLE;
            current_block_20 = 11743904203796629665;
        }
        FFI_TYPE_UINT8 => {
            flags = X86_RET_UINT8;
            current_block_20 = 11743904203796629665;
        }
        FFI_TYPE_UINT16 => {
            flags = X86_RET_UINT16;
            current_block_20 = 11743904203796629665;
        }
        FFI_TYPE_SINT8 => {
            flags = X86_RET_SINT8;
            current_block_20 = 11743904203796629665;
        }
        FFI_TYPE_SINT16 => {
            flags = X86_RET_SINT16;
            current_block_20 = 11743904203796629665;
        }
        FFI_TYPE_INT | FFI_TYPE_SINT32 | FFI_TYPE_UINT32 | FFI_TYPE_POINTER => {
            flags = X86_RET_INT32;
            current_block_20 = 11743904203796629665;
        }
        FFI_TYPE_SINT64 | FFI_TYPE_UINT64 => {
            flags = X86_RET_INT64;
            current_block_20 = 11743904203796629665;
        }
        FFI_TYPE_STRUCT => {
            current_block_20 = 14730518782399518168;
        }
        FFI_TYPE_COMPLEX => {
            match (**(*(*cif).rtype)
                .elements
                .offset(0 as ::core::ffi::c_int as isize))
            .type_0 as ::core::ffi::c_int
            {
                FFI_TYPE_DOUBLE | FFI_TYPE_LONGDOUBLE | FFI_TYPE_SINT64 | FFI_TYPE_UINT64 => {
                    current_block_20 = 14730518782399518168;
                }
                FFI_TYPE_FLOAT | FFI_TYPE_INT | FFI_TYPE_SINT32 | FFI_TYPE_UINT32 => {
                    current_block_20 = 15032839488014255436;
                    match current_block_20 {
                        8343906084206216984 => return FFI_BAD_TYPEDEF,
                        6777586944787613766 => {
                            flags = X86_RET_INT32;
                        }
                        14474407416135258090 => {
                            flags = X86_RET_STRUCT_2B;
                        }
                        _ => {
                            flags = X86_RET_INT64;
                        }
                    }
                    current_block_20 = 11743904203796629665;
                }
                FFI_TYPE_SINT16 | FFI_TYPE_UINT16 => {
                    current_block_20 = 6777586944787613766;
                    match current_block_20 {
                        8343906084206216984 => return FFI_BAD_TYPEDEF,
                        6777586944787613766 => {
                            flags = X86_RET_INT32;
                        }
                        14474407416135258090 => {
                            flags = X86_RET_STRUCT_2B;
                        }
                        _ => {
                            flags = X86_RET_INT64;
                        }
                    }
                    current_block_20 = 11743904203796629665;
                }
                FFI_TYPE_SINT8 | FFI_TYPE_UINT8 => {
                    current_block_20 = 14474407416135258090;
                    match current_block_20 {
                        8343906084206216984 => return FFI_BAD_TYPEDEF,
                        6777586944787613766 => {
                            flags = X86_RET_INT32;
                        }
                        14474407416135258090 => {
                            flags = X86_RET_STRUCT_2B;
                        }
                        _ => {
                            flags = X86_RET_INT64;
                        }
                    }
                    current_block_20 = 11743904203796629665;
                }
                _ => {
                    current_block_20 = 8343906084206216984;
                    match current_block_20 {
                        8343906084206216984 => return FFI_BAD_TYPEDEF,
                        6777586944787613766 => {
                            flags = X86_RET_INT32;
                        }
                        14474407416135258090 => {
                            flags = X86_RET_STRUCT_2B;
                        }
                        _ => {
                            flags = X86_RET_INT64;
                        }
                    }
                    current_block_20 = 11743904203796629665;
                }
            }
        }
        _ => return FFI_BAD_TYPEDEF,
    }
    match current_block_20 {
        14730518782399518168 => {
            match cabi {
                3 | 4 | 5 | 8 => {
                    flags = X86_RET_STRUCTARG;
                }
                _ => {
                    flags = X86_RET_STRUCTPOP;
                }
            }
            bytes = bytes.wrapping_add(
                (::core::mem::size_of::<*mut ::core::ffi::c_void>().wrapping_sub(1 as size_t)
                    | (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(1 as size_t),
            );
        }
        _ => {}
    }
    (*cif).flags = flags as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    n = (*cif).nargs as ::core::ffi::c_int;
    while i < n {
        let mut t: *mut ffi_type = *(*cif).arg_types.offset(i as isize);
        bytes = (bytes.wrapping_sub(1 as size_t)
            | ((*t).alignment as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(1 as size_t);
        bytes = bytes.wrapping_add(
            ((*t).size.wrapping_sub(1 as size_t)
                | (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_add(1 as size_t),
        );
        i += 1;
    }
    (*cif).bytes = bytes as ::core::ffi::c_uint;
    return FFI_OK;
}
unsafe extern "C" fn extend_basic_type(
    mut arg: *mut ::core::ffi::c_void,
    mut type_0: ::core::ffi::c_int,
) -> ffi_arg {
    match type_0 {
        FFI_TYPE_SINT8 => return *(arg as *mut SINT8) as ffi_arg,
        FFI_TYPE_UINT8 => return *(arg as *mut UINT8) as ffi_arg,
        FFI_TYPE_SINT16 => return *(arg as *mut SINT16) as ffi_arg,
        FFI_TYPE_UINT16 => return *(arg as *mut UINT16) as ffi_arg,
        FFI_TYPE_SINT32 | FFI_TYPE_UINT32 | FFI_TYPE_POINTER | FFI_TYPE_FLOAT => {
            return *(arg as *mut UINT32) as ffi_arg;
        }
        _ => {
            abort();
        }
    };
}
static mut abi_params: [abi_params; 9] = [
    abi_params {
        dir: 0,
        static_chain: 0,
        nregs: 0,
        regs: [0; 3],
    },
    abi_params {
        dir: 1 as ::core::ffi::c_int,
        static_chain: R_ECX,
        nregs: 0 as ::core::ffi::c_int,
        regs: [0; 3],
    },
    abi_params {
        dir: 0,
        static_chain: 0,
        nregs: 0,
        regs: [0; 3],
    },
    abi_params {
        dir: 1 as ::core::ffi::c_int,
        static_chain: R_EAX,
        nregs: 1 as ::core::ffi::c_int,
        regs: [R_ECX, 0, 0],
    },
    abi_params {
        dir: 1 as ::core::ffi::c_int,
        static_chain: R_EAX,
        nregs: 2 as ::core::ffi::c_int,
        regs: [R_ECX, R_EDX, 0],
    },
    abi_params {
        dir: 1 as ::core::ffi::c_int,
        static_chain: R_ECX,
        nregs: 0 as ::core::ffi::c_int,
        regs: [0; 3],
    },
    abi_params {
        dir: -(1 as ::core::ffi::c_int),
        static_chain: R_ECX,
        nregs: 0 as ::core::ffi::c_int,
        regs: [0; 3],
    },
    abi_params {
        dir: -(1 as ::core::ffi::c_int),
        static_chain: R_ECX,
        nregs: 3 as ::core::ffi::c_int,
        regs: [R_EAX, R_EDX, R_ECX],
    },
    abi_params {
        dir: 1 as ::core::ffi::c_int,
        static_chain: R_ECX,
        nregs: 0 as ::core::ffi::c_int,
        regs: [0; 3],
    },
];
unsafe extern "C" fn ffi_call_int(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
    mut closure: *mut ::core::ffi::c_void,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut rsize: size_t = 0;
    let mut bytes: size_t = 0;
    let mut frame: *mut call_frame = ::core::ptr::null_mut::<call_frame>();
    let mut stack: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut argp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut arg_types: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut flags: ::core::ffi::c_int = 0;
    let mut cabi: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut dir: ::core::ffi::c_int = 0;
    let mut narg_reg: ::core::ffi::c_int = 0;
    let mut pabi: *const abi_params = ::core::ptr::null::<abi_params>();
    flags = (*cif).flags as ::core::ffi::c_int;
    cabi = (*cif).abi as ::core::ffi::c_int;
    pabi = (&raw const abi_params as *const abi_params).offset(cabi as isize) as *const abi_params;
    dir = (*pabi).dir;
    rsize = 0 as size_t;
    if rvalue.is_null() {
        match flags {
            X86_RET_FLOAT | X86_RET_DOUBLE | X86_RET_LDOUBLE | X86_RET_STRUCTPOP
            | X86_RET_STRUCTARG => {
                rsize = (*(*cif).rtype).size;
            }
            _ => {
                flags = X86_RET_VOID;
            }
        }
    }
    bytes = (((*cif).bytes as size_t).wrapping_sub(1 as size_t)
        | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
        .wrapping_add(1 as size_t);
    alloca_allocations.push(::std::vec::from_elem(
        0,
        bytes
            .wrapping_add(::core::mem::size_of::<call_frame>() as size_t)
            .wrapping_add(rsize) as usize,
    ));
    stack = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    argp = if dir < 0 as ::core::ffi::c_int {
        stack.offset(bytes as isize)
    } else {
        stack
    };
    frame = stack.offset(bytes as isize) as *mut call_frame;
    if rsize != 0 {
        rvalue = frame.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void;
    }
    (*frame).fn_0 = fn_0;
    (*frame).flags = flags;
    (*frame).rvalue = rvalue;
    (*frame).regs[(*pabi).static_chain as usize] = closure as ::core::ffi::c_uint;
    narg_reg = 0 as ::core::ffi::c_int;
    let mut current_block_24: u64;
    match flags {
        X86_RET_STRUCTARG => {
            if (*pabi).nregs > 0 as ::core::ffi::c_int {
                (*frame).regs[(*pabi).regs[0 as ::core::ffi::c_int as usize] as usize] =
                    rvalue as ::core::ffi::c_uint;
                narg_reg = 1 as ::core::ffi::c_int;
                current_block_24 = 16203760046146113240;
            } else {
                current_block_24 = 1683520436886296249;
            }
        }
        X86_RET_STRUCTPOP => {
            current_block_24 = 1683520436886296249;
        }
        _ => {
            current_block_24 = 16203760046146113240;
        }
    }
    match current_block_24 {
        1683520436886296249 => {
            let ref mut fresh4 = *(argp as *mut *mut ::core::ffi::c_void);
            *fresh4 = rvalue;
            argp =
                argp.offset(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize as isize);
        }
        _ => {}
    }
    arg_types = (*cif).arg_types;
    i = 0 as ::core::ffi::c_int;
    n = (*cif).nargs as ::core::ffi::c_int;
    while i < n {
        let mut ty: *mut ffi_type = *arg_types.offset(i as isize);
        let mut valp: *mut ::core::ffi::c_void = *avalue.offset(i as isize);
        let mut z: size_t = (*ty).size;
        let mut t: ::core::ffi::c_int = (*ty).type_0 as ::core::ffi::c_int;
        if z <= FFI_SIZEOF_ARG as size_t && t != FFI_TYPE_STRUCT {
            let mut val: ffi_arg = extend_basic_type(valp, t);
            if t != FFI_TYPE_FLOAT && narg_reg < (*pabi).nregs {
                let fresh5 = narg_reg;
                narg_reg = narg_reg + 1;
                (*frame).regs[(*pabi).regs[fresh5 as usize] as usize] = val as ::core::ffi::c_uint;
            } else if dir < 0 as ::core::ffi::c_int {
                argp = argp.offset(-(4 as ::core::ffi::c_int as isize));
                *(argp as *mut ffi_arg) = val;
            } else {
                *(argp as *mut ffi_arg) = val;
                argp = argp.offset(4 as ::core::ffi::c_int as isize);
            }
        } else {
            let mut za: size_t = (z.wrapping_sub(1 as size_t)
                | (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_add(1 as size_t);
            let mut align: size_t = FFI_SIZEOF_ARG as size_t;
            if (cabi == FFI_THISCALL as ::core::ffi::c_int
                || cabi == FFI_FASTCALL as ::core::ffi::c_int)
                && (t == FFI_TYPE_SINT64 || t == FFI_TYPE_UINT64 || t == FFI_TYPE_STRUCT)
            {
                narg_reg = 2 as ::core::ffi::c_int;
            }
            if t == FFI_TYPE_STRUCT
                && (*ty).alignment as ::core::ffi::c_int >= 16 as ::core::ffi::c_int
            {
                align = 16 as size_t;
            }
            if dir < 0 as ::core::ffi::c_int {
                argp = argp.offset(-(za as isize));
                memcpy(argp as *mut ::core::ffi::c_void, valp, z);
            } else {
                argp = ((argp as size_t).wrapping_sub(1 as size_t)
                    | align.wrapping_sub(1 as size_t))
                .wrapping_add(1 as size_t) as *mut ::core::ffi::c_char;
                memcpy(argp as *mut ::core::ffi::c_void, valp, z);
                argp = argp.offset(za as isize);
            }
        }
        i += 1;
    }
    ffi_call_i386_from_rust(frame, stack);
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
pub unsafe extern "fastcall" fn ffi_closure_inner(
    mut frame: *mut closure_frame,
    mut stack: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut cif: *mut ffi_cif = (*frame).cif;
    let mut cabi: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut dir: ::core::ffi::c_int = 0;
    let mut narg_reg: ::core::ffi::c_int = 0;
    let mut pabi: *const abi_params = ::core::ptr::null::<abi_params>();
    let mut arg_types: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut argp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rvalue: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut avalue: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    cabi = (*cif).abi as ::core::ffi::c_int;
    flags = (*cif).flags as ::core::ffi::c_int;
    narg_reg = 0 as ::core::ffi::c_int;
    rvalue = &raw mut (*frame).rettemp as *mut ::core::ffi::c_uint as *mut ::core::ffi::c_void;
    pabi = (&raw const abi_params as *const abi_params).offset(cabi as isize) as *const abi_params;
    dir = (*pabi).dir;
    argp = if dir < 0 as ::core::ffi::c_int {
        stack.offset(
            (((*cif).bytes as size_t).wrapping_sub(1 as size_t)
                | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_add(1 as size_t) as isize,
        )
    } else {
        stack
    };
    let mut current_block_12: u64;
    match flags {
        X86_RET_STRUCTARG => {
            if (*pabi).nregs > 0 as ::core::ffi::c_int {
                rvalue = (*frame).regs[(*pabi).regs[0 as ::core::ffi::c_int as usize] as usize]
                    as *mut ::core::ffi::c_void;
                narg_reg = 1 as ::core::ffi::c_int;
                (*frame).rettemp[0 as ::core::ffi::c_int as usize] = rvalue as ::core::ffi::c_uint;
                current_block_12 = 4166486009154926805;
            } else {
                current_block_12 = 7158151284807654568;
            }
        }
        X86_RET_STRUCTPOP => {
            current_block_12 = 7158151284807654568;
        }
        _ => {
            current_block_12 = 4166486009154926805;
        }
    }
    match current_block_12 {
        7158151284807654568 => {
            rvalue = *(argp as *mut *mut ::core::ffi::c_void);
            argp =
                argp.offset(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize as isize);
            (*frame).rettemp[0 as ::core::ffi::c_int as usize] = rvalue as ::core::ffi::c_uint;
        }
        _ => {}
    }
    n = (*cif).nargs as ::core::ffi::c_int;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize).wrapping_mul(n as usize)
            as usize,
    ));
    avalue = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_void;
    arg_types = (*cif).arg_types;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let mut ty: *mut ffi_type = *arg_types.offset(i as isize);
        let mut z: size_t = (*ty).size;
        let mut t: ::core::ffi::c_int = (*ty).type_0 as ::core::ffi::c_int;
        let mut valp: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if z <= FFI_SIZEOF_ARG as size_t && t != FFI_TYPE_STRUCT {
            if t != FFI_TYPE_FLOAT && narg_reg < (*pabi).nregs {
                let fresh6 = narg_reg;
                narg_reg = narg_reg + 1;
                valp = (&raw mut (*frame).regs as *mut ::core::ffi::c_uint).offset(
                    *(&raw const (*pabi).regs as *const ::core::ffi::c_int).offset(fresh6 as isize)
                        as isize,
                ) as *mut ::core::ffi::c_uint as *mut ::core::ffi::c_void;
            } else if dir < 0 as ::core::ffi::c_int {
                argp = argp.offset(-(4 as ::core::ffi::c_int as isize));
                valp = argp as *mut ::core::ffi::c_void;
            } else {
                valp = argp as *mut ::core::ffi::c_void;
                argp = argp.offset(4 as ::core::ffi::c_int as isize);
            }
        } else {
            let mut za: size_t = (z.wrapping_sub(1 as size_t)
                | (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_add(1 as size_t);
            let mut align: size_t = FFI_SIZEOF_ARG as size_t;
            if t == FFI_TYPE_STRUCT
                && (*ty).alignment as ::core::ffi::c_int >= 16 as ::core::ffi::c_int
            {
                align = 16 as size_t;
            }
            if (cabi == FFI_THISCALL as ::core::ffi::c_int
                || cabi == FFI_FASTCALL as ::core::ffi::c_int)
                && (t == FFI_TYPE_SINT64 || t == FFI_TYPE_UINT64 || t == FFI_TYPE_STRUCT)
            {
                narg_reg = 2 as ::core::ffi::c_int;
            }
            if dir < 0 as ::core::ffi::c_int {
                argp = argp.offset(-(za as isize));
                valp = argp as *mut ::core::ffi::c_void;
            } else {
                argp = ((argp as size_t).wrapping_sub(1 as size_t)
                    | align.wrapping_sub(1 as size_t))
                .wrapping_add(1 as size_t) as *mut ::core::ffi::c_char;
                valp = argp as *mut ::core::ffi::c_void;
                argp = argp.offset(za as isize);
            }
        }
        let ref mut fresh7 = *avalue.offset(i as isize);
        *fresh7 = valp;
        i += 1;
    }
    (*frame).fun.expect("non-null function pointer")(cif, rvalue, avalue, (*frame).user_data);
    match cabi {
        5 => {
            return (flags as ::core::ffi::c_uint | (*cif).bytes << X86_RET_POP_SHIFT)
                as ::core::ffi::c_int;
        }
        3 | 4 => {
            return (flags as ::core::ffi::c_uint
                | (*cif)
                    .bytes
                    .wrapping_sub((narg_reg * FFI_SIZEOF_ARG) as ::core::ffi::c_uint)
                    << X86_RET_POP_SHIFT) as ::core::ffi::c_int;
        }
        _ => return flags,
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
    let mut tramp: *mut ::core::ffi::c_char =
        &raw mut (*closure).c2rust_unnamed.tramp as *mut ::core::ffi::c_char;
    let mut dest: Option<unsafe extern "C" fn() -> ()> = None;
    let mut op: ::core::ffi::c_int = 0xb8 as ::core::ffi::c_int;
    match (*cif).abi as ::core::ffi::c_uint {
        1 | 8 => {
            dest = Some(ffi_closure_i386 as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>;
        }
        5 | 3 | 4 | 6 => {
            dest = Some(ffi_closure_STDCALL as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>;
        }
        7 => {
            dest = Some(ffi_closure_REGISTER as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>;
            op = 0x68 as ::core::ffi::c_int;
        }
        _ => return FFI_BAD_ABI,
    }
    if ffi_tramp_is_present(closure as *mut ::core::ffi::c_void) != 0 {
        // Select by ABI instead of comparing Rust function pointers, whose
        // addresses are not guaranteed to be unique after optimization.
        dest = match (*cif).abi as ::core::ffi::c_uint {
            1 | 8 => Some(ffi_closure_i386_alt as unsafe extern "C" fn() -> ()),
            5 | 3 | 4 | 6 => Some(ffi_closure_STDCALL_alt as unsafe extern "C" fn() -> ()),
            7 => Some(ffi_closure_REGISTER_alt as unsafe extern "C" fn() -> ()),
            _ => return FFI_BAD_ABI,
        };
        ffi_tramp_set_parms(
            (*closure).c2rust_unnamed.ftramp,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, *mut ::core::ffi::c_void>(
                dest,
            ),
            closure as *mut ::core::ffi::c_void,
        );
    } else {
        *(tramp as *mut UINT32) = 0xfb1e0ff3 as ::core::ffi::c_uint as UINT32;
        *tramp.offset(4 as ::core::ffi::c_int as isize) = op as ::core::ffi::c_char;
        let ref mut fresh2 =
            *(tramp.offset(5 as ::core::ffi::c_int as isize) as *mut *mut ::core::ffi::c_void);
        *fresh2 = codeloc;
        *tramp.offset(9 as ::core::ffi::c_int as isize) =
            0xe9 as ::core::ffi::c_int as ::core::ffi::c_char;
        *(tramp.offset(10 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uint) =
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, ::core::ffi::c_uint>(
                dest,
            )
            .wrapping_sub((codeloc as ::core::ffi::c_uint).wrapping_add(14 as ::core::ffi::c_uint));
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
    let mut dest: Option<unsafe extern "C" fn() -> ()> = None;
    match (*cif).abi as ::core::ffi::c_uint {
        1 | 8 => {
            dest = Some(ffi_go_closure_ECX as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>;
        }
        3 | 4 => {
            dest = Some(ffi_go_closure_EAX as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>;
        }
        5 | 6 => {
            dest = Some(ffi_go_closure_STDCALL as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>;
        }
        7 | _ => return FFI_BAD_ABI,
    }
    (*closure).tramp = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        *mut ::core::ffi::c_void,
    >(dest);
    (*closure).cif = cif;
    (*closure).fun = fun;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_raw_closure_loc(
    mut closure: *mut ffi_raw_closure,
    mut cif: *mut ffi_cif,
    mut fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut ffi_raw,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    mut user_data: *mut ::core::ffi::c_void,
    mut codeloc: *mut ::core::ffi::c_void,
) -> ffi_status {
    let mut tramp: *mut ::core::ffi::c_char = &raw mut (*closure).tramp as *mut ::core::ffi::c_char;
    let mut dest: Option<unsafe extern "C" fn() -> ()> = None;
    let mut i: ::core::ffi::c_int = 0;
    i = (*cif).nargs.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        match (**(*cif).arg_types.offset(i as isize)).type_0 as ::core::ffi::c_int {
            FFI_TYPE_STRUCT | FFI_TYPE_LONGDOUBLE => return FFI_BAD_TYPEDEF,
            _ => {}
        }
        i -= 1;
    }
    match (*cif).abi as ::core::ffi::c_uint {
        3 => {
            dest = Some(ffi_closure_raw_THISCALL as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>;
        }
        1 => {
            dest = Some(ffi_closure_raw_SYSV as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>;
        }
        _ => return FFI_BAD_ABI,
    }
    *tramp.offset(0 as ::core::ffi::c_int as isize) =
        0xb8 as ::core::ffi::c_int as ::core::ffi::c_char;
    let ref mut fresh3 =
        *(tramp.offset(1 as ::core::ffi::c_int as isize) as *mut *mut ::core::ffi::c_void);
    *fresh3 = codeloc;
    *tramp.offset(5 as ::core::ffi::c_int as isize) =
        0xe9 as ::core::ffi::c_int as ::core::ffi::c_char;
    *(tramp.offset(6 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uint) =
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, ::core::ffi::c_uint>(dest)
            .wrapping_sub((codeloc as ::core::ffi::c_uint).wrapping_add(10 as ::core::ffi::c_uint));
    (*closure).cif = cif;
    (*closure).fun = fun;
    (*closure).user_data = user_data;
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_raw_call(
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut ffi_raw,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut rsize: size_t = 0;
    let mut bytes: size_t = 0;
    let mut frame: *mut call_frame = ::core::ptr::null_mut::<call_frame>();
    let mut stack: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut argp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut arg_types: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut flags: ::core::ffi::c_int = 0;
    let mut cabi: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut narg_reg: ::core::ffi::c_int = 0;
    let mut pabi: *const abi_params = ::core::ptr::null::<abi_params>();
    flags = (*cif).flags as ::core::ffi::c_int;
    cabi = (*cif).abi as ::core::ffi::c_int;
    pabi = (&raw const abi_params as *const abi_params).offset(cabi as isize) as *const abi_params;
    rsize = 0 as size_t;
    if rvalue.is_null() {
        match flags {
            X86_RET_FLOAT | X86_RET_DOUBLE | X86_RET_LDOUBLE | X86_RET_STRUCTPOP
            | X86_RET_STRUCTARG => {
                rsize = (*(*cif).rtype).size;
            }
            _ => {
                flags = X86_RET_VOID;
            }
        }
    }
    bytes = (((*cif).bytes as size_t).wrapping_sub(1 as size_t)
        | (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
        .wrapping_add(1 as size_t);
    alloca_allocations.push(::std::vec::from_elem(
        0,
        bytes
            .wrapping_add(::core::mem::size_of::<call_frame>() as size_t)
            .wrapping_add(rsize)
            .wrapping_add(15 as size_t) as usize,
    ));
    stack = ((alloca_allocations.last_mut().unwrap().as_mut_ptr() as uintptr_t + 15) & !15)
        as *mut ::core::ffi::c_char;
    argp = stack;
    frame = stack.offset(bytes as isize) as *mut call_frame;
    if rsize != 0 {
        rvalue = frame.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void;
    }
    (*frame).fn_0 = fn_0;
    (*frame).flags = flags;
    (*frame).rvalue = rvalue;
    narg_reg = 0 as ::core::ffi::c_int;
    let mut current_block_22: u64;
    match flags {
        X86_RET_STRUCTARG => {
            if (*pabi).nregs > 0 as ::core::ffi::c_int {
                (*frame).regs[(*pabi).regs[0 as ::core::ffi::c_int as usize] as usize] =
                    rvalue as ::core::ffi::c_uint;
                narg_reg = 1 as ::core::ffi::c_int;
                current_block_22 = 10043043949733653460;
            } else {
                current_block_22 = 16693832251570692409;
            }
        }
        X86_RET_STRUCTPOP => {
            current_block_22 = 16693832251570692409;
        }
        _ => {
            current_block_22 = 10043043949733653460;
        }
    }
    match current_block_22 {
        16693832251570692409 => {
            let ref mut fresh0 = *(argp as *mut *mut ::core::ffi::c_void);
            *fresh0 = rvalue;
            argp =
                argp.offset(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize as isize);
            bytes = (bytes as ::core::ffi::c_uint)
                .wrapping_sub(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                    as ::core::ffi::c_uint) as size_t as size_t;
        }
        _ => {}
    }
    arg_types = (*cif).arg_types;
    i = 0 as ::core::ffi::c_int;
    n = (*cif).nargs as ::core::ffi::c_int;
    while narg_reg < (*pabi).nregs && i < n {
        let mut ty: *mut ffi_type = *arg_types.offset(i as isize);
        let mut z: size_t = (*ty).size;
        let mut t: ::core::ffi::c_int = (*ty).type_0 as ::core::ffi::c_int;
        if z <= FFI_SIZEOF_ARG as size_t && t != FFI_TYPE_STRUCT && t != FFI_TYPE_FLOAT {
            let mut val: ffi_arg = extend_basic_type(avalue as *mut ::core::ffi::c_void, t);
            let fresh1 = narg_reg;
            narg_reg = narg_reg + 1;
            (*frame).regs[(*pabi).regs[fresh1 as usize] as usize] = val as ::core::ffi::c_uint;
            z = FFI_SIZEOF_ARG as size_t;
        } else {
            memcpy(
                argp as *mut ::core::ffi::c_void,
                avalue as *const ::core::ffi::c_void,
                z,
            );
            z = (z.wrapping_sub(1 as size_t)
                | (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_add(1 as size_t);
            argp = argp.offset(z as isize);
        }
        avalue = avalue.offset(z as isize);
        bytes = bytes.wrapping_sub(z);
        i += 1;
    }
    if i < n {
        memcpy(
            argp as *mut ::core::ffi::c_void,
            avalue as *const ::core::ffi::c_void,
            bytes,
        );
    }
    ffi_call_i386_from_rust(frame, stack);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_arch(
    mut tramp_size: *mut size_t,
    mut map_size: *mut size_t,
) -> *mut ::core::ffi::c_void {
    extern "C" {
        static mut trampoline_code_table: *mut ::core::ffi::c_void;
    }
    *map_size = X86_TRAMP_MAP_SIZE as size_t;
    *tramp_size = X86_TRAMP_SIZE as size_t;
    return &raw mut trampoline_code_table as *mut ::core::ffi::c_void;
}
