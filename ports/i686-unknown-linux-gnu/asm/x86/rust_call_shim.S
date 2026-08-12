/*
 * C2Rust lowers C alloca() to a heap Vec.  ffi_call_i386 expects its argument
 * block on the machine stack because it installs that block as ESP.  Copy the
 * translated block (arguments plus struct call_frame) into a real dynamic
 * stack allocation, then recover this shim's frame independently of ESP.
 */
	.text
	.balign 16
	.globl ffi_call_i386_from_rust
	.type ffi_call_i386_from_rust,@function
ffi_call_i386_from_rust:
	pushl %ebp
	movl %esp, %ebp
	pushl %esi
	pushl %edi

	/* cdecl inputs: frame, stack.  Their difference is the argument size. */
	movl 8(%ebp), %eax
	subl 12(%ebp), %eax

	/* Reserve/copy arguments plus the 32-byte call_frame, 16-byte aligned. */
	leal 47(%eax), %ecx
	andl $-16, %ecx
	subl %ecx, %esp
	movl %esp, %edi
	movl 12(%ebp), %esi
	leal 32(%eax), %ecx
	rep movsb

	/* Upstream fastcall ABI: frame in ECX, outgoing stack in EDX. */
	leal (%esp,%eax), %ecx
	movl %esp, %edx
	call ffi_call_i386

	/* Upstream intentionally deallocates its caller's alloca. */
	leal -8(%ebp), %esp
	popl %edi
	popl %esi
	popl %ebp
	ret
	.size ffi_call_i386_from_rust, .-ffi_call_i386_from_rust
	.section .note.GNU-stack,"",@progbits
