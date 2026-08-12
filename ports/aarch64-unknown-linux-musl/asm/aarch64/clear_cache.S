.text
.align 2
.globl ffi_rs_clear_cache
.hidden ffi_rs_clear_cache
.type ffi_rs_clear_cache, %function
ffi_rs_clear_cache:
    mrs x2, ctr_el0
    tbnz x2, #28, 2f
    ubfx x3, x2, #16, #4
    mov x4, #4
    lsl x3, x4, x3
    neg x4, x3
    and x4, x4, x0
    cmp x4, x1
    b.hs 2f
1:
    dc cvau, x4
    add x4, x4, x3
    cmp x4, x1
    b.lo 1b
2:
    dsb ish
    tbnz x2, #29, 4f
    and x3, x2, #0xf
    mov x4, #4
    lsl x3, x4, x3
    neg x4, x3
    and x4, x4, x0
    cmp x4, x1
    b.hs 3f
1:
    ic ivau, x4
    add x4, x4, x3
    cmp x4, x1
    b.lo 1b
3:
    dsb ish
4:
    isb
    ret
.size ffi_rs_clear_cache, .-ffi_rs_clear_cache
.section .note.GNU-stack,"",%progbits
