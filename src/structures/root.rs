use crate::ast::node::Node;
use crate::token::token::Token;

pub fn build_root(_main: *mut Node, _title: *mut Node) -> *mut Node {
    &mut Node {
        _token: Token::new(Token::ROOT.to_string(), "root".to_string()),
        _children: vec![
            _main,
            _title,
        ],
    }
}

