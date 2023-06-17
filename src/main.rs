mod lexer;
mod opcode;
mod parser;
mod value;
mod vm;

fn main() {
    let tokens = lexer::tokenize("(- (* 10 3) 10)");

    println!("{:?}", tokens);
    println!("All done!");
}
