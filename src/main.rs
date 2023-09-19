mod codegen;
mod lexer;
mod parser;

fn main() {
    let tokenizer = lexer::Tokenizer::new("int main() { return 123; }");
    let token_stream = tokenizer.tokenize().expect("Tokenizing phase has failed.");

    let parser = parser::Parser::new(token_stream);
    let program_ast = parser.parse().expect("Parsing phase has failed.");

    let codegen = codegen::ARMCodegen::new();

    let asm = codegen
        .generate(program_ast)
        .expect("Codegen phase has failed.");

    println!("{}", asm);
}
