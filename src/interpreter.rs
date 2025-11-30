use crate::tokenizer::*;
pub fn interpret(program: Vec<Instruction>)
{
    let mut stack : Vec<i64> = Vec::new();
    let mut i = 0;
    let mut memory : [u8;200] = [0;200];
    while i < program.len()
    {
        match program[i]
        {
            Instruction::Push(val) => 
            {
                stack.push(val);
                i += 1;
            }
            Instruction::Add => 
            {
                // TODO : Make this an error not an assert using errorf
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
            Instruction::While => i += 1,
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
            Instruction::Mem => 
            {
                stack.push(0);
                i += 1;
            }
            Instruction::Store => 
            {
                assert!(stack.len() > 1);
                let val = stack.pop().unwrap();
                let index = stack.pop().unwrap();
                memory[index as usize] = (val & 0xFF) as u8;
                i += 1;
            }
            Instruction::Load =>
            {
                assert!(stack.len() > 0);
                let index = stack.pop().unwrap() as usize;
                stack.push(memory[index] as i64);
                i += 1;
            }
            Instruction::Syscall1 => 
            {
                assert!(stack.len() > 1);
                let syscall_arg1 = stack.pop().unwrap();
                let syscall_num  = stack.pop().unwrap();
                if syscall_num == 60
                {
                    std::process::exit(syscall_arg1 as i32);
                }
                else 
                {
                    assert!(false, "Implement more syscalls\n");
                }
                i += 1;
            }
            Instruction::Syscall3 => 
            {
                assert!(stack.len() > 3);
                let syscall_arg3 = stack.pop().unwrap();
                let syscall_arg2 = stack.pop().unwrap();
                let syscall_arg1 = stack.pop().unwrap();
                let syscall_num  = stack.pop().unwrap();
                if syscall_num == 1
                {
                    for i in syscall_arg2..syscall_arg2+syscall_arg3
                    {
                        if syscall_arg1 == 1
                        {
                            print!("{}",memory[i as usize] as char);
                        }
                        else if syscall_arg1 == 2
                        {
                            eprint!("{}",memory[i as usize] as char);
                        }
                        else
                        {
                            assert!(false, "invalid fd for simulation\n");
                        }
                    }
                }
                else 
                {
                    assert!(false, "Implement more syscalls\n");
                }
                i += 1;
            }
        }
    }
}