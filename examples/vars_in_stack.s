; int main() {
;     int a = 1;
;     int b = 2;
;     int c = 3;
;     int e = 4;
;     int f = 5;
;     int g = 6;
;     int h = 7;
;     int i = 8;
;     return a + b + c - f * g / h + i;
; }

	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 13, 0	sdk_version 13, 3
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
; %bb.0:
	sub	sp, sp, #48
	str	wzr, [sp, #44]
	mov	w8, #1
	str	w8, [sp, #40]
	mov	w8, #2
	str	w8, [sp, #36]
	mov	w8, #3
	str	w8, [sp, #32]
	mov	w8, #4
	str	w8, [sp, #28]
	mov	w8, #5
	str	w8, [sp, #24]
	mov	w8, #6
	str	w8, [sp, #20]
	mov	w8, #7
	str	w8, [sp, #16]
	mov	w8, #8
	str	w8, [sp, #12]
	ldr	w8, [sp, #40]
	ldr	w9, [sp, #36]
	add	w8, w8, w9
	ldr	w9, [sp, #32]
	add	w8, w8, w9
	ldr	w9, [sp, #24]
	ldr	w10, [sp, #20]
	mul	w9, w9, w10
	ldr	w10, [sp, #16]
	sdiv	w9, w9, w10
	subs	w8, w8, w9
	ldr	w9, [sp, #12]
	add	w0, w8, w9
	add	sp, sp, #48
	ret
                                        ; -- End function
.subsections_via_symbols
