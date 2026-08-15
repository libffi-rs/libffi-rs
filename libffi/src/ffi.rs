#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ffi_type {
    pub size: usize,
    pub alignment: core::ffi::c_ushort,
    pub type_0: core::ffi::c_ushort,
    pub elements: *mut *mut _ffi_type,
}

pub type ffi_type = _ffi_type;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_cif {
    pub abi: core::ffi::c_uint,
    pub nargs: core::ffi::c_uint,
    pub arg_types: *mut *mut ffi_type,
    pub rtype: *mut ffi_type,
    pub bytes: core::ffi::c_uint,
    pub flags: core::ffi::c_uint,
    pub aarch64_nfixedargs: core::ffi::c_uint,
}
