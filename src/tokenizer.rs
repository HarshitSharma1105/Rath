
#[derive(Debug)]
pub enum Instruction
{
    Push(i64),
    Dump,
    Add,
    Sub,
    Mult,
    Div
}


pub fn parse(contents:String) -> Vec<Instruction>
{
    let mut instructions: Vec<Instruction> = vec![];
    let mut idx = 0;
    let mut buff : String = String::default();
    let src : Vec<char> = contents.chars().collect();
    let siz = src.len();
    while idx < siz
    {
        if src[idx].is_digit(10)
        {
            while src[idx].is_digit(10)
            {
                buff.push(src[idx]);
                idx += 1;
            }
            let val: i64 = buff.parse().unwrap();
            instructions.push(Instruction::Push(val));
            buff.clear();
            continue;
        }
        if idx < siz && (src[idx] == '\n' || src[idx] == ' ')
        {
            while idx < siz && (src[idx] == '\n' || src[idx] == ' ')
            {
                idx += 1;
            }
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
        buff.clear();
    }
    instructions
}