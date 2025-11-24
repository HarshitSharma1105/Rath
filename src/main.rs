mod interpreter;
mod parser;
mod compiler;

use interpreter::*;
use parser::*;
use compiler::*;

use std::env;
use std::process;

fn print_usage()
{
    print!("Expected arguments <compile/interpret> <file name>\n");
}



fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3
    {
        print_usage();
        process::exit(1);
    }
    let mode = &args[1];
    let file_name = &args[2];
    let contents = read_file!(file_name);
    let prog = parse(contents);
    if mode =="compile"
    {
        compile(prog);
    }
    else if mode == "interpret"
    {
        interpret(prog);
    }
    else 
    {
        print_usage();
        process::exit(1);
    }
}
