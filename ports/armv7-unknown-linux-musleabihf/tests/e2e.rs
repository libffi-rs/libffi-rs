use core::ffi::c_void;
use libffi_c2rust_armv7_musl::api::*;

unsafe extern "C" fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

unsafe extern "C" fn closure_add(
    _cif: *mut ffi_cif,
    result: *mut c_void,
    args: *mut *mut c_void,
    user_data: *mut c_void,
) {
    let a = *(*args.add(0)).cast::<i32>();
    let b = *(*args.add(1)).cast::<i32>();
    let bias = *(user_data.cast::<i32>());
    *result.cast::<i32>() = a + b + bias;
}

unsafe fn int_cif() -> (ffi_cif, Box<[*mut ffi_type; 2]>) {
    let mut cif: ffi_cif = core::mem::zeroed();
    let mut args = Box::new([&raw mut ffi_type_sint32, &raw mut ffi_type_sint32]);
    assert_eq!(ffi_get_default_abi(), FFI_DEFAULT_ABI);
    assert_eq!(
        ffi_prep_cif(
            &mut cif,
            FFI_DEFAULT_ABI,
            args.len() as u32,
            &raw mut ffi_type_sint32,
            args.as_mut_ptr(),
        ),
        FFI_OK
    );
    (cif, args)
}

#[test]
fn configured_arm_layout_and_types() {
    assert_eq!(core::mem::size_of::<ffi_cif>(), 48);
    unsafe {
        assert_eq!(ffi_type_longdouble.size, 8);
        assert_eq!(ffi_type_longdouble.alignment, 8);
        assert_eq!(ffi_type_complex_float.size, 8);
        assert_eq!(ffi_type_complex_double.size, 16);
        assert_eq!(ffi_type_complex_longdouble.size, 16);
    }
}

#[test]
fn calls_foreign_function() {
    unsafe {
        let (mut cif, _types) = int_cif();
        let mut a = 19i32;
        let mut b = 23i32;
        let mut values = [
            (&mut a as *mut i32).cast::<c_void>(),
            (&mut b as *mut i32).cast::<c_void>(),
        ];
        let mut result = 0i32;
        let target: ffi_fn = core::mem::transmute(add_i32 as unsafe extern "C" fn(i32, i32) -> i32);
        ffi_call(
            &mut cif,
            Some(target),
            (&mut result as *mut i32).cast(),
            values.as_mut_ptr(),
        );
        assert_eq!(result, 42);
    }
}

#[test]
fn executable_closure_runs() {
    unsafe {
        let (mut cif, _types) = int_cif();
        let mut code = core::ptr::null_mut();
        let closure = ffi_closure_alloc(ffi_get_closure_size(), &mut code);
        assert!(!closure.is_null());
        assert!(!code.is_null());
        let mut bias = 7i32;
        assert_eq!(
            ffi_prep_closure_loc(
                closure,
                &mut cif,
                Some(closure_add),
                (&mut bias as *mut i32).cast(),
                code,
            ),
            FFI_OK
        );
        let invoke: unsafe extern "C" fn(i32, i32) -> i32 = core::mem::transmute(code);
        assert_eq!(invoke(10, 25), 42);
        ffi_closure_free(closure);
    }
}
