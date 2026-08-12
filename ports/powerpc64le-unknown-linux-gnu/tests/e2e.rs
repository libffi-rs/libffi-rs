use core::ffi::{c_uint, c_ushort, c_void};

const FFI_DEFAULT_ABI: c_uint = 8;
const FFI_OK: c_uint = 0;
const FFI_TYPE_STRUCT: c_ushort = 13;

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
    nfixedargs: c_uint,
}

extern "C" {
    static mut ffi_type_uint8: FfiType;
    static mut ffi_type_sint8: FfiType;
    static mut ffi_type_sint16: FfiType;
    static mut ffi_type_uint32: FfiType;
    static mut ffi_type_sint32: FfiType;
    static mut ffi_type_float: FfiType;
    static mut ffi_type_double: FfiType;

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
        callback: Option<
            unsafe extern "C" fn(*mut FfiCif, *mut c_void, *mut *mut c_void, *mut c_void),
        >,
        user_data: *mut c_void,
        code: *mut c_void,
    ) -> c_uint;
}

unsafe fn prep_cif(result_type: *mut FfiType, arg_types: &mut [*mut FfiType]) -> FfiCif {
    let mut cif = core::mem::MaybeUninit::<FfiCif>::uninit();
    assert_eq!(
        ffi_prep_cif(
            cif.as_mut_ptr(),
            FFI_DEFAULT_ABI,
            arg_types.len() as c_uint,
            result_type,
            arg_types.as_mut_ptr(),
        ),
        FFI_OK
    );
    cif.assume_init()
}

unsafe fn call(
    cif: &mut FfiCif,
    function: *const (),
    result: *mut c_void,
    args: &mut [*mut c_void],
) {
    let function = core::mem::transmute::<*const (), unsafe extern "C" fn()>(function);
    ffi_call(cif, Some(function), result, args.as_mut_ptr());
}

fn value_ptr<T>(value: &mut T) -> *mut c_void {
    core::ptr::from_mut(value).cast()
}

#[test]
fn reports_libffi_version() {
    unsafe {
        assert_eq!(
            libffi_c2rust_powerpc64le::src::types::ffi_get_version_number(),
            30502
        );
    }
}

// Port of testsuite/libffi.call/return_dbl.c.
unsafe extern "C" fn return_dbl(value: f64) -> f64 {
    2.0 * value
}

#[test]
fn return_dbl_test() {
    unsafe {
        let double_type = core::ptr::addr_of_mut!(ffi_type_double);
        let mut arg_types = [double_type];
        let mut cif = prep_cif(double_type, &mut arg_types);
        let mut value = -127.3_f64;

        while value < 127.0 {
            let mut result = 0.0_f64;
            let mut args = [value_ptr(&mut value)];
            call(
                &mut cif,
                return_dbl as *const (),
                value_ptr(&mut result),
                &mut args,
            );
            assert_eq!(result, return_dbl(value));
            value += 1.0;
        }
    }
}

// Port of testsuite/libffi.call/negint.c.
unsafe extern "C" fn checking(a: i32, b: i16, c: i8) -> i32 {
    (a < 0 && b < 0 && c < 0) as i32
}

#[test]
fn negint_test() {
    unsafe {
        let mut arg_types = [
            core::ptr::addr_of_mut!(ffi_type_sint32),
            core::ptr::addr_of_mut!(ffi_type_sint16),
            core::ptr::addr_of_mut!(ffi_type_sint8),
        ];
        let mut cif = prep_cif(core::ptr::addr_of_mut!(ffi_type_sint32), &mut arg_types);
        let (mut a, mut b, mut c) = (-6_i32, -12_i16, -1_i8);
        let mut result = 0_u64;
        let mut args = [value_ptr(&mut a), value_ptr(&mut b), value_ptr(&mut c)];

        call(
            &mut cif,
            checking as *const (),
            value_ptr(&mut result),
            &mut args,
        );
        assert_ne!(result as i32, 0);
    }
}

// Port of testsuite/libffi.call/many.c. Thirteen arguments force some values
// out of the floating-point argument registers and onto the stack.
unsafe extern "C" fn many_floats(
    f1: f32,
    f2: f32,
    f3: f32,
    f4: f32,
    f5: f32,
    f6: f32,
    f7: f32,
    f8: f32,
    f9: f32,
    f10: f32,
    f11: f32,
    f12: f32,
    f13: f32,
) -> f32 {
    f1 + f2 + f3 + f4 + f5 + f6 + f7 + f8 + f9 + f10 + f11 + f12 + f13
}

#[test]
fn many_test() {
    unsafe {
        let float_type = core::ptr::addr_of_mut!(ffi_type_float);
        let mut arg_types = [float_type; 13];
        let mut cif = prep_cif(float_type, &mut arg_types);
        let mut values = core::array::from_fn::<f32, 13, _>(|i| i as f32);
        let mut args = values.each_mut().map(value_ptr);
        let mut result = 0.0_f32;

        call(
            &mut cif,
            many_floats as *const (),
            value_ptr(&mut result),
            &mut args,
        );
        assert!((result - 78.0).abs() < f32::EPSILON);
    }
}

// Port of testsuite/libffi.call/many2.c.
unsafe extern "C" fn many_uint8(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8) -> u8 {
    a + b + c + d + e + f + g
}

#[test]
fn many2_test() {
    unsafe {
        let uint8_type = core::ptr::addr_of_mut!(ffi_type_uint8);
        let mut arg_types = [uint8_type; 7];
        let mut cif = prep_cif(uint8_type, &mut arg_types);
        let mut values = core::array::from_fn::<u8, 7, _>(|i| i as u8);
        let mut args = values.each_mut().map(value_ptr);
        let mut result = 0_u64;

        call(
            &mut cif,
            many_uint8 as *const (),
            value_ptr(&mut result),
            &mut args,
        );
        assert_eq!(result, 21);
    }
}

// Port of testsuite/libffi.call/struct1.c.
#[derive(Clone, Copy)]
#[repr(C)]
struct TestStructure1 {
    uc: u8,
    d: f64,
    ui: c_uint,
}

unsafe extern "C" fn struct1(mut value: TestStructure1) -> TestStructure1 {
    value.uc += 1;
    value.d -= 1.0;
    value.ui += 1;
    value
}

#[test]
fn struct1_test() {
    unsafe {
        let mut elements = [
            core::ptr::addr_of_mut!(ffi_type_uint8),
            core::ptr::addr_of_mut!(ffi_type_double),
            core::ptr::addr_of_mut!(ffi_type_uint32),
            core::ptr::null_mut(),
        ];
        let mut struct_type = FfiType {
            size: 0,
            alignment: 0,
            kind: FFI_TYPE_STRUCT,
            elements: elements.as_mut_ptr(),
        };
        let struct_type = core::ptr::from_mut(&mut struct_type);
        let mut arg_types = [struct_type];
        let mut cif = prep_cif(struct_type, &mut arg_types);
        let mut value = TestStructure1 {
            uc: 1,
            d: 3.14159,
            ui: 555,
        };
        let mut result = core::mem::MaybeUninit::<TestStructure1>::uninit();
        let mut args = [value_ptr(&mut value)];

        call(
            &mut cif,
            struct1 as *const (),
            result.as_mut_ptr().cast(),
            &mut args,
        );
        let result = result.assume_init();
        assert_eq!(result.uc, 2);
        assert_eq!(result.d, 3.14159 - 1.0);
        assert_eq!(result.ui, 556);
        assert_eq!(value.uc, 1, "the structure must be passed by value");
        assert_eq!(value.d, 3.14159);
        assert_eq!(value.ui, 555);
    }
}

unsafe extern "C" fn add_closure_callback(
    _cif: *mut FfiCif,
    result: *mut c_void,
    args: *mut *mut c_void,
    user_data: *mut c_void,
) {
    let input = *(*args).cast::<i32>();
    let addend = *user_data.cast::<i32>();
    *result.cast::<i32>() = input + addend;
}

#[test]
fn closure_callback_test() {
    unsafe {
        let int_type = core::ptr::addr_of_mut!(ffi_type_sint32);
        let mut arg_types = [int_type];
        let mut cif = prep_cif(int_type, &mut arg_types);
        let mut code = core::ptr::null_mut();
        let closure = ffi_closure_alloc(64, &mut code);
        assert!(!closure.is_null());
        assert!(!code.is_null());
        let mut addend = 37_i32;
        assert_eq!(
            ffi_prep_closure_loc(
                closure,
                &mut cif,
                Some(add_closure_callback),
                value_ptr(&mut addend),
                code,
            ),
            FFI_OK
        );
        let callable = core::mem::transmute::<*mut c_void, unsafe extern "C" fn(i32) -> i32>(code);
        assert_eq!(callable(5), 42);
        ffi_closure_free(closure);
    }
}
