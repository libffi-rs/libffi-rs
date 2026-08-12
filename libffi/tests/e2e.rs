use core::ffi::{c_uint, c_ushort, c_void};

#[repr(C)]
struct FfiType {
    size: usize,
    alignment: c_ushort,
    kind: c_ushort,
    elements: *mut *mut FfiType,
}

#[repr(C)]
struct FfiCif {
    abi: c_uint,
    nargs: c_uint,
    arg_types: *mut *mut FfiType,
    rtype: *mut FfiType,
    bytes: c_uint,
    flags: c_uint,
    aarch64_nfixedargs: c_uint,
}

extern "C" {
    static mut ffi_type_sint32: FfiType;
    fn ffi_prep_cif(
        cif: *mut FfiCif,
        abi: c_uint,
        nargs: c_uint,
        rtype: *mut FfiType,
        atypes: *mut *mut FfiType,
    ) -> c_uint;
    fn ffi_call(
        cif: *mut FfiCif,
        function: Option<unsafe extern "C" fn()>,
        result: *mut c_void,
        args: *mut *mut c_void,
    );
}

unsafe extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn calls_function_through_ffi() {
    unsafe {
        assert_eq!(libffi::src::types::ffi_get_version_number(), 30502);
        let int_type = core::ptr::addr_of_mut!(ffi_type_sint32);
        let mut arg_types = [int_type, int_type];
        let mut cif = core::mem::MaybeUninit::<FfiCif>::uninit();
        assert_eq!(
            ffi_prep_cif(cif.as_mut_ptr(), 1, 2, int_type, arg_types.as_mut_ptr()),
            0
        );
        let (mut a, mut b, mut result) = (20_i32, 22_i32, 0_i32);
        let mut args = [
            core::ptr::addr_of_mut!(a).cast(),
            core::ptr::addr_of_mut!(b).cast(),
        ];
        ffi_call(
            cif.as_mut_ptr(),
            Some(core::mem::transmute::<
                unsafe extern "C" fn(i32, i32) -> i32,
                unsafe extern "C" fn(),
            >(add)),
            core::ptr::addr_of_mut!(result).cast(),
            args.as_mut_ptr(),
        );
        assert_eq!(result, 42);
    }
}
