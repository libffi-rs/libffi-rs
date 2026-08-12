/*
 * C2Rust lowers C alloca() to a heap Vec.  The upstream unix64 entry steals
 * its caller's alloca block as the callee's machine stack, so copy the block
 * to an actual dynamically-sized stack allocation first.  This remains
 * 64-bit machine code under the x32 ILP32 ABI; pointer/size arguments occupy
 * the standard x86-64 argument registers with their low 32 bits significant.
 */
#define LIBFFI_ASM
#include "fficonfig.h"
#include "ffi.h"
#include "asmnames.h"

        .text
        .balign 16
        .globl  C(ffi_call_unix64_from_rust)
        FFI_HIDDEN(C(ffi_call_unix64_from_rust))
C(ffi_call_unix64_from_rust):
        _CET_ENDBR
        pushq   %rbp
        movq    %rsp, %rbp

        /* Zero-extend x32 unsigned long BYTES before 64-bit stack arithmetic. */
        movl    %esi, %eax
        addq    $47, %rax
        andq    $-16, %rax
        subq    %rax, %rsp
        movq    %rsp, %r9

        xorl    %eax, %eax
L(copy_args):
        cmpl    %esi, %eax
        jae     L(copy_done)
        movq    (%rdi,%rax), %r10
        movq    %r10, (%r9,%rax)
        addl    $8, %eax
        jmp     L(copy_args)
L(copy_done):
        movq    %r9, %rdi
        call    C(ffi_call_unix64)

        movq    %rbp, %rsp
        popq    %rbp
        ret

#if defined __ELF__ && defined __linux__
        .section .note.GNU-stack,"",@progbits
#endif
