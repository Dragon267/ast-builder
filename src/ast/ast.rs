use crate::ast::node::Node;
use crate::token::token::Token;

// structurs
use crate::structures::let_s::build_let;
use crate::structures::section::build_section;
use crate::structures::root::build_root;

pub fn build(input: Vec<Token>) {
    let mut pointer: usize = 0;
    let mut _top = Node::empty();
    while pointer < input.len() {
        if input[pointer].get_type() == Token::SEC_DECL {
            _top.add_child(
                build_section(
                    &mut Node {
                        _token: input[pointer + 1].clone(),
                        _children: vec![],
                    },
                    &mut Node::empty(),
                )
            );
        }
        pointer += 1;
    }
}

