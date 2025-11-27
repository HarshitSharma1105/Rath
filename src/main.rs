mod interpreter;
mod tokenizer;
mod compiler;

use interpreter::*;
use tokenizer::*;
use compiler::*;

fn print_usage()
{
    use std::process::exit;
    print!("Expected arguments <compile/interpret> <file name>\n");
    exit(1);
}



fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 3
    {
        print_usage();
    }
    let mode = &args[1];
    let file_name = &args[2];
    let prog = parse(file_name);
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
    }
}
