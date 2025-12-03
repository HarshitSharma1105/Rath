
#[macro_export]
macro_rules! write_to_file {
    ($file:expr, $contents:expr) => {{
        use std::io::Write;
        let _ = $file.write_all(format!("{}\n",$contents).as_bytes());
    }};
}


#[macro_export]
macro_rules! create_file{
    ($name:expr) => {{
        use std::fs::File;
        File::create($name).expect("Failed to create file")
    }};
}


#[macro_export]
macro_rules! read_file {
    ($path:expr) => {{
        use std::fs;
        fs::read_to_string($path).expect("Unable to open file")
    }};
}

#[macro_export]
macro_rules! run_command {
    ($command:expr) => {{
        use std::process::Command;
        Command::new("sh").arg("-c").arg($command).status().unwrap()
    }};
}
#[macro_export]
macro_rules! errorf{
    ($error:expr,$file_name:expr,$row:expr,$col:expr) =>{{
        println!("ERROR: {} at {}:{}:{}",$error,$file_name,$row,$col);
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! pop{
    ($vec:expr) => {{
        $vec.pop().unwrap()
    }};
}

#[derive(Debug)]
pub enum Instruction
{
    Push(i64),
    Dump,
    Add,
    Sub,
    Mult,
    Div,
    Equals,
    Less,
    Greater,
    If(usize),
    Else(usize),
    While,
    Do(usize),
    End(usize),
    Dup,
    Dup2,
    Swap,
    Drop,
    Mem,
    Store,
    Load,
    Syscall1,
    Syscall3,
    BitOr,
    BitAnd,
    ShiftLeft,
    ShiftRight
}


pub fn parse(file_name: &String) -> Vec<Instruction>
{
    let mut instructions: Vec<Instruction> = Vec::default();
    let mut idx = 0;
    let mut line_num = 1;
    let mut line_start = 0;
    let mut buff : String = String::default();
    let src : Vec<char> = read_file!(file_name).chars().collect();
    let siz = src.len();
    let mut stk : Vec<usize> = Vec::default();
    while idx < siz
    {
        if src[idx].is_digit(10)
        {
            while idx < siz && src[idx].is_digit(10)
            {
                buff.push(src[idx]);
                idx += 1;
            }
            let val: i64 = buff.parse().unwrap();
            instructions.push(Instruction::Push(val));
            buff.clear();
            continue;
        }
        if idx < siz && src[idx] == ' '
        {
            while idx < siz && src[idx] == ' '
            {
                idx += 1;
            }
            continue;
        }
        if src[idx] == '\n'
        {
            line_num += 1;
            idx += 1;
            line_start = idx; 
            continue;
        }
        while idx < siz && src[idx] != ' ' && src[idx] != '\n'
        {
            buff.push(src[idx]);
            idx += 1;
        }
        if buff=="+" 
        {
            instructions.push(Instruction::Add);
        }
        else if buff=="-"
        {
            instructions.push(Instruction::Sub);
        }
        else if buff == "*"
        {
            instructions.push(Instruction::Mult);
        }
        else if buff == "/"
        {
            instructions.push(Instruction::Div);
        }
        else if buff == "."
        {
            instructions.push(Instruction::Store);
        }
        else if buff == ","
        {
            instructions.push(Instruction::Load);
        }
        else if buff == "="
        {
            instructions.push(Instruction::Equals);
        }
        else if buff == ">"
        {
            instructions.push(Instruction::Greater);
        }
        else if buff == "<"
        {
            instructions.push(Instruction::Less);
        }
        else if buff == "//"
        {
            while idx < siz && src[idx] != '\n'
            {
                idx += 1;
            }   
        }
        else if buff == "dump"
        {
            instructions.push(Instruction::Dump);
        }
        else if buff == "drop"
        {
            instructions.push(Instruction::Drop);
        }
        else if buff == "mem"
        {
            instructions.push(Instruction::Mem);
        }
        else if buff == ">>"
        {
            instructions.push(Instruction::ShiftRight);
        }
        else if buff == "<<"
        {
            instructions.push(Instruction::ShiftLeft);
        }
        else if buff == "&" 
        {
            instructions.push(Instruction::BitAnd);
        }
        else if buff == "|" 
        {
            instructions.push(Instruction::BitOr);
        }
        else if buff == "if"
        {
            stk.push(instructions.len());
            instructions.push(Instruction::If(0));
        }
        else if buff == "else"
        {
            if stk.len() == 0
            {
                errorf!("else without a if block",file_name,line_num,idx-line_start-buff.len()+1);
            } 
            let index = pop!(stk);
            let len = instructions.len();
            match &mut instructions[index]
            {
                Instruction::If(val) => *val = len + 1,
                Instruction::Else(_) | Instruction::Do(_) | Instruction::While => 
                errorf!("missing if block for else",file_name,line_num,idx-line_start-buff.len()+1),
                _ =>   errorf!("Compiler Bug in else",file!(),line!(),column!())
            }
            stk.push(instructions.len());
            instructions.push(Instruction::Else(0));
        }
        else if buff == "dup"
        {
            instructions.push(Instruction::Dup);
        }
        else if buff == "dup2"
        {
            instructions.push(Instruction::Dup2);
        }
        else if buff == "swap"
        {
            instructions.push(Instruction::Swap);
        }
        else if buff == "while"
        {
            stk.push(instructions.len());
            instructions.push(Instruction::While);
        }
        else if buff == "do"
        {
            if stk.len() == 0 
            {
                errorf!("do without a while block",file_name,line_num,idx-line_start-buff.len()+1);
            } 
            let index = pop!(stk);
            if let Instruction::While = instructions[index]
            {
                stk.push(instructions.len());
                instructions.push(Instruction::Do(index));
            }
            else if let Instruction:: If(_) | Instruction::Else(_) = instructions[index]
            {
                errorf!("do without a while block",file_name,line_num,idx-line_start-buff.len()+1);
            }
            else 
            {
                errorf!("Compiler error in loops",file!(),line!(),column!());
            }
        }
        else if buff == "end"
        {
            if stk.len() == 0
            {
                errorf!("end without a if/else block",file_name,line_num,idx-line_start-buff.len()+1);
            } 
            let index = pop!(stk);
            let len = instructions.len();
            match instructions[index]
            {
                Instruction::If(ref mut val) | Instruction::Else(ref mut val) => *val = len,
                Instruction::Do(val) => {
                    instructions.push(Instruction::End(val));
                    instructions[index] = Instruction::Do(instructions.len());
                }
                Instruction::While => errorf!("While Block without Do keyword",file_name,line_num,idx-line_start-buff.len()+1),
                _ =>   errorf!("Compiler error in if/else while",file!(),line!(),column!())
            }
        }
        else if buff == "syscall1"
        {
            instructions.push(Instruction::Syscall1);
        }
        else if buff == "syscall3"
        {
            instructions.push(Instruction::Syscall3);
        }
        else 
        {
            errorf!(format!("Unknown token {}",buff),file_name,line_num,idx-line_start-buff.len()+1);
        }
        buff.clear();
    }
    if stk.len() != 0
    {
        errorf!("Expected end to close off if/else/while blocks",file_name,line_num,idx-line_start-buff.len()+1);
    }
    instructions
}