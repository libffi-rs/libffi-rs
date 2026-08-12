use core::ffi::{c_uint, c_ushort, c_void};

pub type ffi_abi = c_uint;
pub type ffi_status = c_uint;
pub const FFI_OK: ffi_status = 0;
pub const FFI_SYSV: ffi_abi = 1;
pub const FFI_VFP: ffi_abi = 2;
pub const FFI_DEFAULT_ABI: ffi_abi = FFI_VFP;

#[repr(C)]
pub struct ffi_type {
    pub size: c_uint,
    pub alignment: c_ushort,
    pub type_: c_ushort,
    pub elements: *mut *mut ffi_type,
}

#[repr(C)]
pub struct ffi_cif {
    pub abi: ffi_abi,
    pub nargs: c_uint,
    pub arg_types: *mut *mut ffi_type,
    pub rtype: *mut ffi_type,
    pub bytes: c_uint,
    pub flags: c_uint,
    pub vfp_used: i32,
    pub vfp_reg_free: c_ushort,
    pub vfp_nargs: c_ushort,
    pub vfp_args: [i8; 16],
}

pub type ffi_fn = unsafe extern "C" fn();
pub type ffi_closure_fun =
    unsafe extern "C" fn(*mut ffi_cif, *mut c_void, *mut *mut c_void, *mut c_void);

unsafe extern "C" {
    pub static mut ffi_type_void: ffi_type;
    pub static mut ffi_type_uint8: ffi_type;
    pub static mut ffi_type_sint8: ffi_type;
    pub static mut ffi_type_uint16: ffi_type;
    pub static mut ffi_type_sint16: ffi_type;
    pub static mut ffi_type_uint32: ffi_type;
    pub static mut ffi_type_sint32: ffi_type;
    pub static mut ffi_type_uint64: ffi_type;
    pub static mut ffi_type_sint64: ffi_type;
    pub static mut ffi_type_pointer: ffi_type;
    pub static mut ffi_type_float: ffi_type;
    pub static mut ffi_type_double: ffi_type;
    pub static mut ffi_type_longdouble: ffi_type;
    pub static mut ffi_type_complex_float: ffi_type;
    pub static mut ffi_type_complex_double: ffi_type;
    pub static mut ffi_type_complex_longdouble: ffi_type;

    pub fn ffi_get_default_abi() -> ffi_abi;
    pub fn ffi_get_closure_size() -> c_uint;
    pub fn ffi_prep_cif(
        cif: *mut ffi_cif,
        abi: ffi_abi,
        nargs: c_uint,
        rtype: *mut ffi_type,
        atypes: *mut *mut ffi_type,
    ) -> ffi_status;
    pub fn ffi_call(
        cif: *mut ffi_cif,
        function: Option<ffi_fn>,
        result: *mut c_void,
        args: *mut *mut c_void,
    );
    pub fn ffi_closure_alloc(size: c_uint, code: *mut *mut c_void) -> *mut c_void;
    pub fn ffi_closure_free(closure: *mut c_void);
    pub fn ffi_prep_closure_loc(
        closure: *mut c_void,
        cif: *mut ffi_cif,
        fun: Option<ffi_closure_fun>,
        user_data: *mut c_void,
        codeloc: *mut c_void,
    ) -> ffi_status;
}
