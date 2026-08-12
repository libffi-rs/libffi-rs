use core::ffi::{c_uint, c_ushort, c_void};

const FFI_DEFAULT_ABI: c_uint = 10;
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

unsafe extern "C" {
    static mut ffi_type_uint8: FfiType;
    static mut ffi_type_sint32: FfiType;
    static mut ffi_type_sint64: FfiType;
    static mut ffi_type_float: FfiType;
    static mut ffi_type_double: FfiType;
    static mut ffi_type_longdouble: FfiType;
    static mut ffi_type_complex_float: FfiType;
    static mut ffi_type_complex_double: FfiType;
    static mut ffi_type_complex_longdouble: FfiType;

    fn ffi_prep_cif(
        cif: *mut FfiCif,
        abi: c_uint,
        nargs: c_uint,
        result: *mut FfiType,
        arguments: *mut *mut FfiType,
    ) -> c_uint;
    fn ffi_call(
        cif: *mut FfiCif,
        function: Option<unsafe extern "C" fn()>,
        result: *mut c_void,
        arguments: *mut *mut c_void,
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

unsafe fn prep(result: *mut FfiType, argument_types: &mut [*mut FfiType]) -> FfiCif {
    let mut cif = core::mem::MaybeUninit::<FfiCif>::uninit();
    assert_eq!(
        ffi_prep_cif(
            cif.as_mut_ptr(),
            FFI_DEFAULT_ABI,
            argument_types.len() as c_uint,
            result,
            argument_types.as_mut_ptr(),
        ),
        FFI_OK,
    );
    cif.assume_init()
}

unsafe fn call(
    cif: &mut FfiCif,
    function: *const (),
    result: *mut c_void,
    arguments: &mut [*mut c_void],
) {
    let erased = core::mem::transmute::<*const (), unsafe extern "C" fn()>(function);
    ffi_call(cif, Some(erased), result, arguments.as_mut_ptr());
}

fn value_ptr<T>(value: &mut T) -> *mut c_void {
    core::ptr::from_mut(value).cast()
}

unsafe extern "C" fn sum_ten(
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
fn ffi_call_uses_gprs_and_stack_overflow_area() {
    unsafe {
        let ty = core::ptr::addr_of_mut!(ffi_type_sint64);
        let mut argument_types = [ty; 10];
        let mut cif = prep(ty, &mut argument_types);
        let mut values = [3_i64, -4, 7, 11, 19, -23, 29, 31, -37, 41];
        let mut arguments = values.each_mut().map(value_ptr);
        let mut result = 0_i64;
        call(
            &mut cif,
            sum_ten as *const (),
            value_ptr(&mut result),
            &mut arguments,
        );
        assert_eq!(result, 77);
    }
}

unsafe extern "C" fn sum_thirteen_floats(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
    g: f32,
    h: f32,
    i: f32,
    j: f32,
    k: f32,
    l: f32,
    m: f32,
) -> f32 {
    a + b + c + d + e + f + g + h + i + j + k + l + m
}

#[test]
fn ffi_call_uses_fprs_and_float_stack_spill() {
    unsafe {
        let ty = core::ptr::addr_of_mut!(ffi_type_float);
        let mut argument_types = [ty; 13];
        let mut cif = prep(ty, &mut argument_types);
        let mut values = core::array::from_fn::<f32, 13, _>(|index| index as f32);
        let mut arguments = values.each_mut().map(value_ptr);
        let mut result = 0_f32;
        call(
            &mut cif,
            sum_thirteen_floats as *const (),
            value_ptr(&mut result),
            &mut arguments,
        );
        assert_eq!(result, 78.0);
    }
}

unsafe extern "C" fn add_u8(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

#[test]
fn big_endian_narrow_integer_return_is_unpadded() {
    unsafe {
        let ty = core::ptr::addr_of_mut!(ffi_type_uint8);
        let mut argument_types = [ty, ty];
        let mut cif = prep(ty, &mut argument_types);
        let (mut a, mut b) = (240_u8, 11_u8);
        let mut arguments = [value_ptr(&mut a), value_ptr(&mut b)];
        let mut result = 0_u64;
        call(
            &mut cif,
            add_u8 as *const (),
            value_ptr(&mut result),
            &mut arguments,
        );
        assert_eq!(result as u8, 251);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
struct Record {
    tag: u8,
    value: f64,
    count: c_uint,
}

unsafe extern "C" fn modify_record(mut record: Record) -> Record {
    record.tag += 1;
    record.value *= -2.0;
    record.count += 9;
    record
}

#[test]
fn ffi_call_passes_and_returns_struct_by_value() {
    unsafe {
        let mut elements = [
            core::ptr::addr_of_mut!(ffi_type_uint8),
            core::ptr::addr_of_mut!(ffi_type_double),
            core::ptr::addr_of_mut!(ffi_type_sint32),
            core::ptr::null_mut(),
        ];
        let mut record_type = FfiType {
            size: 0,
            alignment: 0,
            kind: FFI_TYPE_STRUCT,
            elements: elements.as_mut_ptr(),
        };
        let record_type = core::ptr::from_mut(&mut record_type);
        let mut argument_types = [record_type];
        let mut cif = prep(record_type, &mut argument_types);
        let mut input = Record {
            tag: 4,
            value: 3.5,
            count: 20,
        };
        let original = input;
        let mut output = core::mem::MaybeUninit::<Record>::uninit();
        let mut arguments = [value_ptr(&mut input)];
        call(
            &mut cif,
            modify_record as *const (),
            output.as_mut_ptr().cast(),
            &mut arguments,
        );
        assert_eq!(
            output.assume_init(),
            Record {
                tag: 5,
                value: -7.0,
                count: 29
            }
        );
        assert_eq!(input, original, "argument must be copied by value");
    }
}

unsafe extern "C" fn closure_callback(
    _cif: *mut FfiCif,
    result: *mut c_void,
    arguments: *mut *mut c_void,
    user_data: *mut c_void,
) {
    let left = *(*arguments).cast::<i64>();
    let right = *(*arguments.add(1)).cast::<i64>();
    let bias = *user_data.cast::<i64>();
    *result.cast::<i64>() = left * right + bias;
}

#[test]
fn executable_closure_trampoline_calls_rust_callback() {
    unsafe {
        let ty = core::ptr::addr_of_mut!(ffi_type_sint64);
        let mut argument_types = [ty, ty];
        let mut cif = prep(ty, &mut argument_types);
        let mut code = core::ptr::null_mut();
        let closure = ffi_closure_alloc(64, &mut code);
        assert!(!closure.is_null());
        assert!(!code.is_null());
        let mut bias = 12_i64;
        assert_eq!(
            ffi_prep_closure_loc(
                closure,
                &mut cif,
                Some(closure_callback),
                value_ptr(&mut bias),
                code,
            ),
            FFI_OK,
        );
        let callable =
            core::mem::transmute::<*mut c_void, unsafe extern "C" fn(i64, i64) -> i64>(code);
        assert_eq!(callable(6, 7), 54);
        assert_eq!(callable(-3, 9), -15);
        ffi_closure_free(closure);
    }
}

#[test]
fn configured_layout_and_extended_type_descriptors_match_target() {
    unsafe {
        assert_eq!(
            libffi_c2rust_powerpc64::src::types::ffi_get_version_number(),
            30502
        );
        assert_eq!(core::mem::size_of::<FfiCif>(), 40);
        assert_eq!(core::mem::align_of::<FfiCif>(), 8);
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
    }
}
