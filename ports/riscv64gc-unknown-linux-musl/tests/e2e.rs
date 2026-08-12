use core::ffi::{c_uint, c_ushort, c_void};
use libffi_riscv64_musl_port as port;

const FFI_DEFAULT_ABI: c_uint = 1;
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
    riscv_nfixedargs: c_uint,
    riscv_unused: c_uint,
}

type ClosureCallback =
    unsafe extern "C" fn(*mut FfiCif, *mut c_void, *mut *mut c_void, *mut c_void);

unsafe extern "C" {
    static mut ffi_type_sint32: FfiType;
    static mut ffi_type_sint64: FfiType;
    static mut ffi_type_double: FfiType;
    static mut ffi_type_longdouble: FfiType;
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

unsafe fn prep(arg_types: &mut [*mut FfiType]) -> FfiCif {
    let mut cif = core::mem::MaybeUninit::uninit();
    assert_eq!(
        ffi_prep_cif(
            cif.as_mut_ptr(),
            FFI_DEFAULT_ABI,
            arg_types.len() as c_uint,
            &raw mut ffi_type_sint32,
            arg_types.as_mut_ptr()
        ),
        FFI_OK
    );
    cif.assume_init()
}

unsafe extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn call_and_configured_layout() {
    unsafe {
        assert_eq!(port::src::types::ffi_get_default_abi(), FFI_DEFAULT_ABI);
        assert_eq!(
            (ffi_type_longdouble.size, ffi_type_longdouble.alignment),
            (16, 16)
        );
        assert_eq!((ffi_type_double.size, ffi_type_double.alignment), (8, 8));
        eprintln!("call: statics ok");
        let ty = &raw mut ffi_type_sint32;
        let mut types = [ty, ty];
        let mut cif = prep(&mut types);
        eprintln!(
            "call: cif prepared bytes={} flags={} nfixed={}",
            cif.bytes, cif.flags, cif.riscv_nfixedargs
        );
        let (mut a, mut b, mut result) = (20_i32, 22_i32, 0_i32);
        let mut args = [(&raw mut a).cast(), (&raw mut b).cast()];
        let function = core::mem::transmute::<
            unsafe extern "C" fn(i32, i32) -> i32,
            unsafe extern "C" fn(),
        >(add);
        eprintln!("call: invoking fn={:p}", add as *const ());
        ffi_call(
            &raw mut cif,
            Some(function),
            (&raw mut result).cast(),
            args.as_mut_ptr(),
        );
        eprintln!("call: returned {result}");
        assert_eq!(result, 42);
    }
}

unsafe extern "C" fn many_i64(
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    f: i64,
    g: i64,
    h: i64,
    i: i64,
    j: i64,
) -> i64 {
    a + b + c + d + e + f + g + h + i + j
}

#[test]
fn call_with_stacked_integer_arguments() {
    unsafe {
        let ty = &raw mut ffi_type_sint64;
        let mut types = [ty; 10];
        let mut cif = core::mem::MaybeUninit::<FfiCif>::uninit();
        assert_eq!(
            ffi_prep_cif(
                cif.as_mut_ptr(),
                FFI_DEFAULT_ABI,
                10,
                ty,
                types.as_mut_ptr()
            ),
            FFI_OK
        );
        let mut cif = cif.assume_init();
        let mut values = [1_i64, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut args = values.each_mut().map(|v| (v as *mut i64).cast::<c_void>());
        let mut result = 0_i64;
        let function = core::mem::transmute::<
            unsafe extern "C" fn(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) -> i64,
            unsafe extern "C" fn(),
        >(many_i64);
        ffi_call(
            &raw mut cif,
            Some(function),
            (&raw mut result).cast(),
            args.as_mut_ptr(),
        );
        assert_eq!(result, 55);
    }
}

unsafe extern "C" fn closure_add(
    _cif: *mut FfiCif,
    result: *mut c_void,
    args: *mut *mut c_void,
    user_data: *mut c_void,
) {
    let bias = *(user_data.cast::<i32>());
    let a = *(*args.add(0)).cast::<i32>();
    let b = *(*args.add(1)).cast::<i32>();
    *result.cast::<i32>() = a + b + bias;
}

#[test]
fn executable_closure() {
    unsafe {
        let ty = &raw mut ffi_type_sint32;
        let mut types = [ty, ty];
        let mut cif = prep(&mut types);
        let mut code = core::ptr::null_mut();
        let closure = ffi_closure_alloc(64, &raw mut code);
        assert!(!closure.is_null() && !code.is_null());
        let mut bias = 7_i32;
        assert_eq!(
            ffi_prep_closure_loc(
                closure,
                &raw mut cif,
                Some(closure_add),
                (&raw mut bias).cast(),
                code
            ),
            FFI_OK
        );
        let function =
            core::mem::transmute::<*mut c_void, unsafe extern "C" fn(i32, i32) -> i32>(code);
        assert_eq!(function(10, 25), 42);
        ffi_closure_free(closure);
    }
}
