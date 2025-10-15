//! This example demonstrates how to use libffi-rs with libloading to call a function
//! from a shared library at runtime. It loads the C standard library and calls the
//! `isdigit` function of libc to check if a character is a digit.
//!
//! This example uses the `middle` level API because the high-level API does not
//! currently support this use case.
//!
//! Note: This example is platform-dependent and works on macOS, Linux, and Windows.

use libffi::high::CodePtr;
use libffi::middle::{arg, Cif, Type};
use std::ffi::c_void;

#[cfg(target_os = "macos")]
const FILE: &str = "/usr/lib/libSystem.B.dylib";
#[cfg(target_os = "linux")]
const FILE: &str = "/lib/x86_64-linux-gnu/libc.so.6";
#[cfg(target_os = "windows")]
const FILE: &str = r"C:\Windows\System32\msvcrt.dll";

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn main() {
    println!("This example is only supported on macOS, Linux, and Windows.");
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
fn main() {
    // Parameters - Feel free to change these to call other functions
    // from the C standard library!
    let func_name = "isdigit";
    let cif = Cif::new(vec![Type::i32()], Type::i32());
    let args1 = [arg(&('1' as i32))];
    let args2 = [arg(&('a' as i32))];
    // None of what we're doing here is safe
    unsafe {
        let lib = libloading::Library::new(FILE).unwrap();

        // Libloading wants a CString because it is null-terminated
        let converted_name = std::ffi::CString::new(func_name).unwrap();
        // This signature doesn't matter, we're just going to get the raw pointer out of it anyhow
        let func: libloading::Symbol<fn() -> c_void> = lib.get(converted_name.as_bytes()).unwrap();
        // Get the raw pointer to the function
        let func = func.into_raw().as_raw_ptr();

        // Now we proceed as normal with libffi-rs
        let result: i32 = cif.call(CodePtr(func as *mut _), &args1);
        assert_eq!(result, 1);
        let result: i32 = cif.call(CodePtr(func as *mut _), &args2);
        assert_eq!(result, 0);
    }
}
