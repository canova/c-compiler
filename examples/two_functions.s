	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 13, 0	sdk_version 13, 3
	.globl	_test                           ; -- Begin function test
	.p2align	2
_test:                                  ; @test
; %bb.0:
	mov	w0, #234
	ret
                                        ; -- End function
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
; %bb.0:
	mov	w0, #3
	ret
                                        ; -- End function
.subsections_via_symbols
