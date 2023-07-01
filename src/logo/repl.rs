use std::env;
use std::fs;

pub fn init_repl() {
    let contents = fs::read_to_string("src/logo/image.txt")
        .expect("Error while reading logo");
    println!("{}", contents);
}
