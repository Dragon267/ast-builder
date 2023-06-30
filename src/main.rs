mod token;
mod lexer;
mod test;
mod logger;
mod logo;

use logo::repl::init_repl;

// testing
use test::test::status;
use test::test::test;

// main

use crate::lexer::lexer::Lexer;

fn main() {
    if status() == true { test(); }
    else {
        init_repl();
        let s = "-- main { print(\"Hello world\"); }";
        let mut _lexer = Lexer::new(s.to_string());
        let mut _tokens = _lexer.get_tokens();
        for i in 0.._tokens.len() {
            println!("{}", _tokens[i].show());
        }
    }
}

