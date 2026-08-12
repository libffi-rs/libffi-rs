use core::ffi::{c_uint, c_ushort, c_void};

use libffi_c2rust_armv7_gnueabihf as _;

const FFI_DEFAULT_ABI: c_uint = 2;
const FFI_OK: c_uint = 0;

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
    vfp_used: core::ffi::c_int,
    vfp_reg_free: c_ushort,
    vfp_nargs: c_ushort,
    vfp_args: [core::ffi::c_schar; 16],
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
    fn ffi_get_closure_size() -> usize;
    fn ffi_closure_alloc(size: usize, code: *mut *mut c_void) -> *mut c_void;
    fn ffi_prep_closure_loc(
        closure: *mut c_void,
        cif: *mut FfiCif,
        callback: Option<
            unsafe extern "C" fn(*mut FfiCif, *mut c_void, *mut *mut c_void, *mut c_void),
        >,
        user_data: *mut c_void,
        code: *mut c_void,
    ) -> c_uint;
    fn ffi_closure_free(closure: *mut c_void);
}

unsafe extern "C" fn add_callback(
    _cif: *mut FfiCif,
    result: *mut c_void,
    args: *mut *mut c_void,
    user_data: *mut c_void,
) {
    let lhs = *(*args.add(0)).cast::<i32>();
    let rhs = *(*args.add(1)).cast::<i32>();
    let bias = *user_data.cast::<i32>();
    *result.cast::<i32>() = lhs + rhs + bias;
}

#[test]
fn closure_trampoline_calls_rust_callback() {
    unsafe {
        let sint32 = core::ptr::addr_of_mut!(ffi_type_sint32);
        let mut arg_types = [sint32, sint32];
        let mut cif = core::mem::MaybeUninit::<FfiCif>::uninit();
        assert_eq!(
            ffi_prep_cif(
                cif.as_mut_ptr(),
                FFI_DEFAULT_ABI,
                2,
                sint32,
                arg_types.as_mut_ptr(),
            ),
            FFI_OK
        );
        let mut cif = cif.assume_init();

        let mut code = core::ptr::null_mut();
        let closure = ffi_closure_alloc(ffi_get_closure_size(), &mut code);
        assert!(!closure.is_null());
        assert!(!code.is_null());
        let mut bias = 7_i32;
        assert_eq!(
            ffi_prep_closure_loc(
                closure,
                &mut cif,
                Some(add_callback),
                core::ptr::from_mut(&mut bias).cast(),
                code,
            ),
            FFI_OK
        );

        let callable: unsafe extern "C" fn(i32, i32) -> i32 = core::mem::transmute(code);
        assert_eq!(callable(11, 24), 42);
        ffi_closure_free(closure);
    }
}
