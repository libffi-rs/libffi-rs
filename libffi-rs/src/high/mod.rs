//! High module

use crate::low::CodePtr;
use crate::middle::{arg, Cif, Type};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ffi::c_void;

trait DeriveType {
    fn derive_type() -> Type where Self: Sized;
    fn derive_type_self(&self) -> Type;
}

macro_rules! derive_type {
    ($($t:ty => $ty:ident),*) => {
        $(
            impl DeriveType for $t {
                fn derive_type() -> Type {
                    Type::$ty()
                }

                fn derive_type_self(&self) -> Type {
                    Self::derive_type()
                }
            }
        )*
    };
}

derive_type!(
    () => void,
    bool => u8
);

macro_rules! derive_num_type {
    ($($t:ident),*) => {
        $(
            impl DeriveType for $t {
                fn derive_type() -> Type where Self: Sized {
                    Type::$t()
                }

                fn derive_type_self(&self) -> Type {
                    Self::derive_type()
                }
            }
        )*
    };
}

derive_num_type!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

unsafe fn call<R: DeriveType>(function: *mut c_void, arguments: Vec<Box<dyn DeriveType>>) -> R {
    let ty = arguments.iter().map(|arg| arg.derive_type_self()).collect::<Vec<_>>();
    let cif = Cif::new(
        ty,
        R::derive_type(),
    );
    let args: Vec<_> = arguments.iter().map(|item| arg(item)).collect();

    let result: R = unsafe {
        cif.call(
            CodePtr(function),
            &args,
        )
    };
    result
}

mod tests {
    use super::*;
    use core::ffi::c_int;
    use alloc::vec::Vec;

    #[test]
    fn test_call() {
        extern "C" fn mul(x: c_int, y: c_int) -> c_int {
            x * y
        }

        let mut arguments: Vec<Box<dyn DeriveType>> = Vec::new();
        arguments.push(Box::new(5i32));
        arguments.push(Box::new(6i32));

        let result: c_int = unsafe {
            call(
                mul as *mut c_void,
                arguments,
            )
        };
        assert_eq!(result, 30);
    }
}
