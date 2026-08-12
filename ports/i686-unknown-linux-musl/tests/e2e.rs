use core::ffi::{c_uint, c_ushort, c_void};
use core::mem::{align_of, offset_of, size_of, MaybeUninit};
use core::ptr;
use libffi_c2rust_i686_musl as port;

core::arch::global_asm!(include_str!("abi_helpers.S"), options(att_syntax));

const FFI_DEFAULT_ABI: c_uint = 1;
const FFI_OK: c_uint = 0;
const FFI_TYPE_COMPLEX: c_ushort = 15;
const FFI_TYPE_STRUCT: c_ushort = 13;

#[repr(C)]
struct FfiType {
    size: u32,
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

extern "C" {
    static mut ffi_type_sint32: FfiType;
    static mut ffi_type_float: FfiType;
    static mut ffi_type_double: FfiType;
    static mut ffi_type_longdouble: FfiType;
    static mut ffi_type_complex_float: FfiType;
    static mut ffi_type_complex_double: FfiType;
    static mut ffi_type_complex_longdouble: FfiType;

    fn ffi_get_default_abi() -> c_uint;
    fn ffi_get_closure_size() -> u32;
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
    fn ffi_closure_alloc(size: u32, code: *mut *mut c_void) -> *mut c_void;
    fn ffi_closure_free(closure: *mut c_void);
    fn ffi_test_longdouble_add_one();
    fn ffi_prep_closure_loc(
        closure: *mut c_void,
        cif: *mut FfiCif,
        callback: Option<ClosureCallback>,
        user_data: *mut c_void,
        code: *mut c_void,
    ) -> c_uint;
}

fn force_link_port_objects() {
    // The tests intentionally exercise the exported C ABI. Taking Rust paths
    // makes rustc retain each translated rlib object that defines that ABI.
    core::hint::black_box(port::prep_cif::ffi_prep_cif as *const () as usize);
    core::hint::black_box(port::ffi::ffi_call as *const () as usize);
    core::hint::black_box(port::ffi::ffi_prep_closure_loc as *const () as usize);
    core::hint::black_box(port::closures::ffi_closure_alloc as *const () as usize);
    core::hint::black_box(port::closures::ffi_closure_free as *const () as usize);
    core::hint::black_box(port::types::ffi_get_default_abi as *const () as usize);
    core::hint::black_box(port::types::ffi_get_closure_size as *const () as usize);
    core::hint::black_box(ptr::addr_of!(port::types::ffi_type_sint32));
    core::hint::black_box(ptr::addr_of!(port::types::ffi_type_float));
    core::hint::black_box(ptr::addr_of!(port::types::ffi_type_double));
    core::hint::black_box(ptr::addr_of!(port::types::ffi_type_longdouble));
    core::hint::black_box(ptr::addr_of!(port::types::ffi_type_complex_float));
    core::hint::black_box(ptr::addr_of!(port::types::ffi_type_complex_double));
    core::hint::black_box(ptr::addr_of!(port::types::ffi_type_complex_longdouble));
}

unsafe fn prep(result: *mut FfiType, args: &mut [*mut FfiType]) -> FfiCif {
    force_link_port_objects();
    let mut cif = MaybeUninit::<FfiCif>::uninit();
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

fn void_ptr<T>(value: &mut T) -> *mut c_void {
    ptr::from_mut(value).cast()
}

unsafe fn erased(function: *const ()) -> Option<unsafe extern "C" fn()> {
    Some(core::mem::transmute::<*const (), unsafe extern "C" fn()>(
        function,
    ))
}

#[test]
fn configured_i686_layout_and_types_are_exact() {
    force_link_port_objects();
    assert_eq!(size_of::<usize>(), 4);
    assert_eq!(size_of::<FfiType>(), 12);
    assert_eq!(align_of::<FfiType>(), 4);
    assert_eq!(size_of::<FfiCif>(), 24);
    assert_eq!(align_of::<FfiCif>(), 4);
    assert_eq!(offset_of!(FfiCif, abi), 0);
    assert_eq!(offset_of!(FfiCif, nargs), 4);
    assert_eq!(offset_of!(FfiCif, arg_types), 8);
    assert_eq!(offset_of!(FfiCif, rtype), 12);
    assert_eq!(offset_of!(FfiCif, bytes), 16);
    assert_eq!(offset_of!(FfiCif, flags), 20);

    unsafe {
        assert_eq!(ffi_get_default_abi(), FFI_DEFAULT_ABI);
        assert_eq!(ffi_get_closure_size(), 32);

        let long_double = ptr::addr_of!(ffi_type_longdouble);
        assert_eq!((*long_double).size, 12);
        assert_eq!((*long_double).alignment, 4);
        assert_eq!((*long_double).kind, 4);

        for (ty, component_size) in [
            (ptr::addr_of!(ffi_type_complex_float), 4),
            (ptr::addr_of!(ffi_type_complex_double), 8),
            (ptr::addr_of!(ffi_type_complex_longdouble), 12),
        ] {
            assert_eq!((*ty).kind, FFI_TYPE_COMPLEX);
            assert_eq!((*ty).size, 2 * component_size);
            assert_eq!((*ty).alignment, 4);
            assert!(!(*ty).elements.is_null());
            assert_eq!((**(*ty).elements).size, component_size);
            assert!((*(*ty).elements.add(1)).is_null());
        }
    }
}

unsafe extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn ffi_call_integer() {
    unsafe {
        let sint32 = ptr::addr_of_mut!(ffi_type_sint32);
        let mut arg_types = [sint32, sint32];
        let mut cif = prep(sint32, &mut arg_types);
        let (mut a, mut b, mut result) = (20_i32, 22_i32, 0_i32);
        let mut values = [void_ptr(&mut a), void_ptr(&mut b)];

        ffi_call(
            &mut cif,
            erased(add as *const ()),
            void_ptr(&mut result),
            values.as_mut_ptr(),
        );
        assert_eq!(result, 42);
        assert_eq!(cif.abi, FFI_DEFAULT_ABI);
        assert_eq!(cif.nargs, 2);
        assert_eq!(cif.bytes, 8);
    }
}

unsafe extern "C" fn ten_values(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: i32,
    g: i32,
    h: i32,
    i: i32,
    j: i32,
) -> i32 {
    a + 2 * b + 3 * c + 4 * d + 5 * e + 6 * f + 7 * g + 8 * h + 9 * i + 10 * j
}

#[test]
fn ffi_call_stack_arguments() {
    unsafe {
        let sint32 = ptr::addr_of_mut!(ffi_type_sint32);
        let mut arg_types = [sint32; 10];
        let mut cif = prep(sint32, &mut arg_types);
        let mut inputs = [1_i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut values = inputs.each_mut().map(void_ptr);
        let mut result = 0_i32;

        ffi_call(
            &mut cif,
            erased(ten_values as *const ()),
            void_ptr(&mut result),
            values.as_mut_ptr(),
        );
        assert_eq!(result, 385);
    }
}

#[test]
fn ffi_call_x87_long_double() {
    unsafe {
        let long_double = ptr::addr_of_mut!(ffi_type_longdouble);
        let mut arg_types = [long_double];
        let mut cif = prep(long_double, &mut arg_types);
        // i686 SysV x87 80-bit values occupy 12-byte C objects. These are
        // little-endian encodings of 1.5L and 2.5L (the last two bytes pad).
        let mut input = [0_u8, 0, 0, 0, 0, 0, 0, 0xc0, 0xff, 0x3f, 0, 0];
        let expected = [0_u8, 0, 0, 0, 0, 0, 0, 0xa0, 0x00, 0x40];
        let mut output = [0xa5_u8; 12];
        let mut values = [input.as_mut_ptr().cast()];

        ffi_call(
            &mut cif,
            erased(ffi_test_longdouble_add_one as *const ()),
            output.as_mut_ptr().cast(),
            values.as_mut_ptr(),
        );
        assert_eq!(&output[..10], &expected);
    }
}

unsafe extern "C" fn mixed_float(a: f64, b: f32, c: f64) -> f64 {
    a + f64::from(b) * c
}

#[test]
fn ffi_call_floating_point() {
    unsafe {
        let mut arg_types = [
            ptr::addr_of_mut!(ffi_type_double),
            ptr::addr_of_mut!(ffi_type_float),
            ptr::addr_of_mut!(ffi_type_double),
        ];
        let mut cif = prep(ptr::addr_of_mut!(ffi_type_double), &mut arg_types);
        let (mut a, mut b, mut c) = (1.5_f64, 2.0_f32, 20.25_f64);
        let mut values = [void_ptr(&mut a), void_ptr(&mut b), void_ptr(&mut c)];
        let mut result = 0.0_f64;

        ffi_call(
            &mut cif,
            erased(mixed_float as *const ()),
            void_ptr(&mut result),
            values.as_mut_ptr(),
        );
        assert_eq!(result, 42.0);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
struct Pair {
    first: i32,
    second: i32,
}

unsafe extern "C" fn change_pair(value: Pair) -> Pair {
    Pair {
        first: value.first + 10,
        second: value.second * 2,
    }
}

#[test]
fn ffi_call_struct_by_value_and_return() {
    unsafe {
        let sint32 = ptr::addr_of_mut!(ffi_type_sint32);
        let mut elements = [sint32, sint32, ptr::null_mut()];
        let mut pair_type = FfiType {
            size: 0,
            alignment: 0,
            kind: FFI_TYPE_STRUCT,
            elements: elements.as_mut_ptr(),
        };
        let pair_type = ptr::from_mut(&mut pair_type);
        let mut arg_types = [pair_type];
        let mut cif = prep(pair_type, &mut arg_types);
        assert_eq!((*pair_type).size, 8);
        assert_eq!((*pair_type).alignment, 4);

        let mut input = Pair {
            first: 7,
            second: 16,
        };
        let mut output = MaybeUninit::<Pair>::uninit();
        let mut values = [void_ptr(&mut input)];
        ffi_call(
            &mut cif,
            erased(change_pair as *const ()),
            output.as_mut_ptr().cast(),
            values.as_mut_ptr(),
        );
        assert_eq!(
            output.assume_init(),
            Pair {
                first: 17,
                second: 32
            }
        );
        assert_eq!(
            input,
            Pair {
                first: 7,
                second: 16
            }
        );
    }
}

unsafe extern "C" fn closure_add(
    _cif: *mut FfiCif,
    result: *mut c_void,
    args: *mut *mut c_void,
    user_data: *mut c_void,
) {
    let a = *(*args).cast::<i32>();
    let b = *(*args.add(1)).cast::<i32>();
    let bias = *user_data.cast::<i32>();
    *result.cast::<i32>() = a + b + bias;
}

#[test]
fn executable_closure_round_trip() {
    unsafe {
        let sint32 = ptr::addr_of_mut!(ffi_type_sint32);
        let mut arg_types = [sint32, sint32];
        let mut cif = prep(sint32, &mut arg_types);
        let mut code = ptr::null_mut();
        let closure = ffi_closure_alloc(ffi_get_closure_size(), &mut code);
        assert!(!closure.is_null());
        assert!(!code.is_null());
        let mut bias = 2_i32;
        assert_eq!(
            ffi_prep_closure_loc(
                closure,
                &mut cif,
                Some(closure_add),
                void_ptr(&mut bias),
                code,
            ),
            FFI_OK
        );

        let callable =
            core::mem::transmute::<*mut c_void, unsafe extern "C" fn(i32, i32) -> i32>(code);
        assert_eq!(callable(19, 21), 42);
        assert_eq!(callable(-10, 7), -1);
        ffi_closure_free(closure);
    }
}
