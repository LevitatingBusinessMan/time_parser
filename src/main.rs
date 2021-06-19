mod lexer;
use lexer::lex;
use std::io;

fn main() {
    let mut buf = String::new();
    let stdin = io::stdin();
    loop {
        stdin.read_line(&mut buf).unwrap();

        let tokens = lex(&buf);

        buf = String::new();
    }
}
