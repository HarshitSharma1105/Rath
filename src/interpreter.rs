use crate::instructions::*;
pub fn interpret(program: Vec<Instruction>)
{
    let mut stack : Vec<i64> = Vec::new();
    for instruction in program
    {
        match instruction
        {
            Instruction::Push(val) => {stack.push(val);},
            Instruction::Pop => {stack.pop();},
            Instruction::Add => {
                assert!(stack.len() > 1);
                let a=stack.pop().unwrap();
                let b=stack.pop().unwrap();
                stack.push(a+b);
            }
            Instruction::Sub => {
                assert!(stack.len() > 1);
                let a=stack.pop().unwrap();
                let b=stack.pop().unwrap();
                stack.push(a-b);
            }
            Instruction::Mult => {
                assert!(stack.len() > 1);
                let a=stack.pop().unwrap();
                let b=stack.pop().unwrap();
                stack.push(a*b);
            }
            Instruction::Div => {
                assert!(stack.len() > 1);
                let a=stack.pop().unwrap();
                let b=stack.pop().unwrap();
                stack.push(a/b);
            }
            Instruction::Dump => println!("{}",stack.last().unwrap()),
        }
    }
}