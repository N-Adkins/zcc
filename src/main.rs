#![allow(dead_code)]

use lexer::Lexer;

mod comp_error;
mod lang;
mod lexer;

fn main() {
    let test_src = include_str!("../tests/big_test.c");
    let mut lexer = Lexer::new(test_src);
    match lexer.tokenize() {
        Ok(_) => println!("Success"),
        Err(err) => print!("{}", err),
    }

    println!("{:#?}", lexer);
}
