/*
 * C2Rust lowers C alloca() to Vec<u8>, but LoongArch ffi_call_asm switches SP
 * to the alloca region.  Move the translated heap frame to a genuine C stack
 * allocation for the duration of the unmodified upstream assembly call.
 */
#include <ffi.h>
#include <stddef.h>
#include <string.h>

extern void ffi_call_asm(void *stack, void *regs, void (*fn)(void), void *closure);

void ffi_call_asm_stack_bridge(void *heap_frame, size_t context_offset,
                               size_t total_size, void (*fn)(void),
                               void *closure)
{
    unsigned char *stack_frame = __builtin_alloca(total_size);
    memcpy(stack_frame, heap_frame, total_size);
    ffi_call_asm(stack_frame, stack_frame + context_offset, fn, closure);
    memcpy((unsigned char *)heap_frame + context_offset,
           stack_frame + context_offset, total_size - context_offset);
}
