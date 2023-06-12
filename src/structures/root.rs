use crate::ast::node::Node;
use crate::token::token::Token;

pub fn build_root() -> Node {
    Node {
        _token: Token::new(Token::ROOT.to_string(), "ROOT".to_string()),
        _children: vec![
        ],
    }
}

