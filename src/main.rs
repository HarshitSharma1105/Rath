mod interpreter;
mod instructions;
use instructions::Instruction as Instruction;

fn main() {
    let prog:Vec<Instruction>=vec![
        Instruction::Push(34),
        Instruction::Push(35),
        Instruction::Add,
        Instruction::Dump];
    interpreter::interpret(prog);
}
