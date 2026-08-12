extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ffi_call_LINUX64(
        _: *mut extended_cif,
        _: Option<unsafe extern "C" fn() -> ()>,
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_ulong,
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_long,
    );
    fn ffi_go_closure_linux64();
    fn ffi_prep_types_linux64(_: ffi_abi);
    fn ffi_prep_cif_linux64(_: *mut ffi_cif) -> ffi_status;
    fn ffi_prep_cif_linux64_var(
        _: *mut ffi_cif,
        _: ::core::ffi::c_uint,
        _: ::core::ffi::c_uint,
    ) -> ffi_status;
    fn ffi_prep_closure_loc_linux64(
        _: *mut ffi_closure,
        _: *mut ffi_cif,
        _: Option<
            unsafe extern "C" fn(
                *mut ffi_cif,
                *mut ::core::ffi::c_void,
                *mut *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        _: *mut ::core::ffi::c_void,
        _: *mut ::core::ffi::c_void,
    ) -> ffi_status;
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
pub type float128 = [::core::ffi::c_char; 16];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extended_cif {
    pub cif: *mut ffi_cif,
    pub rvalue: *mut ::core::ffi::c_void,
    pub avalue: *mut *mut ::core::ffi::c_void,
}
pub const FLAG_RETURNS_SMST: C2RustUnnamed_0 = 1;
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
pub const FFI_TYPE_STRUCT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FFI_TYPE_COMPLEX: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PPC_TRAMP_MAP_SHIFT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PPC_TRAMP_MAP_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << PPC_TRAMP_MAP_SHIFT;
pub const PPC_TRAMP_SIZE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_types(mut abi: ffi_abi) {
    ffi_prep_types_linux64(abi);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep(mut cif: *mut ffi_cif) -> ffi_status {
    return ffi_prep_cif_linux64(cif);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_machdep_var(
    mut cif: *mut ffi_cif,
    mut nfixedargs: ::core::ffi::c_uint,
    mut ntotalargs: ::core::ffi::c_uint,
) -> ffi_status {
    return ffi_prep_cif_linux64_var(cif, nfixedargs, ntotalargs);
}
unsafe extern "C" fn ffi_call_int(
    cif: *mut ffi_cif,
    fn_0: Option<unsafe extern "C" fn() -> ()>,
    rvalue: *mut ::core::ffi::c_void,
    avalue: *mut *mut ::core::ffi::c_void,
    closure: *mut ::core::ffi::c_void,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut smst_buffer: [float128; 8] = [[0; 16]; 8];
    let mut ecif = extended_cif {
        cif,
        rvalue,
        avalue,
    };
    if (*cif).flags & FLAG_RETURNS_SMST != 0 {
        ecif.rvalue = (&raw mut smst_buffer).cast();
    } else if rvalue.is_null() && (*(*cif).rtype).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT {
        alloca_allocations.push(vec![0; (*(*cif).rtype).size]);
        ecif.rvalue = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast();
    }
    ffi_call_LINUX64(
        &raw mut ecif,
        fn_0,
        ecif.rvalue,
        (*cif).flags as ::core::ffi::c_ulong,
        closure,
        -((*cif).bytes as ::core::ffi::c_long),
    );
    let bounce: *mut ::core::ffi::c_void = (&raw mut smst_buffer).cast();
    if !rvalue.is_null() && ecif.rvalue == bounce {
        let rsize = (*(*cif).rtype).size;
        if (*(*cif).rtype).type_0 as ::core::ffi::c_int == FFI_TYPE_COMPLEX
            && (*cif).flags & (FLAG_RETURNS_FP | FLAG_RETURNS_VEC) == 0
        {
            let hsize = (**(*(*cif).rtype).elements).size;
            let off = 8usize - hsize;
            memcpy(rvalue, (bounce as *const u8).add(off).cast(), hsize);
            memcpy(
                (rvalue as *mut u8).add(hsize).cast(),
                (bounce as *const u8).add(8 + off).cast(),
                hsize,
            );
        } else if rsize <= 8 && (*cif).flags & FLAG_RETURNS_FP == 0 {
            memcpy(rvalue, (bounce as *const u8).add(8 - rsize).cast(), rsize);
        } else {
            memcpy(rvalue, bounce, rsize);
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
    return ffi_prep_closure_loc_linux64(closure, cif, fun, user_data, codeloc);
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
    (*closure).tramp = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        *mut ::core::ffi::c_void,
    >(Some(ffi_go_closure_linux64 as unsafe extern "C" fn() -> ()));
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
    *tramp_size = PPC_TRAMP_SIZE as size_t;
    *map_size = PPC_TRAMP_MAP_SIZE as size_t;
    return *(trampoline_code_table as *mut *mut ::core::ffi::c_void);
}
