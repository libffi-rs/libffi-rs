use core::ffi::{c_uint, c_ushort};

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

#[test]
fn powerpc64le_public_layouts() {
    assert_eq!(core::mem::size_of::<FfiType>(), 24);
    assert_eq!(core::mem::align_of::<FfiType>(), 8);
    assert_eq!(core::mem::size_of::<FfiCif>(), 40);
    assert_eq!(core::mem::align_of::<FfiCif>(), 8);
}
