use crate::ast::node::Node;
use crate::token::token::Token;

// structurs
use crate::structures::let_s::build_let;
use crate::structures::section::build_section;
use crate::structures::root::build_root;

pub fn build(mut input: Vec<Token>) -> Node {
    let mut pointer: usize = 0;
    let mut _top = Box::new(build_root());
    let mut _current = &mut *_top as *mut Node;

    while pointer < input.len() {
        if input[pointer].get_type() == Token::SEC_DECL {
            println!("Section declartion: {}", input[pointer + 1].clone().show());
            let mut _section = build_section(
                &mut Node {
                    _token: input[pointer + 1].clone(),
                    _children: vec![],
                },
                &mut Node::empty(),
            );
            unsafe {
                println!("Section value: {{");
                (*_section).output("".to_string());
                println!("}}");
                (*_current).add_child(_section);
            }
            pointer += 2;
        } else {
            pointer += 1;
        }
    }
    println!("Child count _top: {}", (*_top)._children.len());
    *_top
}

