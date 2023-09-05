#!/bin/bash

# Exit with an error if any command exits with an error.
set -e

gcc_asm() {
  echo "Generating assembly for $@"
  gcc -S -O3 -fno-asynchronous-unwind-tables "$@"
  basename="$(basename $@)"
  bat "${basename%.*}.s"
}

compile_asm() {
  echo "Compiling $@"
  as -o output.o "$@"
  ld -macosx_version_min 13.0.0 -o output output.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _main -arch arm64
  basename="$(basename $@)"
}


command="$1"
# Remove the command argument.
shift 1

if [ "$command" == "gcc_asm" ]
then
  gcc_asm "$@"
elif [ $command == "compile_asm" ]
then
  compile_asm "$@"
elif [ $command == "run_asm" ]
then
  compile_asm "$@"
  ./output
else
    echo "Invalid command"
fi
