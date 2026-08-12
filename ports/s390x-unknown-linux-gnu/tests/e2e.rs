use core::ffi::{c_int, c_uint, c_ushort, c_void};

const FFI_DEFAULT_ABI: c_uint = 1;
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

type ClosureCallback =
    unsafe extern "C" fn(*mut FfiCif, *mut c_void, *mut *mut c_void, *mut c_void);

unsafe extern "C" {
    static mut ffi_type_sint32: FfiType;
    static mut ffi_type_uint8: FfiType;
    static mut ffi_type_uint32: FfiType;
    static mut ffi_type_double: FfiType;
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
        function: Option<unsafe extern "C" fn()>,
        result: *mut c_void,
        args: *mut *mut c_void,
    );
    fn ffi_closure_alloc(size: usize, code: *mut *mut c_void) -> *mut c_void;
    fn ffi_closure_free(closure: *mut c_void);
    fn ffi_get_closure_size() -> usize;
    fn ffi_prep_closure_loc(
        closure: *mut c_void,
        cif: *mut FfiCif,
        callback: Option<ClosureCallback>,
        user_data: *mut c_void,
        code: *mut c_void,
    ) -> c_uint;
}

unsafe fn prep(result_type: *mut FfiType, arg_types: &mut [*mut FfiType]) -> FfiCif {
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

unsafe extern "C" fn add7(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32 {
    a + b + c + d + e + f + g
}

#[test]
fn ffi_call_integer_registers_and_stack() {
    unsafe {
        // Referencing a Rust item ensures this uniquely named crate is linked.
        assert_eq!(
            libffi_c2rust_s390x::src::types::ffi_get_version_number(),
            30502
        );
        let sint32 = core::ptr::addr_of_mut!(ffi_type_sint32);
        let mut arg_types = [sint32; 7];
        let mut cif = prep(sint32, &mut arg_types);
        let mut values = [1_i32, -2, 3, 40, 5, 60, 7];
        let mut args = values.each_mut().map(value_ptr);
        let mut result = 0_i64;
        call(
            &mut cif,
            add7 as *const (),
            value_ptr(&mut result),
            &mut args,
        );
        assert_eq!(result as i32, 114);
    }
}

unsafe extern "C" fn affine_double(value: f64, addend: f64) -> f64 {
    value * 2.0 + addend
}

#[test]
fn ffi_call_floating_point() {
    unsafe {
        let double = core::ptr::addr_of_mut!(ffi_type_double);
        let mut arg_types = [double, double];
        let mut cif = prep(double, &mut arg_types);
        let (mut value, mut addend) = (-127.25_f64, 0.5_f64);
        let mut args = [value_ptr(&mut value), value_ptr(&mut addend)];
        let mut result = 0.0_f64;
        call(
            &mut cif,
            affine_double as *const (),
            value_ptr(&mut result),
            &mut args,
        );
        assert_eq!(result, -254.0);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
struct Record {
    byte: u8,
    number: f64,
    count: u32,
}

unsafe extern "C" fn alter_record(mut value: Record) -> Record {
    value.byte += 1;
    value.number -= 1.0;
    value.count += 2;
    value
}

#[test]
fn ffi_call_struct_by_value() {
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
        let mut cif = prep(struct_type, &mut arg_types);
        let mut value = Record {
            byte: 4,
            number: 8.5,
            count: 10,
        };
        let mut result = core::mem::MaybeUninit::<Record>::uninit();
        let mut args = [value_ptr(&mut value)];
        call(
            &mut cif,
            alter_record as *const (),
            result.as_mut_ptr().cast(),
            &mut args,
        );
        assert_eq!(
            result.assume_init(),
            Record {
                byte: 5,
                number: 7.5,
                count: 12
            }
        );
        assert_eq!(value.count, 10, "structure must be passed by value");
    }
}

unsafe extern "C" fn closure_add(
    _cif: *mut FfiCif,
    result: *mut c_void,
    args: *mut *mut c_void,
    user_data: *mut c_void,
) {
    let lhs = *(*args).cast::<c_int>();
    let rhs = *(*args.add(1)).cast::<c_int>();
    let bias = *user_data.cast::<c_int>();
    // libffi requires integer closure results narrower than ffi_arg to be
    // written using ffi_arg-sized storage.
    *result.cast::<i64>() = i64::from(lhs + rhs + bias);
}

#[test]
fn closure_trampoline_end_to_end() {
    unsafe {
        let sint32 = core::ptr::addr_of_mut!(ffi_type_sint32);
        let mut arg_types = [sint32, sint32];
        let mut cif = prep(sint32, &mut arg_types);
        let mut code = core::ptr::null_mut();
        let closure = ffi_closure_alloc(ffi_get_closure_size(), &mut code);
        assert!(!closure.is_null());
        assert!(!code.is_null());
        let mut bias = 9_i32;
        assert_eq!(
            ffi_prep_closure_loc(
                closure,
                &mut cif,
                Some(closure_add),
                value_ptr(&mut bias),
                code,
            ),
            FFI_OK
        );
        let function =
            core::mem::transmute::<*mut c_void, unsafe extern "C" fn(i32, i32) -> i32>(code);
        assert_eq!(function(11, 22), 42);
        ffi_closure_free(closure);
    }
}

#[test]
fn target_type_descriptor_layouts() {
    unsafe {
        assert_eq!(
            (ffi_type_longdouble.size, ffi_type_longdouble.alignment),
            (16, 8)
        );
        assert_eq!(
            (
                ffi_type_complex_longdouble.size,
                ffi_type_complex_longdouble.alignment,
            ),
            (32, 8)
        );
    }
}
