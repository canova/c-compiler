mod lexer;
mod parser;

fn main() {
    let tokenizer = lexer::Tokenizer::new("int main() { return 123; }");
    let token_stream = tokenizer.tokenize().expect("Tokenizer has failed.");

    let parser = parser::Parser::new(token_stream);
    let ast = parser.parse();

    println!("{:#?}", ast);
}
