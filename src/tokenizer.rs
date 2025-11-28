
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
    Greater
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
        else 
        {
            println!("Unknown token {} at {}:{}:{}",buff,file_name,line_num,idx-line_start-buff.len()+1);
            std::process::exit(1);
        }
        buff.clear();
    }
    instructions
}