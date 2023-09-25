# Toy C compiler - WIP

The goal is to learn ARM architecture and ARM assembly, not to write a feature complete C compiler. Probably, I will not be writing compiler optimizations etc. This is jus a compiler that I'm writing for learning.

I've been reading [Writing a C compiler series](https://norasandler.com/2017/11/29/Write-a-Compiler.html) by Nora Sandler to implement this, which uses 32-bit x86 assembly. I'm converting that to ARM64 assembly along the way.

## Build and run

```sh
cargo run -- <file-path>
```

You can also use the `--dry-run` argument to only print the assembly to stdout without saving/compiling the assembly to file.
