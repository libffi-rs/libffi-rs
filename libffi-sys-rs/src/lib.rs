#![doc(html_root_url = "https://docs.rs/libffi-sys/3.3.2")]
//! Low-level Rust bindings for [libffi](https://sourceware.org/libffi/)
//!
//! The C libffi library provides two main facilities: assembling calls
//! to functions dynamically, and creating closures that can be called
//! as ordinary C functions.
//!
//! This is a mostly undocumented wrapper, originally generated by bindgen then
//! cleaned up manually, intended as the basis for higher-level bindings.
//!
//! See [the libffi crate](https://crates.io/crates/libffi/) for a
//! higher-level API.
//!
//! # Usage
//!
//! `libffi-sys` can either build its own copy of the libffi C library [from
//! github](https://github.com/libffi/libffi) or it can link against your
//! system's C libffi. By default it builds its own because many systems ship
//! with an old C libffi; this requires that you have a working make, C
//! compiler, automake, and autoconf first. If your system libffi is recent
//! enough (v3.2.1 as of October 2019), you can instead enable the `system`
//! feature flag to use that.
//!
//! On Windows, it is not supported to link against a shared libffi library as
//! it is generally not available. Automake and autoconf are not required when
//! building for the `x86_64-pc-windows-msvc` and `i686-pc-windows-msvc`
//! targets.
//!
//! If you want this crate to build a C libffi for you, add
//!
//! ```toml
//! [dependencies]
//! libffi-sys = "3.3.2"
//! ```
//!
//! to your `Cargo.toml`. If you want to use your system C libffi, then
//!
//! ```toml
//! [dependencies.libffi-sys]
//! version = "3.3.2"
//! features = ["system"]
//! ```
//!
//! to your `Cargo.toml` instead.
//!
//! This crate supports Rust version 1.78 and later.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(unused_imports)]

use core::ffi::{c_char, c_int, c_long, c_schar, c_uint, c_ulong, c_ushort, c_void};
use core::fmt::{self, Debug, Formatter};
use core::mem::zeroed;

mod arch;
pub use arch::*;

/// The smallest unsigned integer type returned by [`ffi_call`].
pub type ffi_arg = c_ulong;
/// The smallest signed integer type returned by [`ffi_call`].
pub type ffi_sarg = c_long;
/// The type used to convey the ABI of a function.
pub type ffi_abi = u32;
/// The return type of `libffi`'s functions that may return an error.
pub type ffi_status = u32;
pub type ffi_type_enum = u32;

pub const FFI_64_BIT_MAX: u64 = 9_223_372_036_854_775_807;
pub const FFI_CLOSURES: u32 = 1;
pub const FFI_SIZEOF_ARG: usize = core::mem::size_of::<c_long>();
// NOTE: This only differs from FFI_SIZEOF_ARG on ILP platforms, which Rust does not support
pub const FFI_SIZEOF_JAVA_RAW: usize = FFI_SIZEOF_ARG;

pub const FFI_TYPE_VOID: u32 = 0;
pub const FFI_TYPE_INT: u32 = 1;
pub const FFI_TYPE_FLOAT: u32 = 2;
pub const FFI_TYPE_DOUBLE: u32 = 3;
pub const FFI_TYPE_LONGDOUBLE: u32 = 4;
pub const FFI_TYPE_UINT8: u32 = 5;
pub const FFI_TYPE_SINT8: u32 = 6;
pub const FFI_TYPE_UINT16: u32 = 7;
pub const FFI_TYPE_SINT16: u32 = 8;
pub const FFI_TYPE_UINT32: u32 = 9;
pub const FFI_TYPE_SINT32: u32 = 10;
pub const FFI_TYPE_UINT64: u32 = 11;
pub const FFI_TYPE_SINT64: u32 = 12;
pub const FFI_TYPE_STRUCT: u32 = 13;
pub const FFI_TYPE_POINTER: u32 = 14;
pub const FFI_TYPE_COMPLEX: u32 = 15;
pub const FFI_TYPE_LAST: u32 = 15;

pub const ffi_status_FFI_OK: ffi_status = 0;
pub const ffi_status_FFI_BAD_TYPEDEF: ffi_status = 1;
pub const ffi_status_FFI_BAD_ABI: ffi_status = 2;
pub const ffi_status_FFI_BAD_ARGTYPE: ffi_status = 3;

pub const ffi_type_enum_STRUCT: ffi_type_enum = 13;
pub const ffi_type_enum_COMPLEX: ffi_type_enum = 15;

/// A struct used by `libffi` to describe types and their memory layout.
///
/// New `ffi_type` variables should only be constructed for describing the
/// layout of custom structs. For plain scalar types it is recommended to refer
/// to the `static` variables defined by libffi instead of creating new
/// `ffi_type`s.
///
/// When creating new `ffi_type`s, the `size` and `alignment` fields should be
/// left at their default values, as `libffi` will fill out these fields during
/// [`ffi_prep_cif`].
///
/// # Example
///
/// ```
/// use std::ptr;
///
/// #[repr(C)]
/// struct CustomStruct {
///     num: u32,
///     num2: i64,
///     float_num: f32,
/// }
///
/// // We need to describe the types of the values in `CustomStruct`. The order
/// // must be the same as the order in the struct definition. Note that this
/// // array must be alive and at the same address for the entire lifetime of
/// // the resulting `ffi_type`.
/// let mut elements_array = unsafe {[
///     // `libffi::low::types::uint32`, `sint64`, and `float` can be used
///     // instead if using libffi (not -sys)
///     ptr::addr_of_mut!(libffi_sys::ffi_type_uint32),
///     ptr::addr_of_mut!(libffi_sys::ffi_type_sint64),
///     ptr::addr_of_mut!(libffi_sys::ffi_type_float),
///     // The last element in the array must be a `NULL` since `ffi_type` does
///     // not store the number of elements in the struct.
///     ptr::null_mut(),
/// ]};
///
/// let mut custom_struct_description = libffi_sys::ffi_type {
///     // `libffi::low::type_tag::STRUCT` can be used instead if using libffi
///     type_: libffi_sys::FFI_TYPE_STRUCT as u16,
///     elements: elements_array.as_mut_ptr(),
///     ..Default::default()
/// };
///
/// // `custom_struct_description` can now be used in a [`ffi_cif`] to send
/// // `CustomStruct` as an argument or receive a `CustomStruct` as response.
/// ```
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffi_type {
    pub size: usize,
    pub alignment: c_ushort,
    pub type_: c_ushort,
    pub elements: *mut *mut ffi_type,
}

impl Default for ffi_type {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

/// A struct used by `libffi` to describe a function's ABI and type signature.
///
/// It is recommended to not fill out the fields in a `ffi_cif` manually, but
/// rather supply the correct arguments to [`ffi_prep_cif`].
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ffi_cif {
    pub abi: ffi_abi,
    pub nargs: c_uint,
    pub arg_types: *mut *mut ffi_type,
    pub rtype: *mut ffi_type,
    pub bytes: c_uint,
    pub flags: c_uint,
    #[cfg(all(target_arch = "aarch64", target_os = "windows"))]
    pub is_variadic: c_uint,
    #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
    pub aarch64_nfixedargs: c_uint,
    #[cfg(target_arch = "arm")]
    pub vfp_used: c_int,
    #[cfg(target_arch = "arm")]
    pub vfp_reg_free: c_ushort,
    #[cfg(target_arch = "arm")]
    pub vfp_nargs: c_ushort,
    #[cfg(target_arch = "arm")]
    pub vfp_args: [c_schar; 16],
    #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
    pub nfixedargs: c_uint,
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    pub riscv_nfixedargs: c_uint,
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    pub riscv_unused: c_uint,
    #[cfg(target_arch = "sparc64")]
    pub nfixedargs: c_uint,
    #[cfg(target_arch = "loongarch64")]
    pub loongarch_nfixedargs: c_uint,
    #[cfg(target_arch = "loongarch64")]
    pub loongarch_unused: c_uint,
    #[cfg(any(
        target_arch = "mips",
        target_arch = "mips32r6",
        target_arch = "mips64",
        target_arch = "mips64r6"
    ))]
    pub mips_nfixedargs: c_uint,
}

impl Default for ffi_cif {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C, align(64))]
#[derive(Copy, Clone)]
pub union ffi_raw {
    pub sint: ffi_sarg,
    pub uint: ffi_arg,
    pub flt: f32,
    pub data: [c_char; FFI_SIZEOF_ARG],
    pub ptr: *mut c_void,
}

impl Default for ffi_raw {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

pub type ffi_java_raw = ffi_raw;

#[repr(C, align(64))]
#[derive(Copy, Clone)]
pub union ffi_trampoline {
    pub tramp: [c_char; FFI_TRAMPOLINE_SIZE],
    pub ftramp: *mut c_void,
}

/// A struct used by `libffi` to describe and manage closures.
///
/// Closures in libffi can be used to create function pointers to functions with
/// arbitrary signatures that can also have some custom data embedded.
///
/// `ffi_closure` should not be created manually. Instead, [`ffi_closure_alloc`]
/// should be invoked to allocate memory for the `ffi_closure` before its fields
/// are populated by [`ffi_prep_closure`].
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffi_closure {
    pub tramp: ffi_trampoline,
    pub cif: *mut ffi_cif,
    pub fun: Option<
        unsafe extern "C" fn(
            arg1: *mut ffi_cif,
            arg2: *mut c_void,
            arg3: *mut *mut c_void,
            arg4: *mut c_void,
        ),
    >,
    pub user_data: *mut c_void,
}

/// Implements Debug manually since sometimes `FFI_TRAMPOLINE_SIZE` is too large
impl Debug for ffi_closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ffi_closure")
            .field("tramp", unsafe { &&self.tramp.tramp[..] })
            .field("cif", &self.cif)
            .field("fun", &self.fun)
            .field("user_data", &self.user_data)
            .finish()
    }
}

impl Default for ffi_closure {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffi_raw_closure {
    pub tramp: [c_char; FFI_TRAMPOLINE_SIZE],
    pub cif: *mut ffi_cif,
    // See: https://github.com/libffi/libffi/blob/3a7580da73b7f16f275277316d00e3497cbb5a8c/include/ffi.h.in#L364
    #[cfg(not(target_arch = "x86"))]
    pub translate_args: Option<
        unsafe extern "C" fn(
            arg1: *mut ffi_cif,
            arg2: *mut c_void,
            arg3: *mut *mut c_void,
            arg4: *mut c_void,
        ),
    >,
    #[cfg(not(target_arch = "x86"))]
    pub this_closure: *mut c_void,
    pub fun: Option<
        unsafe extern "C" fn(
            arg1: *mut ffi_cif,
            arg2: *mut c_void,
            arg3: *mut ffi_raw,
            arg4: *mut c_void,
        ),
    >,
    pub user_data: *mut c_void,
}

/// Implements Debug manually since sometimes `FFI_TRAMPOLINE_SIZE` is too large
impl Debug for ffi_raw_closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("ffi_raw_closure");
        debug_struct
            .field("tramp", &&self.tramp[..])
            .field("cif", &self.cif);

        #[cfg(not(target_arch = "x86"))]
        debug_struct.field("translate_args", &self.translate_args);
        #[cfg(not(target_arch = "x86"))]
        debug_struct.field("this_closure", &self.this_closure);

        debug_struct
            .field("fun", &self.fun)
            .field("user_data", &self.user_data)
            .finish()
    }
}

impl Default for ffi_raw_closure {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffi_java_raw_closure {
    pub tramp: [c_char; FFI_TRAMPOLINE_SIZE],
    pub cif: *mut ffi_cif,
    // See: https://github.com/libffi/libffi/blob/252c0f463641e6100169c3f0a4a590d7df438278/include/ffi.h.in#L386
    #[cfg(not(target_arch = "x86"))]
    pub translate_args: Option<
        unsafe extern "C" fn(
            arg1: *mut ffi_cif,
            arg2: *mut c_void,
            arg3: *mut *mut c_void,
            arg4: *mut c_void,
        ),
    >,
    #[cfg(not(target_arch = "x86"))]
    pub this_closure: *mut c_void,
    pub fun: Option<
        unsafe extern "C" fn(
            arg1: *mut ffi_cif,
            arg2: *mut c_void,
            arg3: *mut ffi_java_raw,
            arg4: *mut c_void,
        ),
    >,
    pub user_data: *mut c_void,
}

/// Implements Debug manually since sometimes `FFI_TRAMPOLINE_SIZE` is too large
impl Debug for ffi_java_raw_closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("ffi_java_raw_closure");
        debug_struct
            .field("tramp", &&self.tramp[..])
            .field("cif", &self.cif);

        #[cfg(not(target_arch = "x86"))]
        debug_struct.field("translate_args", &self.translate_args);
        #[cfg(not(target_arch = "x86"))]
        debug_struct.field("this_closure", &self.this_closure);

        debug_struct
            .field("fun", &self.fun)
            .field("user_data", &self.user_data)
            .finish()
    }
}

impl Default for ffi_java_raw_closure {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ffi_go_closure {
    pub tramp: *mut c_void,
    pub cif: *mut ffi_cif,
    pub fun: Option<
        unsafe extern "C" fn(
            arg1: *mut ffi_cif,
            arg2: *mut c_void,
            arg3: *mut *mut c_void,
            arg4: *mut c_void,
        ),
    >,
}
impl Default for ffi_go_closure {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

extern "C" {
    pub static mut ffi_type_void: ffi_type;
    pub static mut ffi_type_uint8: ffi_type;
    pub static mut ffi_type_sint8: ffi_type;
    pub static mut ffi_type_uint16: ffi_type;
    pub static mut ffi_type_sint16: ffi_type;
    pub static mut ffi_type_uint32: ffi_type;
    pub static mut ffi_type_sint32: ffi_type;
    pub static mut ffi_type_uint64: ffi_type;
    pub static mut ffi_type_sint64: ffi_type;
    pub static mut ffi_type_float: ffi_type;
    pub static mut ffi_type_double: ffi_type;
    pub static mut ffi_type_longdouble: ffi_type;
    pub static mut ffi_type_pointer: ffi_type;

    #[cfg(feature = "complex")]
    pub static mut ffi_type_complex_float: ffi_type;

    #[cfg(feature = "complex")]
    pub static mut ffi_type_complex_double: ffi_type;

    #[cfg(feature = "complex")]
    pub static mut ffi_type_complex_longdouble: ffi_type;

    pub fn ffi_raw_call(
        cif: *mut ffi_cif,
        fn_: Option<unsafe extern "C" fn()>,
        rvalue: *mut c_void,
        avalue: *mut ffi_raw,
    );

    pub fn ffi_ptrarray_to_raw(cif: *mut ffi_cif, args: *mut *mut c_void, raw: *mut ffi_raw);

    pub fn ffi_raw_to_ptrarray(cif: *mut ffi_cif, raw: *mut ffi_raw, args: *mut *mut c_void);

    pub fn ffi_raw_size(cif: *mut ffi_cif) -> usize;

    // See: https://github.com/libffi/libffi/blob/252c0f463641e6100169c3f0a4a590d7df438278/include/ffi.h.in#L302
    #[cfg(not(target_arch = "x86"))]
    #[deprecated = "Deprecated in libffi 3.3"]
    pub fn ffi_java_raw_call(
        cif: *mut ffi_cif,
        fn_: Option<unsafe extern "C" fn()>,
        rvalue: *mut c_void,
        avalue: *mut ffi_java_raw,
    );

    #[deprecated = "Deprecated in libffi 3.3"]
    pub fn ffi_java_ptrarray_to_raw(
        cif: *mut ffi_cif,
        args: *mut *mut c_void,
        raw: *mut ffi_java_raw,
    );

    #[deprecated = "Deprecated in libffi 3.3"]
    pub fn ffi_java_raw_to_ptrarray(
        cif: *mut ffi_cif,
        raw: *mut ffi_java_raw,
        args: *mut *mut c_void,
    );

    #[deprecated = "Deprecated in libffi 3.3"]
    pub fn ffi_java_raw_size(cif: *mut ffi_cif) -> usize;

    pub fn ffi_closure_alloc(size: usize, code: *mut *mut c_void) -> *mut c_void;

    pub fn ffi_closure_free(arg1: *mut c_void);

    #[deprecated = "Deprecated in libffi 3.3, use `ffi_prep_closure_loc` instead"]
    pub fn ffi_prep_closure(
        arg1: *mut ffi_closure,
        arg2: *mut ffi_cif,
        fun: Option<
            unsafe extern "C" fn(
                arg1: *mut ffi_cif,
                arg2: *mut c_void,
                arg3: *mut *mut c_void,
                arg4: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
    ) -> ffi_status;

    pub fn ffi_prep_closure_loc(
        arg1: *mut ffi_closure,
        arg2: *mut ffi_cif,
        fun: Option<
            unsafe extern "C" fn(
                arg1: *mut ffi_cif,
                arg2: *mut c_void,
                arg3: *mut *mut c_void,
                arg4: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
        codeloc: *mut c_void,
    ) -> ffi_status;

    pub fn ffi_prep_raw_closure(
        arg1: *mut ffi_raw_closure,
        cif: *mut ffi_cif,
        fun: Option<
            unsafe extern "C" fn(
                arg1: *mut ffi_cif,
                arg2: *mut c_void,
                arg3: *mut ffi_raw,
                arg4: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
    ) -> ffi_status;

    pub fn ffi_prep_raw_closure_loc(
        arg1: *mut ffi_raw_closure,
        cif: *mut ffi_cif,
        fun: Option<
            unsafe extern "C" fn(
                arg1: *mut ffi_cif,
                arg2: *mut c_void,
                arg3: *mut ffi_raw,
                arg4: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
        codeloc: *mut c_void,
    ) -> ffi_status;

    // See: https://github.com/libffi/libffi/blob/252c0f463641e6100169c3f0a4a590d7df438278/include/ffi.h.in#L441
    #[cfg(not(target_arch = "x86"))]
    #[deprecated = "Deprecated in libffi 3.3"]
    pub fn ffi_prep_java_raw_closure(
        arg1: *mut ffi_java_raw_closure,
        cif: *mut ffi_cif,
        fun: Option<
            unsafe extern "C" fn(
                arg1: *mut ffi_cif,
                arg2: *mut c_void,
                arg3: *mut ffi_java_raw,
                arg4: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
    ) -> ffi_status;

    // See: https://github.com/libffi/libffi/blob/252c0f463641e6100169c3f0a4a590d7df438278/include/ffi.h.in#L449
    #[cfg(not(target_arch = "x86"))]
    #[deprecated = "Deprecated in libffi 3.3"]
    pub fn ffi_prep_java_raw_closure_loc(
        arg1: *mut ffi_java_raw_closure,
        cif: *mut ffi_cif,
        fun: Option<
            unsafe extern "C" fn(
                arg1: *mut ffi_cif,
                arg2: *mut c_void,
                arg3: *mut ffi_java_raw,
                arg4: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
        codeloc: *mut c_void,
    ) -> ffi_status;

    pub fn ffi_prep_go_closure(
        arg1: *mut ffi_go_closure,
        arg2: *mut ffi_cif,
        fun: Option<
            unsafe extern "C" fn(
                arg1: *mut ffi_cif,
                arg2: *mut c_void,
                arg3: *mut *mut c_void,
                arg4: *mut c_void,
            ),
        >,
    ) -> ffi_status;

    pub fn ffi_call_go(
        cif: *mut ffi_cif,
        fn_: Option<unsafe extern "C" fn()>,
        rvalue: *mut c_void,
        avalue: *mut *mut c_void,
        closure: *mut c_void,
    );

    pub fn ffi_prep_cif(
        cif: *mut ffi_cif,
        abi: ffi_abi,
        nargs: c_uint,
        rtype: *mut ffi_type,
        atypes: *mut *mut ffi_type,
    ) -> ffi_status;

    pub fn ffi_prep_cif_var(
        cif: *mut ffi_cif,
        abi: ffi_abi,
        nfixedargs: c_uint,
        ntotalargs: c_uint,
        rtype: *mut ffi_type,
        atypes: *mut *mut ffi_type,
    ) -> ffi_status;

    pub fn ffi_call(
        cif: *mut ffi_cif,
        fn_: Option<unsafe extern "C" fn()>,
        rvalue: *mut c_void,
        avalue: *mut *mut c_void,
    );

    pub fn ffi_get_struct_offsets(
        abi: ffi_abi,
        struct_type: *mut ffi_type,
        offsets: *mut usize,
    ) -> ffi_status;
}

#[cfg(test)]
mod test {
    use std::{mem::transmute, ptr::addr_of_mut};

    use super::*;

    extern "C" fn cast_u8_u32(x: u8) -> u32 {
        x as u32
    }

    #[test]
    fn test_function_sign_extension() {
        unsafe {
            let mut cif: ffi_cif = Default::default();
            let mut arg_types: Vec<*mut ffi_type> = vec![addr_of_mut!(ffi_type_uint8)];

            let prep_status = ffi_prep_cif(
                &mut cif,
                ffi_abi_FFI_DEFAULT_ABI,
                1,
                addr_of_mut!(ffi_type_uint8),
                arg_types.as_mut_ptr(),
            );

            assert_eq!(prep_status, ffi_status_FFI_OK);

            let mut rval: ffi_arg = 0;
            let func = transmute::<extern "C" fn(u8) -> u32, extern "C" fn()>(cast_u8_u32);

            ffi_call(
                &mut cif,
                Some(func),
                &mut rval as *mut _ as *mut c_void,
                vec![&mut 256 as *mut _ as *mut c_void].as_mut_ptr(),
            );

            assert_eq!(rval, 0);
        }
    }

    extern "C" fn add(x: u64, y: u64) -> u64 {
        x + y
    }

    #[test]
    fn test_function_with_two_arguments() {
        unsafe {
            let mut cif: ffi_cif = Default::default();
            let mut arg_types: Vec<*mut ffi_type> =
                vec![addr_of_mut!(ffi_type_uint64), addr_of_mut!(ffi_type_uint64)];

            let prep_status = ffi_prep_cif(
                &mut cif,
                ffi_abi_FFI_DEFAULT_ABI,
                2,
                addr_of_mut!(ffi_type_uint64),
                arg_types.as_mut_ptr(),
            );

            assert_eq!(prep_status, ffi_status_FFI_OK);

            let mut rval = 0u64;
            let func = transmute::<extern "C" fn(u64, u64) -> u64, extern "C" fn()>(add);

            ffi_call(
                &mut cif,
                Some(func),
                &mut rval as *mut _ as *mut c_void,
                vec![
                    &mut 4u64 as *mut _ as *mut c_void,
                    &mut 5u64 as *mut _ as *mut c_void,
                ]
                .as_mut_ptr(),
            );

            assert_eq!(rval, 9);
        }
    }
}
