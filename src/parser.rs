
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
    let parts = contents.split_whitespace();
    for part in parts{
        if part=="+" {
            instructions.push(Instruction::Add);
        }
        else if part=="-"{
            instructions.push(Instruction::Sub);
        }
        else if part == "*"{
            instructions.push(Instruction::Mult);
        }
        else if part == "/"{
            instructions.push(Instruction::Div);
        }
        else if part == "."{
            instructions.push(Instruction::Dump);
        }
        else {
            let val: i64 = str::from_utf8(part.as_bytes()).expect("").parse().expect("");
            instructions.push(Instruction::Push(val));
        }
    }
    instructions
}