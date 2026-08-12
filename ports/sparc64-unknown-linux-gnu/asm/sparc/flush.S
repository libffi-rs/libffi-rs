/* C2Rust cannot lower the GCC SPARC inline-asm template from ffi64.c.
   This target assembly helper is the exact two-flush operation, with the
   ordinary SPARC V9 return sequence around it. */
	.text
	.align 4
	.global ffi_flush_icache
	.hidden ffi_flush_icache
	.type ffi_flush_icache, #function
ffi_flush_icache:
	flush %o0
	flush %o0+8
	retl
	 nop
	.size ffi_flush_icache, .-ffi_flush_icache
