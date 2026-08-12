extern "C" {
    static mut ffi_type_sint32: ffi_type;
    static mut ffi_type_float: ffi_type;
    fn ffi_prep_closure_loc(
        _: *mut ffi_closure,
        _: *mut ffi_cif,
        fun: Option<
            unsafe extern "C" fn(
                *mut ffi_cif,
                *mut ::core::ffi::c_void,
                *mut *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        user_data: *mut ::core::ffi::c_void,
        codeloc: *mut ::core::ffi::c_void,
    ) -> ffi_status;
    fn ffi_prep_cif_machdep(cif: *mut ffi_cif) -> ffi_status;
    fn ffi_prep_cif_machdep_var(
        cif: *mut ffi_cif,
        nfixedargs: ::core::ffi::c_uint,
        ntotalargs: ::core::ffi::c_uint,
    ) -> ffi_status;
    fn ffi_prep_types(abi: ffi_abi);
    fn abort() -> !;
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
pub const FFI_TYPE_STRUCT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FFI_TYPE_COMPLEX: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn initialize_aggregate(
    mut arg: *mut ffi_type,
    mut offsets: *mut size_t,
) -> ffi_status {
    let mut ptr: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    if ((arg.is_null() || (*arg).elements.is_null()) as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        return FFI_BAD_TYPEDEF;
    }
    (*arg).size = 0 as size_t;
    (*arg).alignment = 0 as ::core::ffi::c_ushort;
    ptr = (*arg).elements.offset(0 as ::core::ffi::c_int as isize) as *mut *mut _ffi_type
        as *mut *mut ffi_type;
    if ((ptr == ::core::ptr::null_mut::<*mut ffi_type>()) as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        return FFI_BAD_TYPEDEF;
    }
    while !(*ptr).is_null() {
        if (((**ptr).size == 0 as size_t
            && initialize_aggregate(*ptr, ::core::ptr::null_mut::<size_t>()) as ::core::ffi::c_uint
                != FFI_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            return FFI_BAD_TYPEDEF;
        }
        (*arg).size = ((*arg).size.wrapping_sub(1 as size_t)
            | ((**ptr).alignment as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(1 as size_t);
        if !offsets.is_null() {
            let fresh0 = offsets;
            offsets = offsets.offset(1);
            *fresh0 = (*arg).size;
        }
        (*arg).size = (*arg).size.wrapping_add((**ptr).size);
        (*arg).alignment =
            (if (*arg).alignment as ::core::ffi::c_int > (**ptr).alignment as ::core::ffi::c_int {
                (*arg).alignment as ::core::ffi::c_int
            } else {
                (**ptr).alignment as ::core::ffi::c_int
            }) as ::core::ffi::c_ushort;
        ptr = ptr.offset(1);
    }
    (*arg).size = ((*arg).size.wrapping_sub(1 as size_t)
        | ((*arg).alignment as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
        .wrapping_add(1 as size_t);
    if (*arg).size == 0 as size_t {
        return FFI_BAD_TYPEDEF;
    } else {
        return FFI_OK;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_core(
    mut cif: *mut ffi_cif,
    mut abi: ffi_abi,
    mut isvariadic: ::core::ffi::c_uint,
    mut nfixedargs: ::core::ffi::c_uint,
    mut ntotalargs: ::core::ffi::c_uint,
    mut rtype: *mut ffi_type,
    mut atypes: *mut *mut ffi_type,
) -> ffi_status {
    let mut bytes: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint = 0;
    let mut ptr: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    if !(abi as ::core::ffi::c_uint > FFI_FIRST_ABI as ::core::ffi::c_int as ::core::ffi::c_uint
        && (abi as ::core::ffi::c_uint) < FFI_LAST_ABI as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        return FFI_BAD_ABI;
    }
    (*cif).abi = abi;
    (*cif).arg_types = atypes;
    (*cif).nargs = ntotalargs;
    (*cif).rtype = rtype;
    (*cif).flags = 0 as ::core::ffi::c_uint;
    ffi_prep_types(abi);
    if (*(*cif).rtype).size == 0 as size_t
        && initialize_aggregate((*cif).rtype, ::core::ptr::null_mut::<size_t>())
            as ::core::ffi::c_uint
            != FFI_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return FFI_BAD_TYPEDEF;
    }
    if (*rtype).type_0 as ::core::ffi::c_int == FFI_TYPE_COMPLEX {
        abort();
    }
    if (*(*cif).rtype).type_0 as ::core::ffi::c_int == FFI_TYPE_STRUCT {
        bytes = (::core::mem::size_of::<*mut ::core::ffi::c_void>().wrapping_sub(1 as size_t)
            | (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(1 as size_t) as ::core::ffi::c_uint;
    }
    ptr = (*cif).arg_types;
    i = (*cif).nargs;
    while i > 0 as ::core::ffi::c_uint {
        if (**ptr).size == 0 as size_t
            && initialize_aggregate(*ptr, ::core::ptr::null_mut::<size_t>()) as ::core::ffi::c_uint
                != FFI_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return FFI_BAD_TYPEDEF;
        }
        if (**ptr).type_0 as ::core::ffi::c_int == FFI_TYPE_COMPLEX {
            abort();
        }
        if ((**ptr).alignment as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_uint
            & bytes
            != 0
        {
            bytes = ((bytes as size_t).wrapping_sub(1 as size_t)
                | ((**ptr).alignment as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_add(1 as size_t) as ::core::ffi::c_uint;
        }
        bytes = bytes.wrapping_add(
            ((**ptr).size.wrapping_sub(1 as size_t)
                | (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_add(1 as size_t) as ::core::ffi::c_uint,
        );
        i = i.wrapping_sub(1);
        ptr = ptr.offset(1);
    }
    (*cif).bytes = bytes;
    if isvariadic != 0 {
        return ffi_prep_cif_machdep_var(cif, nfixedargs, ntotalargs);
    }
    return ffi_prep_cif_machdep(cif);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif(
    mut cif: *mut ffi_cif,
    mut abi: ffi_abi,
    mut nargs: ::core::ffi::c_uint,
    mut rtype: *mut ffi_type,
    mut atypes: *mut *mut ffi_type,
) -> ffi_status {
    return ffi_prep_cif_core(
        cif,
        abi,
        0 as ::core::ffi::c_uint,
        nargs,
        nargs,
        rtype,
        atypes,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_cif_var(
    mut cif: *mut ffi_cif,
    mut abi: ffi_abi,
    mut nfixedargs: ::core::ffi::c_uint,
    mut ntotalargs: ::core::ffi::c_uint,
    mut rtype: *mut ffi_type,
    mut atypes: *mut *mut ffi_type,
) -> ffi_status {
    let mut rc: ffi_status = FFI_OK;
    let mut int_size: size_t = ffi_type_sint32.size;
    let mut i: ::core::ffi::c_uint = 0;
    rc = ffi_prep_cif_core(
        cif,
        abi,
        1 as ::core::ffi::c_uint,
        nfixedargs,
        ntotalargs,
        rtype,
        atypes,
    );
    if rc as ::core::ffi::c_uint != FFI_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    i = nfixedargs;
    while i < ntotalargs {
        let mut arg_type: *mut ffi_type = *atypes.offset(i as isize);
        if arg_type == &raw mut ffi_type_float
            || (*arg_type).type_0 as ::core::ffi::c_int != FFI_TYPE_STRUCT
                && (*arg_type).type_0 as ::core::ffi::c_int != FFI_TYPE_COMPLEX
                && (*arg_type).size < int_size
        {
            return FFI_BAD_ARGTYPE;
        }
        i = i.wrapping_add(1);
    }
    return FFI_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_prep_closure(
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
) -> ffi_status {
    return ffi_prep_closure_loc(
        closure,
        cif,
        fun,
        user_data,
        closure as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ffi_get_struct_offsets(
    mut abi: ffi_abi,
    mut struct_type: *mut ffi_type,
    mut offsets: *mut size_t,
) -> ffi_status {
    if !(abi as ::core::ffi::c_uint > FFI_FIRST_ABI as ::core::ffi::c_int as ::core::ffi::c_uint
        && (abi as ::core::ffi::c_uint) < FFI_LAST_ABI as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        return FFI_BAD_ABI;
    }
    if (*struct_type).type_0 as ::core::ffi::c_int != FFI_TYPE_STRUCT {
        return FFI_BAD_TYPEDEF;
    }
    ffi_prep_types(abi);
    return initialize_aggregate(struct_type, offsets);
}
