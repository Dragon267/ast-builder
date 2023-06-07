mod token;
mod lexer;
mod repl;
//mod ast;
//mod structures;

use repl::repl::init_repl;

use crate::token::token::Token;
use crate::lexer::lexer::Lexer;

fn main() {
    init_repl();
    let s = "-- main { print(\"Hello world\"); }";
    let mut _lexer = Lexer::new(s.to_string());
    let mut _tokens = _lexer.get_tokens();
    for i in 0.._tokens.len() {
        println!("{}", _tokens[i].show());
    }
}
