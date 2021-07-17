mod lexer;
mod parser;

use lexer::lex;
use std::io;
use parser::parse;

fn main() {
    let mut buf = String::new();
    let stdin = io::stdin();
    loop {
        stdin.read_line(&mut buf).unwrap();

        let tokens = lex(&buf);

        parse(tokens);

        buf = String::new();
    }
}
