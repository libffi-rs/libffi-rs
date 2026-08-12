use core::ffi::{c_uint, c_ushort, c_void};

const FFI_DEFAULT_ABI: c_uint = 1;
const FFI_OK: c_uint = 0;
const FFI_TYPE_COMPLEX: c_ushort = 15;

#[repr(C)]
struct FfiType {
    size: usize,
    alignment: c_ushort,
    kind: c_ushort,
    elements: *mut *mut FfiType,
}

// Configured SPARC64 ffitarget.h appends nfixedargs to the common ffi_cif.
#[repr(C)]
struct FfiCif {
    abi: c_uint,
    nargs: c_uint,
    arg_types: *mut *mut FfiType,
    rtype: *mut FfiType,
    bytes: c_uint,
    flags: c_uint,
    nfixedargs: c_uint,
}

type ClosureCallback =
    unsafe extern "C" fn(*mut FfiCif, *mut c_void, *mut *mut c_void, *mut c_void);

unsafe extern "C" {
    static mut ffi_type_sint32: FfiType;
    static mut ffi_type_sint64: FfiType;
    static mut ffi_type_double: FfiType;
    static mut ffi_type_longdouble: FfiType;
    static mut ffi_type_complex_float: FfiType;
    static mut ffi_type_complex_double: FfiType;
    static mut ffi_type_complex_longdouble: FfiType;

    fn ffi_get_version_number() -> usize;
    fn ffi_get_default_abi() -> c_uint;
    fn ffi_get_closure_size() -> usize;
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
    fn ffi_closure_alloc(size: usize, code: *mut *mut c_void) -> *mut c_void;
    fn ffi_closure_free(closure: *mut c_void);
    fn ffi_prep_closure_loc(
        closure: *mut c_void,
        cif: *mut FfiCif,
        callback: Option<ClosureCallback>,
        user_data: *mut c_void,
        code: *mut c_void,
    ) -> c_uint;
}

unsafe fn prep(result: *mut FfiType, args: &mut [*mut FfiType]) -> FfiCif {
    let mut cif = core::mem::MaybeUninit::uninit();
    assert_eq!(
        ffi_prep_cif(
            cif.as_mut_ptr(),
            FFI_DEFAULT_ABI,
            args.len() as c_uint,
            result,
            args.as_mut_ptr(),
        ),
        FFI_OK
    );
    cif.assume_init()
}

unsafe extern "C" fn mixed(a: i64, b: f64, c: i32) -> f64 {
    a as f64 + b * 2.0 + c as f64
}

#[test]
fn ffi_call_mixed_register_classes() {
    ffi::force_link();
    unsafe {
        assert_eq!(ffi_get_version_number(), 30502);
        assert_eq!(ffi_get_default_abi(), FFI_DEFAULT_ABI);
        let mut types = [
            core::ptr::addr_of_mut!(ffi_type_sint64),
            core::ptr::addr_of_mut!(ffi_type_double),
            core::ptr::addr_of_mut!(ffi_type_sint32),
        ];
        let mut cif = prep(core::ptr::addr_of_mut!(ffi_type_double), &mut types);
        assert_eq!(cif.nfixedargs, 3);
        let (mut a, mut b, mut c) = (40_i64, 0.75_f64, 2_i32);
        let mut values = [
            core::ptr::from_mut(&mut a).cast(),
            core::ptr::from_mut(&mut b).cast(),
            core::ptr::from_mut(&mut c).cast(),
        ];
        let mut result = 0.0_f64;
        ffi_call(
            &mut cif,
            Some(core::mem::transmute::<
                unsafe extern "C" fn(i64, f64, i32) -> f64,
                unsafe extern "C" fn(),
            >(mixed)),
            core::ptr::from_mut(&mut result).cast(),
            values.as_mut_ptr(),
        );
        assert_eq!(result, 43.5);
    }
}

unsafe extern "C" fn closure_sum(
    _cif: *mut FfiCif,
    result: *mut c_void,
    args: *mut *mut c_void,
    user_data: *mut c_void,
) {
    let a = *(*args.add(0)).cast::<i64>();
    let b = *(*args.add(1)).cast::<i64>();
    let bias = *user_data.cast::<i64>();
    *result.cast::<i64>() = a + b + bias;
}

#[test]
fn executable_closure_calls_back() {
    unsafe {
        let ty = core::ptr::addr_of_mut!(ffi_type_sint64);
        let mut types = [ty, ty];
        let mut cif = prep(ty, &mut types);
        let mut code = core::ptr::null_mut();
        let closure = ffi_closure_alloc(ffi_get_closure_size(), &mut code);
        assert!(!closure.is_null());
        assert!(!code.is_null());
        let mut bias = 9_i64;
        assert_eq!(
            ffi_prep_closure_loc(
                closure,
                &mut cif,
                Some(closure_sum),
                core::ptr::from_mut(&mut bias).cast(),
                code,
            ),
            FFI_OK
        );
        let invoke: unsafe extern "C" fn(i64, i64) -> i64 = core::mem::transmute(code);
        assert_eq!(invoke(11, 22), 42);
        ffi_closure_free(closure);
    }
}

#[test]
fn configured_long_double_and_complex_layouts() {
    unsafe {
        assert_eq!(
            (ffi_type_longdouble.size, ffi_type_longdouble.alignment),
            (16, 16)
        );
        assert_eq!(
            (
                ffi_type_complex_float.size,
                ffi_type_complex_float.alignment
            ),
            (8, 4)
        );
        assert_eq!(
            (
                ffi_type_complex_double.size,
                ffi_type_complex_double.alignment
            ),
            (16, 8)
        );
        assert_eq!(
            (
                ffi_type_complex_longdouble.size,
                ffi_type_complex_longdouble.alignment
            ),
            (32, 16)
        );
        assert_eq!(ffi_type_complex_longdouble.kind, FFI_TYPE_COMPLEX);
    }
}
