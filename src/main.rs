mod interpreter;
mod parser;


use interpreter::*;
use parser::*;


use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = fs::read_to_string(file_name).expect("Unable to open file");
    let prog=parse(contents);
    interpret(prog);
}
