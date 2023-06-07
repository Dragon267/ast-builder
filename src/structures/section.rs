use crate::ast::node::Node;
use crate::token::token::Token;

pub fn build_section(name: *mut Node, value: *mut Node) -> *mut Node {
    &mut Node {
        _token: Token::new(Token::SEC_DECL.to_string(), "--".to_string()),
        _children: vec![
            name,
            value,
        ],
    }
}

