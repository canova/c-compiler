mod codegen;
mod lexer;
mod parser;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Skip the first argument, which is the name of the program.
    // TODO: Use a proper argument parser.
    let args = env::args().skip(1).collect::<Vec<_>>();
    let is_dry_run = args.iter().any(|arg| arg.contains("--dry-run"));
    let file_path = args
        .iter()
        .find(|arg| !arg.starts_with("--"))
        .map(PathBuf::from);

    let file_content = if let Some(ref file_path) = file_path {
        fs::read_to_string(file_path).unwrap()
    } else {
        // FIXME: Just a dummy program for now.
        println!("No input file provided. Using a dummy program.\n");
        "int main() { return !123; }".into()
    };

    let tokenizer = lexer::Tokenizer::new(&file_content);
    let token_stream = tokenizer.tokenize().expect("Tokenizing phase has failed.");

    let parser = parser::Parser::new(token_stream);
    let program_ast = parser.parse().expect("Parsing phase has failed.");

    let codegen = codegen::ARMCodegen::new();

    let asm = codegen
        .generate(program_ast)
        .expect("Codegen phase has failed.");

    println!("Assembly output:");
    println!("{}\n", asm);

    if is_dry_run {
        // No need to generate the assembly if we're just doing a dry run.
        return;
    }

    if let Some(ref path) = file_path {
        // Write the assembly to a file if it was provided.
        let mut asm_file = path.clone();
        asm_file.set_extension("s");
        println!("Writing assembly to file: {:?}", asm_file);

        fs::write(&asm_file, asm).expect("Couldn't write to file.");
        compile_asm(&asm_file);
    }
}

fn compile_asm(asm_file: &Path) {
    let obj_file = asm_file.with_extension("o");
    println!("Writing object file to: {:?}", obj_file);
    // as -o output.o output.s
    let output = Command::new("as")
        .args(["-o", obj_file.to_str().unwrap(), asm_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute process");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stdout().write_all(&output.stderr).unwrap();

    let executable_file = obj_file.with_extension("");
    println!("Writing executable file to: {:?}", executable_file);
    // ld -macosx_version_min 13.0.0 -o output output.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _main -arch arm64
    let sdk_path = Command::new("xcrun")
        .args(["-sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("Failed to execute process");

    let output = Command::new("ld")
        .args([
            "-macosx_version_min",
            "13.0.0",
            "-o",
            executable_file.to_str().unwrap(),
            obj_file.to_str().unwrap(),
            "-lSystem",
            "-syslibroot",
            String::from_utf8_lossy(&sdk_path.stdout).trim(),
            "-e",
            "_main",
            "-arch",
            "arm64",
        ])
        .output()
        .expect("Failed to execute process");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stdout().write_all(&output.stderr).unwrap();
}
