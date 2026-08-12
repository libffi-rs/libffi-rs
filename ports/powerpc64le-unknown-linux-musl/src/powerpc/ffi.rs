extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ffi_go_closure_linux64();
    fn ffi_call_LINUX64(
        _: *mut extended_cif,
        _: Option<unsafe extern "C" fn() -> ()>,
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_long,
    );
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
pub const FFI_DEFAULT_ABI: ffi_abi = 8;
pub const FFI_SYSV_LONG_DOUBLE_128: ffi_abi = 16;
pub const FFI_SYSV_IBM_LONG_DOUBLE: ffi_abi = 4;
pub const FFI_SYSV_STRUCT_RET: ffi_abi = 2;
pub const FFI_SYSV_SOFT_FLOAT: ffi_abi = 1;
pub const FFI_SYSV: ffi_abi = 8;
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
    pub tramp: [::core::ffi::c_char; 40],
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
pub struct extended_cif {
    pub cif: *mut ffi_cif,
    pub rvalue: *mut ::core::ffi::c_void,
    pub avalue: *mut *mut ::core::ffi::c_void,
}
#[repr(align(16))]
struct BounceBuffer([::core::ffi::c_uchar; 128]);

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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PPC_TRAMP_MAP_SHIFT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PPC_TRAMP_MAP_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << PPC_TRAMP_MAP_SHIFT;
pub const PPC_TRAMP_SIZE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_types(mut abi: ffi_abi) {
    crate::types::initialize_complex_types();
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
    mut cif: *mut ffi_cif,
    mut fn_0: Option<unsafe extern "C" fn() -> ()>,
    mut rvalue: *mut ::core::ffi::c_void,
    mut avalue: *mut *mut ::core::ffi::c_void,
    mut closure: *mut ::core::ffi::c_void,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut smst_buffer = BounceBuffer([0; 128]);
    let mut ecif: extended_cif = extended_cif {
        cif: ::core::ptr::null_mut::<ffi_cif>(),
        rvalue: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        avalue: ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    };
    ecif.cif = cif;
    ecif.avalue = avalue;
    ecif.rvalue = rvalue;
    if (*cif).flags & FLAG_RETURNS_SMST as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        ecif.rvalue = smst_buffer.0.as_mut_ptr() as *mut ::core::ffi::c_void;
    } else if rvalue.is_null() && (*(*cif).rtype).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT {
        alloca_allocations.push(::std::vec::from_elem(0, (*(*cif).rtype).size as usize));
        ecif.rvalue = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast();
    }
    ffi_call_LINUX64(
        &raw mut ecif,
        fn_0,
        ecif.rvalue,
        (*cif).flags,
        closure,
        -((*cif).bytes as ::core::ffi::c_long),
    );
    if !rvalue.is_null() && ecif.rvalue == smst_buffer.0.as_mut_ptr() as *mut ::core::ffi::c_void {
        let mut rsize: ::core::ffi::c_uint = (*(*cif).rtype).size as ::core::ffi::c_uint;
        memcpy(
            rvalue,
            smst_buffer.0.as_mut_ptr() as *const ::core::ffi::c_void,
            rsize as size_t,
        );
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
    return &raw mut trampoline_code_table as *mut ::core::ffi::c_void;
}
