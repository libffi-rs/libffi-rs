use core::ffi::c_void;
use libffi_powerpc64le_musl_c2rust as ffi;

unsafe extern "C" fn add(a: i64, b: i64) -> i64 {
    a + b
}
unsafe extern "C" fn closure_callback(
    _cif: *mut ffi::powerpc::ffi::ffi_cif,
    result: *mut c_void,
    args: *mut *mut c_void,
    user: *mut c_void,
) {
    let value = *(*args).cast::<i64>();
    *result.cast::<i64>() = value + user as usize as i64;
}

#[test]
fn target_layout_and_complex_descriptors() {
    unsafe {
        assert_eq!(core::mem::size_of::<ffi::prep_cif::ffi_cif>(), 40);
        assert_eq!(core::mem::size_of::<ffi::powerpc::ffi::ffi_cif>(), 40);
        let mut cif: ffi::prep_cif::ffi_cif = core::mem::zeroed();
        let ty = (&raw mut ffi::types::ffi_type_sint64).cast::<ffi::prep_cif::ffi_type>();
        assert_eq!(
            ffi::prep_cif::ffi_prep_cif(&mut cif, 8, 0, ty, core::ptr::null_mut()),
            0
        );
        let longdouble = &raw const ffi::types::ffi_type_longdouble;
        assert_eq!(((*longdouble).size, (*longdouble).alignment), (8, 8));
        let complex = &raw const ffi::types::ffi_type_complex_longdouble;
        assert_eq!(
            ((*complex).size, (*complex).alignment, (*complex).type_0),
            (16, 8, 15)
        );
        assert_eq!(
            *(*complex).elements,
            (&raw mut ffi::types::ffi_type_longdouble)
        );
    }
}

#[test]
fn ffi_call_i64() {
    unsafe {
        let mut cif: ffi::prep_cif::ffi_cif = core::mem::zeroed();
        let ty = (&raw mut ffi::types::ffi_type_sint64).cast::<ffi::prep_cif::ffi_type>();
        let mut types = [ty, ty];
        assert_eq!(
            ffi::prep_cif::ffi_prep_cif(&mut cif, 8, 2, ty, types.as_mut_ptr()),
            0
        );
        let mut a = 19i64;
        let mut b = 23i64;
        let mut values = [
            (&mut a as *mut i64).cast::<c_void>(),
            (&mut b as *mut i64).cast::<c_void>(),
        ];
        let mut result = 0i64;
        let fun: Option<unsafe extern "C" fn()> = Some(core::mem::transmute(
            add as unsafe extern "C" fn(i64, i64) -> i64,
        ));
        ffi::powerpc::ffi::ffi_call(
            (&mut cif as *mut ffi::prep_cif::ffi_cif).cast(),
            fun,
            (&mut result as *mut i64).cast(),
            values.as_mut_ptr(),
        );
        assert_eq!(result, 42);
    }
}

#[test]
fn executable_closure_i64() {
    unsafe {
        let mut cif: ffi::prep_cif::ffi_cif = core::mem::zeroed();
        let ty = (&raw mut ffi::types::ffi_type_sint64).cast::<ffi::prep_cif::ffi_type>();
        let mut types = [ty];
        assert_eq!(
            ffi::prep_cif::ffi_prep_cif(&mut cif, 8, 1, ty, types.as_mut_ptr()),
            0
        );
        let mut code: *mut c_void = core::ptr::null_mut();
        let closure = ffi::closures::ffi_closure_alloc(
            core::mem::size_of::<ffi::powerpc::ffi::ffi_closure>(),
            &mut code,
        )
        .cast::<ffi::powerpc::ffi::ffi_closure>();
        assert!(!closure.is_null() && !code.is_null());
        let status = ffi::powerpc::ffi::ffi_prep_closure_loc(
            closure,
            (&mut cif as *mut ffi::prep_cif::ffi_cif).cast(),
            Some(closure_callback),
            7usize as *mut c_void,
            code,
        );
        assert_eq!(status, 0);
        let callable: unsafe extern "C" fn(i64) -> i64 = core::mem::transmute(code);
        assert_eq!(callable(35), 42);
        ffi::closures::ffi_closure_free(closure.cast());
    }
}
