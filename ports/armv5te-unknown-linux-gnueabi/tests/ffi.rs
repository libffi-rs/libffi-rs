use core::ffi::{c_int, c_void};
use libffi_armv5te_port::{arm::ffi as arm, closures, prep_cif as prep, types};

unsafe extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}
unsafe extern "C" fn add_six(a: c_int, b: c_int, c: c_int, d: c_int, e: c_int, f: c_int) -> c_int {
    a + b + c + d + e + f
}

unsafe extern "C" fn closure_add(
    _cif: *mut arm::ffi_cif,
    result: *mut c_void,
    args: *mut *mut c_void,
    user_data: *mut c_void,
) {
    let a = *(*args.add(0)).cast::<c_int>();
    let b = *(*args.add(1)).cast::<c_int>();
    *result.cast::<c_int>() = a + b + *(user_data.cast::<c_int>());
}

fn prepared_cif(arg_types: &mut [*mut prep::ffi_type]) -> prep::ffi_cif {
    for ty in arg_types.iter_mut() {
        *ty = core::ptr::addr_of_mut!(types::ffi_type_sint32).cast::<prep::ffi_type>();
    }
    let mut cif: prep::ffi_cif = unsafe { core::mem::zeroed() };
    let status = unsafe {
        prep::ffi_prep_cif(
            &mut cif,
            prep::FFI_DEFAULT_ABI,
            arg_types.len() as u32,
            core::ptr::addr_of_mut!(types::ffi_type_sint32).cast(),
            arg_types.as_mut_ptr(),
        )
    };
    assert_eq!(status, prep::FFI_OK);
    cif
}

#[test]
fn call_two_ints() {
    let mut arg_types = [core::ptr::null_mut(); 2];
    let mut cif = prepared_cif(&mut arg_types);
    let (mut a, mut b, mut result) = (19i32, 23i32, 0i32);
    let mut values = [
        (&mut a as *mut i32).cast::<c_void>(),
        (&mut b as *mut i32).cast::<c_void>(),
    ];
    unsafe {
        arm::ffi_call(
            (&mut cif as *mut prep::ffi_cif).cast(),
            Some(core::mem::transmute::<
                unsafe extern "C" fn(c_int, c_int) -> c_int,
                unsafe extern "C" fn(),
            >(add)),
            (&mut result as *mut i32).cast(),
            values.as_mut_ptr(),
        );
    }
    assert_eq!(result, 42);
}

#[test]
fn call_with_stack_arguments() {
    let mut arg_types = [core::ptr::null_mut(); 6];
    let mut cif = prepared_cif(&mut arg_types);
    let mut args = [1i32, 2, 3, 4, 5, 6];
    let mut values = args.map(|v| core::ptr::null_mut::<c_void>());
    for (out, arg) in values.iter_mut().zip(args.iter_mut()) {
        *out = (arg as *mut i32).cast();
    }
    let mut result = 0i32;
    unsafe {
        arm::ffi_call(
            (&mut cif as *mut prep::ffi_cif).cast(),
            Some(core::mem::transmute::<
                unsafe extern "C" fn(c_int, c_int, c_int, c_int, c_int, c_int) -> c_int,
                unsafe extern "C" fn(),
            >(add_six)),
            (&mut result as *mut i32).cast(),
            values.as_mut_ptr(),
        );
    }
    assert_eq!(result, 21);
}

#[test]
fn executable_closure() {
    let mut arg_types = [core::ptr::null_mut(); 2];
    let mut cif = prepared_cif(&mut arg_types);
    let mut code: *mut c_void = core::ptr::null_mut();
    let closure =
        unsafe { closures::ffi_closure_alloc(core::mem::size_of::<arm::ffi_closure>(), &mut code) };
    assert!(!closure.is_null());
    assert!(!code.is_null());
    let mut bias = 7i32;
    let status = unsafe {
        arm::ffi_prep_closure_loc(
            closure.cast(),
            (&mut cif as *mut prep::ffi_cif).cast(),
            Some(closure_add),
            (&mut bias as *mut i32).cast(),
            code,
        )
    };
    assert_eq!(status, arm::FFI_OK);
    let invoke: unsafe extern "C" fn(c_int, c_int) -> c_int = unsafe { core::mem::transmute(code) };
    assert_eq!(unsafe { invoke(10, 20) }, 37);
    unsafe { closures::ffi_closure_free(closure) };
}

#[test]
fn configured_layout_and_long_double() {
    assert_eq!(core::mem::size_of::<prep::ffi_cif>(), 48);
    assert_eq!(unsafe { types::ffi_type_longdouble.size }, 8);
    assert_eq!(unsafe { types::ffi_type_longdouble.type_0 }, 3); // aliases double
    assert_eq!(unsafe { types::ffi_type_complex_longdouble.size }, 16);
}
