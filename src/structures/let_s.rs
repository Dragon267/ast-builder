use crate::ast::node::Node;
use crate::token::token::Token;

pub fn build_let(name: *mut Node, value: *mut Node) -> *mut Node {
    &mut Node {
        _token: Token::new(Token::LET.to_string(), "let".to_string()),
        _children: vec![
            name,
            &mut Node {
                _token: Token::new(Token::ASSIGN.to_string(), "=".to_string()),
                _children: vec![],
            },
            value,
        ],
    }
}

