use core::ffi::{c_uint, c_void};
use core::{mem, ptr};

const FFI_OK: c_uint = 0;
const FFI_DEFAULT_ABI: c_uint = 3; // LP64D from configured ffitarget.h

#[repr(C)]
struct FfiType {
    size: usize,
    alignment: u16,
    type_: u16,
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
    loongarch_nfixedargs: c_uint,
    loongarch_unused: c_uint,
}

type ClosureCallback =
    unsafe extern "C" fn(*mut FfiCif, *mut c_void, *mut *mut c_void, *mut c_void);

unsafe extern "C" {
    static mut ffi_type_sint64: FfiType;
    static mut ffi_type_longdouble: FfiType;
    static mut ffi_type_complex_longdouble: FfiType;

    fn ffi_prep_cif(
        cif: *mut FfiCif,
        abi: c_uint,
        nargs: c_uint,
        rtype: *mut FfiType,
        atypes: *mut *mut FfiType,
    ) -> c_uint;
    fn ffi_call(
        cif: *mut FfiCif,
        fun: Option<unsafe extern "C" fn()>,
        rvalue: *mut c_void,
        avalue: *mut *mut c_void,
    );
    fn ffi_get_closure_size() -> usize;
    fn ffi_closure_alloc(size: usize, code: *mut *mut c_void) -> *mut c_void;
    fn ffi_closure_free(closure: *mut c_void);
    fn ffi_prep_closure_loc(
        closure: *mut c_void,
        cif: *mut FfiCif,
        fun: Option<ClosureCallback>,
        user_data: *mut c_void,
        codeloc: *mut c_void,
    ) -> c_uint;
}

unsafe extern "C" fn add(a: i64, b: i64) -> i64 {
    a + b
}

unsafe extern "C" fn sum10(
    a0: i64,
    a1: i64,
    a2: i64,
    a3: i64,
    a4: i64,
    a5: i64,
    a6: i64,
    a7: i64,
    a8: i64,
    a9: i64,
) -> i64 {
    a0 + a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9
}

unsafe extern "C" fn closure_add(
    _cif: *mut FfiCif,
    result: *mut c_void,
    args: *mut *mut c_void,
    user_data: *mut c_void,
) {
    let a = *(*args.add(0)).cast::<i64>();
    let b = *(*args.add(1)).cast::<i64>();
    *result.cast::<i64>() = a + b + *user_data.cast::<i64>();
}

#[test]
fn configured_layout_and_long_double() {
    unsafe {
        assert_eq!(ffi::types::ffi_get_default_abi(), FFI_DEFAULT_ABI);
        assert_eq!(mem::size_of::<FfiCif>(), 40);
        assert_eq!(ffi_get_closure_size(), 48);
        let longdouble = (&raw const ffi_type_longdouble).read();
        let complex_longdouble = (&raw const ffi_type_complex_longdouble).read();
        assert_eq!(longdouble.size, 16);
        assert_eq!(longdouble.alignment, 16);
        assert_eq!(complex_longdouble.size, 32);
        assert_eq!(complex_longdouble.alignment, 16);
    }
}

#[test]
fn ffi_call_two_integer_arguments() {
    unsafe {
        let mut cif: FfiCif = mem::zeroed();
        let mut types = [&raw mut ffi_type_sint64, &raw mut ffi_type_sint64];
        assert_eq!(
            ffi_prep_cif(
                &mut cif,
                FFI_DEFAULT_ABI,
                2,
                &raw mut ffi_type_sint64,
                types.as_mut_ptr()
            ),
            FFI_OK
        );
        let mut a = 19_i64;
        let mut b = 23_i64;
        let mut values = [ptr::from_mut(&mut a).cast(), ptr::from_mut(&mut b).cast()];
        let mut result = 0_i64;
        let erased =
            mem::transmute::<unsafe extern "C" fn(i64, i64) -> i64, unsafe extern "C" fn()>(add);
        ffi_call(
            &mut cif,
            Some(erased),
            ptr::from_mut(&mut result).cast(),
            values.as_mut_ptr(),
        );
        assert_eq!(result, 42);
    }
}

#[test]
fn executable_closure_calls_rust_callback() {
    unsafe {
        let mut cif: FfiCif = mem::zeroed();
        let mut types = [&raw mut ffi_type_sint64, &raw mut ffi_type_sint64];
        assert_eq!(
            ffi_prep_cif(
                &mut cif,
                FFI_DEFAULT_ABI,
                2,
                &raw mut ffi_type_sint64,
                types.as_mut_ptr()
            ),
            FFI_OK
        );
        let mut code = ptr::null_mut();
        let closure = ffi_closure_alloc(ffi_get_closure_size(), &mut code);
        assert!(!closure.is_null());
        assert!(!code.is_null());
        let mut bias = 7_i64;
        assert_eq!(
            ffi_prep_closure_loc(
                closure,
                &mut cif,
                Some(closure_add),
                ptr::from_mut(&mut bias).cast(),
                code
            ),
            FFI_OK
        );
        let callable = mem::transmute::<*mut c_void, unsafe extern "C" fn(i64, i64) -> i64>(code);
        assert_eq!(callable(10, 25), 42);
        ffi_closure_free(closure);
    }
}

#[test]
fn ffi_call_uses_stack_arguments() {
    unsafe {
        let mut cif: FfiCif = mem::zeroed();
        let mut types = [&raw mut ffi_type_sint64; 10];
        assert_eq!(
            ffi_prep_cif(
                &mut cif,
                FFI_DEFAULT_ABI,
                10,
                &raw mut ffi_type_sint64,
                types.as_mut_ptr()
            ),
            FFI_OK
        );
        assert!(cif.bytes >= 16);
        let mut input = [1_i64, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut values: [*mut c_void; 10] =
            core::array::from_fn(|i| ptr::from_mut(&mut input[i]).cast());
        let mut result = 0_i64;
        let erased = mem::transmute::<
            unsafe extern "C" fn(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) -> i64,
            unsafe extern "C" fn(),
        >(sum10);
        ffi_call(
            &mut cif,
            Some(erased),
            ptr::from_mut(&mut result).cast(),
            values.as_mut_ptr(),
        );
        assert_eq!(result, 55);
    }
}
