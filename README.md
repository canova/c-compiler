# Toy C compiler

The goal of this project is not to write a complete C compiler. The goal is to learn ARM architecture and ARM assembly. I will not be writing compiler optimizations etc. This is only a tool for learning.

I've been using [Writing a C compiler series](https://norasandler.com/2017/11/29/Write-a-Compiler.html) to implement this, which uses x86 assembly. I'm converting that with ARM assembly.

## Notes

Output assembly using gcc:

```sh
gcc -S -O3 -fno-asynchronous-unwind-tables <file>
```

Compile assembly to executable:

```sh
# Create object files from the assembly.
as -o file.o file.s

# Create the binary with the linker.
ld -macosx_version_min 13.0.0 -o file file.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _main -arch arm64
```
