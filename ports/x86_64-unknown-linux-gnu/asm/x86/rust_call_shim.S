/*
 * C2Rust lowers C alloca() to a heap Vec.  ffi_call_unix64 expects its args
 * block to be on the machine stack because it installs that block as the
 * target function's stack.  Copy the translated heap block into a real,
 * dynamically-sized stack allocation before entering upstream's assembly.
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

        /* Reserve ARGS bytes plus ffi_call_unix64's 32-byte local record. */
        movq    %rsi, %rax
        addq    $47, %rax
        andq    $-16, %rax
        subq    %rax, %rsp
        movq    %rsp, %r9

        /* ARGS is always a multiple of eight for the x86-64 backend. */
        xorl    %eax, %eax
L(copy_args):
        cmpq    %rsi, %rax
        jae     L(copy_done)
        movq    (%rdi,%rax), %r10
        movq    %r10, (%r9,%rax)
        addq    $8, %rax
        jmp     L(copy_args)
L(copy_done):
        movq    %r9, %rdi
        call    C(ffi_call_unix64)

        /* Upstream assembly intentionally deallocates its caller's alloca.
           Recover this shim's frame independent of the resulting %rsp. */
        movq    %rbp, %rsp
        popq    %rbp
        ret

#if defined __ELF__ && defined __linux__
        .section .note.GNU-stack,"",@progbits
#endif
