mod interpreter;
mod tokenizer;
mod compiler;

use interpreter::*;
use tokenizer::*;
use compiler::*;

fn print_usage()
{
    use std::process::exit;
    print!("Expected arguments <file name> <compile/interpret>  run\n");
    exit(1);
}



fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 3
    {
        print_usage();
    }
    let mode = &args[2];
    let file_name = &args[1];
    let prog = parse(file_name);
    if mode == "compile"
    {
        let mut run : bool = false;
        if args.len() > 3
        {
            if args[3] == "run"
            {
                run = true;
            }
            else 
            {
                print_usage();
            }
        }
        compile(prog,run);
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
