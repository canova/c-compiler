pub mod lexer;

fn main() {
    println!("Hello, world!");

    let tokenizer = lexer::Tokenizer::new("int main() { return 2; }");
    let token_stream = tokenizer.tokenize();
    println!("{:#?}", token_stream);
}
