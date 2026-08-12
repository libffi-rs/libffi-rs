use core::ffi::{c_uint, c_ushort, c_void};

const FFI_UNIX64: c_uint = 2;
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
}

extern "C" {
    static mut ffi_type_sint64: FfiType;
    static mut ffi_type_uint32: FfiType;
    static mut ffi_type_double: FfiType;
    static mut ffi_type_pointer: FfiType;
    static mut ffi_type_longdouble: FfiType;
    static mut ffi_type_complex_float: FfiType;
    static mut ffi_type_complex_double: FfiType;
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
        function: Option<unsafe extern "C" fn()>,
        result: *mut c_void,
        args: *mut *mut c_void,
    );
    fn ffi_get_closure_size() -> usize;
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

unsafe fn make_cif(result: *mut FfiType, args: &mut [*mut FfiType]) -> FfiCif {
    let mut cif = core::mem::MaybeUninit::<FfiCif>::uninit();
    assert_eq!(
        ffi_prep_cif(
            cif.as_mut_ptr(),
            FFI_UNIX64,
            args.len() as c_uint,
            result,
            args.as_mut_ptr(),
        ),
        FFI_OK
    );
    cif.assume_init()
}

fn erased<T>(value: &mut T) -> *mut c_void {
    core::ptr::from_mut(value).cast()
}

unsafe fn invoke(
    cif: &mut FfiCif,
    function: *const (),
    result: *mut c_void,
    args: &mut [*mut c_void],
) {
    let function = core::mem::transmute::<*const (), unsafe extern "C" fn()>(function);
    ffi_call(cif, Some(function), result, args.as_mut_ptr());
}

#[test]
fn x32_descriptor_and_cif_layouts() {
    // Ensure this is truly an ILP32 target using the x86-64 instruction set.
    assert_eq!(core::mem::size_of::<usize>(), 4);
    assert_eq!(core::mem::size_of::<FfiType>(), 12);
    assert_eq!(core::mem::size_of::<FfiCif>(), 24);
    unsafe {
        let pointer = &raw const ffi_type_pointer;
        let long_double = &raw const ffi_type_longdouble;
        assert_eq!((*pointer).size, 4);
        assert_eq!((*pointer).alignment, 4);
        assert_eq!((*long_double).size, 16);
        assert_eq!((*long_double).alignment, 16);
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
        assert_eq!(
            libffi_c2rust_x86_64_gnux32::src::types::ffi_get_default_abi(),
            FFI_UNIX64
        );
    }
}

unsafe extern "C" fn weighted_sum(
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    f: i64,
    g: i64,
    h: i64,
) -> i64 {
    a + 2 * b + 3 * c + 4 * d + 5 * e + 6 * f + 7 * g + 8 * h
}

#[test]
fn ffi_call_integer_registers_and_stack() {
    unsafe {
        let int64 = core::ptr::addr_of_mut!(ffi_type_sint64);
        let mut types = [int64; 8];
        let mut cif = make_cif(int64, &mut types);
        let mut values = [1_i64, -2, 3, -4, 5, -6, 7, -8];
        let mut pointers = values.each_mut().map(erased);
        let mut result = 0_i64;
        invoke(
            &mut cif,
            weighted_sum as *const (),
            erased(&mut result),
            &mut pointers,
        );
        assert_eq!(result, weighted_sum(1, -2, 3, -4, 5, -6, 7, -8));
    }
}

unsafe extern "C" fn affine_double(x: f64, y: f64) -> f64 {
    x * 1.5 + y
}

#[test]
fn ffi_call_sse_arguments_and_result() {
    unsafe {
        let double = core::ptr::addr_of_mut!(ffi_type_double);
        let mut types = [double, double];
        let mut cif = make_cif(double, &mut types);
        let (mut x, mut y) = (6.25_f64, -1.5_f64);
        let mut pointers = [erased(&mut x), erased(&mut y)];
        let mut result = 0.0_f64;
        invoke(
            &mut cif,
            affine_double as *const (),
            erased(&mut result),
            &mut pointers,
        );
        assert_eq!(result, affine_double(x, y));
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
struct Pair {
    left: u32,
    right: u32,
}

unsafe extern "C" fn rotate_pair(pair: Pair, delta: u32) -> Pair {
    Pair {
        left: pair.right + delta,
        right: pair.left - delta,
    }
}

#[test]
fn ffi_call_structure_by_value() {
    unsafe {
        let uint32 = core::ptr::addr_of_mut!(ffi_type_uint32);
        let mut elements = [uint32, uint32, core::ptr::null_mut()];
        let mut pair_type = FfiType {
            size: 0,
            alignment: 0,
            kind: FFI_TYPE_STRUCT,
            elements: elements.as_mut_ptr(),
        };
        let mut types = [core::ptr::addr_of_mut!(pair_type), uint32];
        let mut cif = make_cif(core::ptr::addr_of_mut!(pair_type), &mut types);
        assert_eq!((pair_type.size, pair_type.alignment), (8, 4));
        let mut pair = Pair { left: 20, right: 7 };
        let mut delta = 3_u32;
        let mut pointers = [erased(&mut pair), erased(&mut delta)];
        let mut result = Pair { left: 0, right: 0 };
        invoke(
            &mut cif,
            rotate_pair as *const (),
            erased(&mut result),
            &mut pointers,
        );
        assert_eq!(result, rotate_pair(pair, delta));
    }
}

unsafe extern "C" fn closure_callback(
    _cif: *mut FfiCif,
    result: *mut c_void,
    args: *mut *mut c_void,
    user_data: *mut c_void,
) {
    let argument = *(*args).cast::<i64>();
    let bias = *user_data.cast::<i64>();
    *result.cast::<i64>() = argument + bias;
}

#[test]
fn executable_closure_round_trip() {
    unsafe {
        let int64 = core::ptr::addr_of_mut!(ffi_type_sint64);
        let mut types = [int64];
        let mut cif = make_cif(int64, &mut types);
        let mut code = core::ptr::null_mut();
        let closure = ffi_closure_alloc(ffi_get_closure_size(), &mut code);
        assert!(!closure.is_null());
        assert!(!code.is_null());
        let mut bias = 19_i64;
        assert_eq!(
            ffi_prep_closure_loc(
                closure,
                &mut cif,
                Some(closure_callback),
                erased(&mut bias),
                code,
            ),
            FFI_OK
        );
        let callable = core::mem::transmute::<*mut c_void, unsafe extern "C" fn(i64) -> i64>(code);
        assert_eq!(callable(23), 42);
        assert_eq!(callable(-20), -1);
        ffi_closure_free(closure);
    }
}
