# Toy C compiler - WIP

The goal is to learn ARM architecture and ARM assembly, not to write a feature complete C compiler. Probably, I will not be writing compiler optimizations etc. This is jus a compiler that I'm writing for learning.

I've been reading [Writing a C compiler series](https://norasandler.com/2017/11/29/Write-a-Compiler.html) by Nora Sandler to implement this, which uses 32-bit x86 assembly. I'm converting that to ARM64 assembly along the way.

## Build and run

```sh
cargo run -- <file-path>
```

You can also use the `--dry-run` argument to only print the assembly to stdout without saving/compiling the assembly to file.

## Current status

It's still under heavy development. Things that are currently working:

- `int` variable type
- Unary operators: `~`, `!`, `-`
- Binary arithmetic operators
- Local variables and the assignment operator
- `if`/`else` statements and ternary conditional expressions.
- Compound Statements
- Basic support for code blocks
- `while` and `do-while` loops.
- `break` and `continue`.

### Next steps

- Implement the for loop
- Better block support with variable shadowing
- Implement static strings
- Implement other data types.
- Implement functions and function calls
- Implement global variables
- Split semantic analysis step from codegen step.
- And more...
