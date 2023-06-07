use crate::token::token::Token;
use std::rc::Rc;

pub struct Node {
    pub _token: Token,
    pub _children: Vec<*mut Node>,
}

impl Node {
    pub fn empty() -> Node {
        Node {
            _token: Token::new(Token::COMPILER.to_string(), "COMPILER".to_string()),
            _children: vec![],
        }
    }

    pub fn new(t: Token, childs: Vec<*mut Node>) -> Node {
        Node {
            _token: t,
            _children: childs,
        }
    }

    pub fn add_child(&mut self, child: *mut Node) {
        self._children.push(child);
    }
}
