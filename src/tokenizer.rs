
#[macro_export]
macro_rules! write_to_file {
    ($file:expr, $contents:expr) => {{
        use std::io::Write;
        let _ = $file.write_all($contents.as_bytes());
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
        let _ = Command::new("sh").arg("-c").arg($command).status().unwrap();
    }};
}

macro_rules! errorf{
    ($error:expr,$file_name:expr,$row:expr,$col:expr) =>{{
        println!("ERROR: {} at {}:{}:{}",$error,$file_name,$row,$col);
        std::process::exit(1);
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
            instructions.push(Instruction::Dump);
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
        else if buff == "if"
        {
            stk.push(instructions.len());
            instructions.push(Instruction::If(0));
        }
        else if buff == "else"
        {
            if stk.len() == 0
            {
                errorf!("else without a if block",file_name,line_num,idx-line_start-buff.len()+1)
            } 
            let idx = stk.pop().unwrap();
            let len = instructions.len();
            match &mut instructions[idx]
            {
                Instruction::If(val) => *val = len + 1,
                _ =>   errorf!("Compiler Bug in else",file!(),line!(),column!())
            }
            stk.push(instructions.len());
            instructions.push(Instruction::Else(0));
        }
        else if buff == "dup"
        {
            instructions.push(Instruction::Dup);
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
                errorf!("do without a while block",file_name,line_num,idx-line_start-buff.len()+1)
            } 
            let idx = stk.pop().unwrap();
            if let Instruction::While = instructions[idx]
            {
                stk.push(instructions.len());
                instructions.push(Instruction::Do(idx));
            }
            else 
            {
                errorf!("Compiler error in loops",file!(),line!(),column!())
            }
        }
        else if buff == "end"
        {
            if stk.len() == 0
            {
                errorf!("end without a if/else block",file_name,line_num,idx-line_start-buff.len()+1)
            } 
            let idx = stk.pop().unwrap();
            let len = instructions.len();
            match &mut instructions[idx]
            {
                Instruction::If(val) | Instruction::Else(val) => *val = len,
                Instruction::Do(_) => {}
                _ =>   errorf!("Compiler error in if/else while",file!(),line!(),column!())
            }
            if let Instruction::Do(val) = instructions[idx]
            {
                instructions[idx] = Instruction::Do(len + 1);
                instructions.push(Instruction::End(val));
            }
        }
        else 
        {
            errorf!(format!("Unknown token {}",buff),file_name,line_num,idx-line_start-buff.len()+1);
        }
        buff.clear();
    }
    instructions
}