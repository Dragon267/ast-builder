use crate::ast::node::Node;
use crate::token::token::Token;

pub fn build_section(name: *mut Node, value: *mut Node) -> *mut Node {
    let node = Box::new(Node {
        _token: Token::new(Token::SEC_DECL.to_string(), "--".to_string()),
        _children: vec![
            name,
            value,
        ],
    });

    Box::into_raw(node)
}

