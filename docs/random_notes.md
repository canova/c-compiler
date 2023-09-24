# Random Notes

## Output assembly using gcc

```sh
gcc -S -O3 -fno-asynchronous-unwind-tables <file>
```

Play with the optimization flag to see what's changing.

## Output assembly using clang

Similarly:

```sh
clang -S -O3 -fno-asynchronous-unwind-tables <file>
```

### Compile assembly to executable

```sh
# Create object files from the assembly.
as -o file.o file.s

# Create the binary with the linker.
ld -macosx_version_min 13.0.0 -o file file.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _main -arch arm64
```

## Compiler Explorer Example

Compiler explorer is super useful for looking at the outputs of other compilers. [Here's a clang example.](https://godbolt.org/z/Eavz3s1fb)
