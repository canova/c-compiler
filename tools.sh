#!/bin/bash

# Exit with an error if any command exits with an error.
set -e

OBJDIR="obj/"

gcc_asm() {
  echo "Generating assembly for $@"
  out_path="$OBJDIR$@"
  out_path="${out_path%.*}.s"
  mkdir -p $(dirname "$out_path")
  gcc -S -O3 -fno-exceptions -fno-asynchronous-unwind-tables -fno-dwarf2-cfi-asm "$@" -o "$out_path"
  bat "$out_path"
}

compile_asm() {
  asm_path="$@"
  if [[ $asm_path == $OBJDIR* ]]
  then
    # We don't need to append objdir path if it's already there.
    obj_path="$@"
  else
    obj_path="$OBJDIR$@"
  fi

  obj_path="${obj_path%.*}.o"
  out_path="${obj_path%.*}.out"
  mkdir -p $(dirname "$out_path")
  echo "Compiling $asm_path to $out_path"
  as -o "$obj_path" "$@"
  ld -macosx_version_min 13.0.0 -o "$out_path" "$obj_path" -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _main -arch arm64
}

clean() {
  # `brew install trash`.
  trash "$OBJDIR"
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
  obj_path="$OBJDIR$@"
  out_path="${obj_path%.*}.out"
  echo "Running $out_path"
  $out_path
elif [ $command == "clean" ]
then
  clean
else
    echo "Invalid command"
fi
