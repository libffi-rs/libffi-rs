#include <stddef.h>
#include "ffi.h"

_Static_assert(sizeof(void *) == 4, "x32 pointers are 32-bit");
_Static_assert(sizeof(size_t) == 4, "x32 size_t is 32-bit");
_Static_assert(sizeof(long) == 4, "x32 long is 32-bit");
_Static_assert(sizeof(ffi_arg) == 8, "libffi x32 argument slot is 64-bit");
_Static_assert(sizeof(long double) == 16, "x87 long double storage");
_Static_assert(_Alignof(long double) == 16, "x87 long double alignment");
_Static_assert(sizeof(_Complex float) == 8, "complex float storage");
_Static_assert(sizeof(_Complex double) == 16, "complex double storage");
_Static_assert(sizeof(_Complex long double) == 32, "complex long double storage");
_Static_assert(sizeof(ffi_type) == 12, "x32 ffi_type layout");
_Static_assert(sizeof(ffi_cif) == 24, "six-field x32 ffi_cif layout");
_Static_assert(offsetof(ffi_cif, flags) == 20, "ffi_cif flags offset");
_Static_assert(FFI_DEFAULT_ABI == FFI_UNIX64, "x32 default ABI is UNIX64");
int abi_layout_x32_is_valid;
