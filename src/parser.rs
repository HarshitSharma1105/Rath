
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



pub fn parse_line(line: &str,instructions: &mut Vec<Instruction>)
{
    let parts = line.split_whitespace();
    for word in parts{
        if word=="+" 
        {
            instructions.push(Instruction::Add);
        }
        else if word=="-"
        {
            instructions.push(Instruction::Sub);
        }
        else if word == "*"
        {
            instructions.push(Instruction::Mult);
        }
        else if word == "/"
        {
            instructions.push(Instruction::Div);
        }
        else if word == "."
        {
            instructions.push(Instruction::Dump);
        }
        else 
        {
            let val: i64 = str::from_utf8(word.as_bytes()).expect("").parse().expect("");
            instructions.push(Instruction::Push(val));
        }
    }
}

pub fn parse(contents:String) -> Vec<Instruction>
{
    let mut instructions: Vec<Instruction> = vec![];
    let lines = contents.lines();
    for line in lines{
        parse_line(line,&mut instructions);
    }
    instructions
}