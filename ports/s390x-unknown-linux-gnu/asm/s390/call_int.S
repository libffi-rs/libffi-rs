	.file	"ffi.c"
	.machinemode zarch
	.machine "z13"
.text
	.align	8
	.align	16
	.type	ffi_check_struct_type, @function
ffi_check_struct_type:
.LFB14:
	.cfi_startproc
	lg	%r4,0(%r2)
	j	.L3
.L19:
	lg	%r1,16(%r2)
	ltg	%r2,0(%r1)
	je	.L2
	ltg	%r1,8(%r1)
	jne	.L2
.L3:
	chhsi	10(%r2),13
	llh	%r3,10(%r2)
	je	.L19
.L2:
	aghik	%r0,%r4,-1
	clgijle	%r0,7,.L20
.L4:
	lhi	%r3,14
	risbg	%r2,%r3,60,128+63,0
	br	%r14
.L20:
	clijh	%r0,7,.L4
	risbgn	%r5,%r0,29,128+60,3
	larl	%r2,.L6
	lg	%r4,0(%r5,%r2)
	b	0(%r4,%r2)
	.section	.rodata
	.align	8
	.align	4
.L6:
	.quad	.L9-.L6
	.quad	.L10-.L6
	.quad	.L4-.L6
	.quad	.L7-.L6
	.quad	.L4-.L6
	.quad	.L4-.L6
	.quad	.L4-.L6
	.quad	.L5-.L6
.text
.L5:
	chi	%r3,3
	lhi	%r3,3
	lochine	%r3,11
	risbg	%r2,%r3,60,128+63,0
	br	%r14
.L7:
	chi	%r3,2
	lhi	%r3,2
	lochine	%r3,9
	risbg	%r2,%r3,60,128+63,0
	br	%r14
.L10:
	lhi	%r3,7
	risbg	%r2,%r3,60,128+63,0
	br	%r14
.L9:
	lhi	%r3,5
	risbg	%r2,%r3,60,128+63,0
	br	%r14
	.cfi_endproc
.LFE14:
	.size	ffi_check_struct_type, .-ffi_check_struct_type
	.align	8
	.align	16
	.globl	ffi_call_int
	.hidden	ffi_call_int
	.type	ffi_call_int, @function
ffi_call_int:
.LFB16:
	.cfi_startproc
	stmg	%r6,%r15,48(%r15)
	.cfi_offset 6, -112
	.cfi_offset 7, -104
	.cfi_offset 8, -96
	.cfi_offset 9, -88
	.cfi_offset 10, -80
	.cfi_offset 11, -72
	.cfi_offset 12, -64
	.cfi_offset 13, -56
	.cfi_offset 14, -48
	.cfi_offset 15, -40
	lgr	%r14,%r15
	ear	%r8,%a0
	lay	%r15,-272(%r15)
	.cfi_def_cfa_offset 432
	aghi	%r14,-40
	std	%f8,0(%r14)
	std	%f10,8(%r14)
	std	%f12,16(%r14)
	std	%f14,24(%r14)
	std	%f9,32(%r14)
	sllg	%r8,%r8,32
	ear	%r8,%a1
	lgr	%r11,%r15
	.cfi_def_cfa_register 11
	.cfi_offset 24, -200
	.cfi_offset 25, -192
	.cfi_offset 26, -184
	.cfi_offset 27, -176
	.cfi_offset 28, -168
	mvc	224(8,%r11),40(%r8)
	l	%r9,28(%r2)
	llgf	%r1,24(%r2)
	ldgr	%f10,%r3
	ldgr	%f8,%r6
	risbg	%r10,%r9,60,128+60,0
	cgije	%r4,0,.L23
	aghik	%r3,%r1,167
	aghik	%r14,%r1,160
	risbg	%r7,%r3,31,128+51,0
	risbg	%r0,%r3,31,128+60,0
	sgrk	%r6,%r15,%r7
	cgrje	%r15,%r6,.L25
.L81:
	lay	%r15,-4096(%r15)
	cg	%r0,4088(%r15)
	cgrjlh	%r15,%r6,.L81
.L25:
	risbg	%r13,%r0,52,128+63,0
	sgr	%r15,%r13
	cgijlh	%r13,0,.L82
.L26:
	clgr	%r1,%r14
	lgr	%r3,%r14
	locgrhe	%r3,%r1
	sgr	%r3,%r1
	stg	%r3,168(%r11)
	la	%r7,160(%r15)
	stg	%r7,208(%r11)
	la	%r0,272(%r11)
	stg	%r0,160(%r1,%r15)
	la	%r1,0(%r1,%r7)
	la	%r6,16(%r1)
	stg	%r6,200(%r11)
	la	%r13,128(%r1)
	risbg	%r3,%r9,61,128+63,0
	ldgr	%f12,%r13
	cijlh	%r10,0,.L27
.L80:
	lhi	%r13,0
.L28:
	lg	%r9,8(%r2)
	lt	%r2,4(%r2)
	jle	.L59
	stg	%r14,160(%r11)
	st	%r3,192(%r11)
	stg	%r1,184(%r11)
	mvhi	196(%r11),0
	stg	%r8,176(%r11)
	ahi	%r2,-1
	lgr	%r10,%r5
	llgfr	%r0,%r2
	lhi	%r5,0
	aghik	%r6,%r0,1
	ldgr	%f14,%r1
	lr	%r7,%r13
	ldgr	%f9,%r4
	lr	%r8,%r5
.L58:
	lg	%r4,0(%r9)
	lg	%r13,0(%r10)
	llh	%r0,10(%r4)
	stg	%r4,216(%r11)
.L38:
	ahi	%r0,-1
	clijh	%r0,16,.L39
	risbgn	%r1,%r0,29,128+60,3
	larl	%r14,.L41
	lg	%r3,0(%r14,%r1)
	b	0(%r14,%r3)
	.section	.rodata
	.align	8
	.align	4
.L41:
	.quad	.L45-.L41
	.quad	.L52-.L41
	.quad	.L51-.L41
	.quad	.L40-.L41
	.quad	.L50-.L41
	.quad	.L49-.L41
	.quad	.L48-.L41
	.quad	.L47-.L41
	.quad	.L46-.L41
	.quad	.L45-.L41
	.quad	.L42-.L41
	.quad	.L42-.L41
	.quad	.L43-.L41
	.quad	.L42-.L41
	.quad	.L40-.L41
	.quad	.L40-.L41
	.quad	.L40-.L41
.text
.L43:
	lg	%r2,216(%r11)
	brasl	%r14,ffi_check_struct_type@PLT
	lr	%r0,%r2
	cijlh	%r2,14,.L38
.L40:
	lg	%r2,216(%r11)
	lg	%r1,160(%r11)
	lg	%r4,0(%r2)
	lgr	%r3,%r13
	lg	%r13,168(%r11)
	aghik	%r5,%r4,15
	nill	%r5,65520
	clgr	%r13,%r1
	locgrhe	%r1,%r13
	sgrk	%r0,%r1,%r13
	lgdr	%r2,%f14
	sgr	%r0,%r5
	clgr	%r0,%r1
	locgrnhe	%r0,%r1
	sgr	%r2,%r5
	sgrk	%r14,%r13,%r1
	ldgr	%f14,%r2
	agr	%r14,%r5
	agrk	%r5,%r0,%r14
	stg	%r5,168(%r11)
	brasl	%r14,__memcpy_chk@PLT
	lgdr	%r4,%f14
.L53:
	cijh	%r7,4,.L54
.L84:
	lgfr	%r1,%r7
	ahi	%r7,1
	sllg	%r3,%r1,3
	ag	%r3,200(%r11)
.L55:
	stg	%r4,0(%r3)
.L39:
	aghi	%r9,8
	aghi	%r10,8
	brctg	%r6,.L58
	l	%r3,192(%r11)
	lg	%r1,184(%r11)
	lgdr	%r4,%f9
	lg	%r8,176(%r11)
.L59:
	lgdr	%r6,%f8
	lgdr	%r5,%f10
	risbg	%r3,%r3,61,128+63,0
	lgr	%r2,%r1
	brasl	%r14,ffi_call_SYSV@PLT
	clc	224(8,%r11),40(%r8)
	jne	.L83
	ld	%f8,232(%r11)
	ld	%f10,240(%r11)
	ld	%f12,248(%r11)
	ld	%f14,256(%r11)
	ld	%f9,264(%r11)
	lmg	%r6,%r15,320(%r11)
	.cfi_remember_state
	.cfi_restore 15
	.cfi_restore 14
	.cfi_restore 13
	.cfi_restore 12
	.cfi_restore 11
	.cfi_restore 10
	.cfi_restore 9
	.cfi_restore 8
	.cfi_restore 7
	.cfi_restore 6
	.cfi_restore 28
	.cfi_restore 27
	.cfi_restore 26
	.cfi_restore 25
	.cfi_restore 24
	.cfi_def_cfa 15, 160
	br	%r14
.L42:
	.cfi_restore_state
	lg	%r4,0(%r13)
	cijle	%r7,4,.L84
.L54:
	lgfr	%r2,%r8
	ahi	%r8,1
	sllg	%r3,%r2,3
	ag	%r3,208(%r11)
	j	.L55
.L45:
	lgf	%r4,0(%r13)
	j	.L53
.L52:
	l	%r2,196(%r11)
	llgf	%r13,0(%r13)
	cijh	%r2,3,.L57
	sllg	%r0,%r13,32
	lgdr	%r13,%f12
	asi	196(%r11),1
	lgfr	%r5,%r2
	sllg	%r4,%r5,3
	stg	%r0,0(%r4,%r13)
	j	.L39
.L51:
	l	%r3,196(%r11)
	lg	%r2,0(%r13)
	cijh	%r3,3,.L56
	lgdr	%r4,%f12
	asi	196(%r11),1
	lgfr	%r0,%r3
	sllg	%r5,%r0,3
	stg	%r2,0(%r5,%r4)
	j	.L39
.L46:
	llgf	%r4,0(%r13)
	j	.L53
.L47:
	lgh	%r4,0(%r13)
	j	.L53
.L48:
	llgh	%r4,0(%r13)
	j	.L53
.L49:
	lgb	%r4,0(%r13)
	j	.L53
.L50:
	llgc	%r4,0(%r13)
	j	.L53
.L57:
	lg	%r14,208(%r11)
	lgfr	%r3,%r8
	ahi	%r8,1
	sllg	%r1,%r3,3
	stg	%r13,0(%r1,%r14)
	j	.L39
.L56:
	lg	%r14,208(%r11)
	lgfr	%r13,%r8
	ahi	%r8,1
	sllg	%r1,%r13,3
	stg	%r2,0(%r1,%r14)
	j	.L39
.L23:
	cije	%r10,0,.L85
	lg	%r14,16(%r2)
	lg	%r3,0(%r14)
	agrk	%r0,%r1,%r3
	aghik	%r13,%r0,167
	aghik	%r14,%r0,160
	risbg	%r10,%r13,0,128+51,0
	nill	%r13,65528
	sgrk	%r7,%r15,%r10
	cgrje	%r15,%r7,.L34
.L86:
	lay	%r15,-4096(%r15)
	cg	%r0,4088(%r15)
	cgrjlh	%r15,%r7,.L86
.L34:
	risbg	%r6,%r13,52,128+63,0
	sgr	%r15,%r6
	cgije	%r6,0,.L35
	cg	%r0,-8(%r6,%r15)
.L35:
	clgr	%r14,%r1
	lgr	%r0,%r1
	locgrhe	%r0,%r14
	la	%r13,160(%r15)
	stg	%r13,208(%r11)
	sgr	%r0,%r1
	stg	%r0,168(%r11)
	agr	%r1,%r13
	cgije	%r3,0,.L36
	la	%r4,160(%r1)
	lgr	%r3,%r4
.L36:
	la	%r10,272(%r11)
	stg	%r10,0(%r1)
	la	%r7,16(%r1)
	stg	%r7,200(%r11)
	la	%r6,128(%r1)
	ldgr	%f12,%r6
.L60:
	stg	%r3,16(%r1)
	lhi	%r13,1
	risbg	%r3,%r9,61,128+63,0
	j	.L28
.L85:
	aghik	%r9,%r1,167
	aghik	%r14,%r1,160
	risbg	%r10,%r9,31,128+51,0
	risbg	%r7,%r9,31,128+60,0
	sgrk	%r3,%r15,%r10
	cgrje	%r15,%r3,.L31
.L87:
	lay	%r15,-4096(%r15)
	cg	%r0,4088(%r15)
	cgrjlh	%r15,%r3,.L87
.L31:
	risbg	%r6,%r7,52,128+63,0
	sgr	%r15,%r6
	cgije	%r6,0,.L32
	cg	%r0,-8(%r6,%r15)
.L32:
	clgr	%r1,%r14
	lgr	%r13,%r14
	locgrhe	%r13,%r1
	la	%r0,160(%r15)
	sgr	%r13,%r1
	lgr	%r9,%r0
	la	%r10,272(%r11)
	stg	%r0,208(%r11)
	stg	%r10,0(%r1,%r9)
	stg	%r13,168(%r11)
	la	%r1,0(%r1,%r9)
	lhi	%r3,4
	la	%r7,16(%r1)
	la	%r6,128(%r1)
	stg	%r7,200(%r11)
	ldgr	%f12,%r6
	j	.L80
.L82:
	cg	%r0,-8(%r13,%r15)
	j	.L26
.L27:
	lgr	%r3,%r4
	j	.L60
.L83:
	brasl	%r14,__stack_chk_fail@PLT
	.cfi_endproc
.LFE16:
	.size	ffi_call_int, .-ffi_call_int
