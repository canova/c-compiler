# ARM64 assembly learnings

## Recommended reads

- [A Guide to ARM64 / AArch64 Assembly on Linux with Shellcodes and Cryptography](https://modexp.wordpress.com/2018/10/30/arm64-assembly/)
- [Whirlwind Tour of ARM Assembly](https://www.coranac.com/tonc/text/asm.htm)
- [Introduction to ARM Assembly Basics](https://azeria-labs.com/writing-arm-assembly-part-1/)
- [ARMv8 A64 Quick Reference](./ARMv8_A64_Quick_Reference.pdf)

## Registers

- There are 32 64-bit registers in total: `x0-x31`.
  - There are two addressing modes:
    - 64-bit: `x0`
    - 32-bit: `w0`

  32 version only accesses the lower 32bits of the same register, so they are basically the same register.
- `sp`: stack pointer register.
- `lr`: link register.
- `pc`: program counter/instruction pointer.
- The rest of the registers are general purpose registers: `x0-x28`.
- `xzr`-`wzr`: Zero register.
  `mov w0, wzr` writes zero. Same as `mov w0, #0`.
- TODO: Explain the registers for floats and vectors.
- Move immediate value to a register:

  ```asm
  mov x0, #12 ; Can also be a hex: #0x1F
  ```

### Calling convention

- `x0` to `x7`: Argument values passed to and results returned from a subroutine.
  - The value of these registers may be freely modified by the called function (the callee) so the caller cannot assume anything about their content, even if they are not used in the parameter passing or for the returned value. This means that these registers are in practice caller-saved.
- `x8` (XR): Indirect return value address.
- `x9` to `x15`: Local variables, caller saved.
  - Temporary registers for every function. No assumption can be made on their values upon returning from a function.
- `x16` (IP0) and `x17` (IP1): Intra-Procedure-call scratch registers.
- `x18` (PR): Platform register. Used for some operating-system-specific special purpose, or an additional caller-saved register.
- `x19` to `x28`: Callee-saved registers.
  - If used by a function, must have their values preserved and later restored upon returning to the caller.
- `x29` (FP): Frame pointer.
- `x30` (LR): Procedure link register, used to return from subroutines.
- `x31` (SP): Stack pointer or a zero register, depending on context.

## Data types

- Byte: 8 bits
- Halfword: 16 bits
- Word: 32 bits
- Doubleword: 64 bits
- Quadword: 128 bits

### Data alignment

The alignment of `sp` must be two times the size of a pointer. For AArch64/ARM64 it’s 16 bytes (quadword).

## Memory

- Use `ldr` and `str` with multiple variations and addressing modes.

  ```asm
  ldr x0, [x1] ; See the "Addressing modes" section.
  ```

- Use `ldp` and`stp` to load/store two registers at once.

### Addressing modes

```asm
  ; load the address of the first element of list variable to x1.
  ldr x1, =list

  ; load the content of first element at the x1 address to x2
  ; like `x2 = list[x1]`
  ldr x2, [x1]

  ; load the content of x1+8 address to x3. +8 is the next element .because it's 64 bit.
  ; like `x3 = list[x1+1]`
  ldr x3, [x1, #8]

  ; Pre-increment the register and load the contents of it.
  ; like `x4 = list[++x1]`
  ldr x4, [x1, #8]!

  ; Load the contents of the x1 address and post-increment the register.
  ; like `x5 = list[x1++]`
  ldr x5, [x1], #8

.data
list:
  .dword 1,2,3,4
```

## Stack

Use stack pointer `SP` to push and pop things to/from the stack. `str` (store single) and `stp` (store pair) instructions can be used to write to the stack. `ldr` (load single) and `ldp` (load pair) instructions can be used to read from the stack.

### Push 8 byte(64 bit) register to stack

```asm
; store content of `x3` into `[sp - #8]` and decrement `sp` by 8
str x3, [sp, #-8]! 
```

### Pop 8 byte(64 bit) register to stack

```asm
ldr x3, [sp], #8
```

### `SP` alignment

**IMPORTANT NOTE**: `SP` has to be 16 byte aligned in ARM64!
That's why the example above is not really great! We have to align it by 16 instead of 8.

Here's a better way:
You can use `stp` and `ldp` to write and read 2 registers from the stack.

```asm
stp w1, w2, [sp, #-16]!
; or if you want to write only one (with zero register)
stp w1, wzr, [sp, #-16]!
```

But as you can see, if you want to write a single register to the stack only, it's not really an efficient since you will be always adding a 8 bytes of padding.

Instead, there is a better way. Compute the total amount of stack space you will need for local variables first, and roll that up to multiples of 16. Then, allocate that space in the function prologue.

Here's an example

```asm
func:
  sub sp, sp, #48
  str w0, [sp, #44]
  str w1, [sp, #40]
  ...
```

## Register spilling

TODO

## Instructions

The following sections only mention the important instructions I used. They are just a subset of all the instructions and definitely not complete. I will expand these sections as I learn more.

### Logical operations

```asm
and x1, x2, x3
orr x1, x2, x3
mvn X1, X2 ; NOT
```

### Arithmetic operations

TODO

### Logical shift and rotation

- `LSL` -> Logical shift to the left - multiples the number by 2.
- `LSR` -> Logical shift to the right - dividing the number by 2
- `ROR` -> Rotation to right (not very common, only in hashing and crypto etc.)

```asm
lsl x1, #1 ; shift left 1 time

mov x1, x0, LSL #1 ; move the x0 to x1 and do a shift at the same time
```

### Conditionals and branching

TODO

### Bit manipulations
