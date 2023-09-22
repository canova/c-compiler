; int __attribute__((noinline)) square(int num) {
;    return num * num;
; }
;
; int main() {
;   int a = square(12);
;   return a * 1231;
; }

	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 13, 0	sdk_version 13, 3
	.globl	_square                         ; -- Begin function square
	.p2align	2
_square:                                ; @square
; %bb.0:
	mul	w0, w0, w0
	ret
                                        ; -- End function
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	mov	x29, sp
	mov	w0, #12
	bl	_square
	mov	w8, #1231
	mul	w0, w0, w8
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
                                        ; -- End function
.subsections_via_symbols
