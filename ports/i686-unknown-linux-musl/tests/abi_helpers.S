/* Test-only i686 SysV helper. A C long double is passed at 4(%esp) and
   returned in x87 st(0). */
.text
.balign 16
.globl ffi_test_longdouble_add_one
.type ffi_test_longdouble_add_one,@function
ffi_test_longdouble_add_one:
    fldt 4(%esp)
    fld1
    faddp
    ret
.size ffi_test_longdouble_add_one,.-ffi_test_longdouble_add_one
.section .note.GNU-stack,"",@progbits
