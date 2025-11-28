use crate::tokenizer::*;
pub fn interpret(program: Vec<Instruction>)
{
    let mut stack : Vec<i64> = Vec::new();
    let mut i = 0;
    while i < program.len()
    {
        let instruction = &program[i];
        match *instruction
        {
            Instruction::Push(val) => 
            {
                stack.push(val);
                i += 1;
            }
            Instruction::Add => 
            {
                assert!(stack.len() > 1);
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a+b);
                i += 1;
            }
            Instruction::Sub => 
            {
                assert!(stack.len() > 1);
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a-b);
                i += 1;
            }
            Instruction::Mult => 
            {
                assert!(stack.len() > 1);
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a*b);
                i += 1;
            }
            Instruction::Div => 
            {
                assert!(stack.len() > 1);
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a/b);
                i += 1;
            }
            Instruction::Equals =>
            {
                assert!(stack.len() > 1);
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a==b) as i64);
                i += 1;
            }
            Instruction::Greater =>
            {
                assert!(stack.len() > 1);
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a>b) as i64);
                i += 1;
            }
            Instruction::Less =>
            {
                assert!(stack.len() > 1);
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a<b) as i64);
                i += 1;
            }
            Instruction::If(val) =>
            {
                let top = stack.pop().unwrap();
                if top == 0 
                {
                    i = val;
                    continue;
                }
                i += 1;
            }
            Instruction::Else(val) => 
            {
                i = val;
            }
            Instruction::Do(val) => 
            {
                let top = stack.pop().unwrap();
                if top == 0 
                {
                    i = val;
                    continue;
                }
                i += 1;
            }
            Instruction::While(_) => i += 1,
            Instruction::End(val) => 
            {
                i = val;
            }
            Instruction::Dup => 
            {
                let last = stack.pop().unwrap();
                stack.push(last);
                stack.push(last);
                i += 1;
            }
            Instruction::Dump => 
            {
                println!("{}",stack.pop().unwrap());
                i += 1;
            }
        }
    }
}