mod codegen;
mod parser;
mod tokenizer;

use clap::Parser;
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

/// A toy C compiler that outputs ARM64 assembly.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// C source file path to compile.
    file: Option<PathBuf>,

    /// Whether to just do a dry run. Will only print the assembly to stdout.
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,

    /// Whether to print the AST to stdout.
    #[arg(short, long, default_value_t = false)]
    ast: bool,

    /// Whether to not print the assembly to stdout.
    #[arg(short, long, default_value_t = false)]
    no_asm: bool,
}

fn main() {
    let args = Args::parse();
    let file_content = if let Some(ref file_path) = args.file {
        fs::read_to_string(file_path).unwrap()
    } else {
        println!("No input file provided. Using a dummy program.\n");
        "int main() { return 1 || 2; }".into()
    };

    let tokenizer = tokenizer::Tokenizer::new(&file_content);
    let token_stream = match tokenizer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Tokenizing phase has failed: {}", err);
            std::process::exit(1);
        }
    };

    let parser = parser::Parser::new(token_stream);
    let program_ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("Parsing phase has failed: {}", err);
            std::process::exit(1);
        }
    };

    if args.ast {
        println!("AST output:\n{:#?}\n", program_ast);
    }

    let codegen = codegen::ARMCodegen::new();
    let asm = match codegen.generate(program_ast) {
        Ok(asm) => asm,
        Err(err) => {
            eprintln!("Codegen phase has failed: {}", err);
            std::process::exit(1);
        }
    };

    if !args.no_asm {
        println!("Assembly output:\n{}", asm);
    }

    if args.dry_run {
        // No need to generate the assembly if we're just doing a dry run.
        return;
    }

    if let Some(ref path) = args.file {
        // Write the assembly to a file if it was provided.
        let mut asm_file = Path::new("obj").join(path);
        asm_file.set_extension("s");
        println!("Writing assembly to file: {:?}", asm_file);

        let _ = fs::create_dir_all(asm_file.parent().unwrap());
        fs::write(&asm_file, asm).expect("Couldn't write to file");
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
