use crate::tokenizer::*;
use crate::{write_to_file,create_file,run_command};





pub fn compile(program: Vec<Instruction>,run: bool)
{
    run_command!("mkdir -p builds");
    let mut file = create_file!("builds/output.asm");
    write_to_file!(file,"format ELF64");
    write_to_file!(file,"section \".text\" executable");
    write_to_file!(file,"print:");
    write_to_file!(file,"    mov     r9, -3689348814741910323");
    write_to_file!(file,"    sub     rsp, 40");
    write_to_file!(file,"    mov     BYTE [rsp+31], 10");
    write_to_file!(file,"    lea     rcx, [rsp+30]");
    write_to_file!(file,".L2:");
    write_to_file!(file,"    mov     rax, rdi");
    write_to_file!(file,"    lea     r8, [rsp+32]");
    write_to_file!(file,"    mul     r9");
    write_to_file!(file,"    mov     rax, rdi");
    write_to_file!(file,"    sub     r8, rcx");
    write_to_file!(file,"    shr     rdx, 3");
    write_to_file!(file,"    lea     rsi, [rdx+rdx*4]");
    write_to_file!(file,"    add     rsi, rsi");
    write_to_file!(file,"    sub     rax, rsi");
    write_to_file!(file,"    add     eax, 48");
    write_to_file!(file,"    mov     BYTE [rcx], al");
    write_to_file!(file,"    mov     rax, rdi");
    write_to_file!(file,"    mov     rdi, rdx");
    write_to_file!(file,"    mov     rdx, rcx");
    write_to_file!(file,"    sub     rcx, 1");
    write_to_file!(file,"    cmp     rax, 9");
    write_to_file!(file,"    ja      .L2");
    write_to_file!(file,"    lea     rax, [rsp+32]");
    write_to_file!(file,"    mov     edi, 1");
    write_to_file!(file,"    sub     rdx, rax");
    write_to_file!(file,"    xor     eax, eax");
    write_to_file!(file,"    lea     rsi, [rsp+32+rdx]");
    write_to_file!(file,"    mov     rdx, r8");
    write_to_file!(file,"    mov     rax, 1");
    write_to_file!(file,"    syscall");
    write_to_file!(file,"    add     rsp, 40");
    write_to_file!(file,"    ret");
    write_to_file!(file,"public _start");
    write_to_file!(file,"_start:");
    write_to_file!(file,"    mov rbp,rsp");
    for (idx,instruction) in program.iter().enumerate()
    {
        write_to_file!(file,format!("instr_{}:",idx));
        match instruction
        {
            Instruction::Push(val) => {write_to_file!(file,format!("    push {}",val));}
            Instruction::Add =>
            {
                write_to_file!(file,"    pop rbx");
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    add rax,rbx");
                write_to_file!(file,"    push rax");
            }
            Instruction::Sub =>
            {
                write_to_file!(file,"    pop rbx");
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    sub rax,rbx");
                write_to_file!(file,"    push rax");
            }
            Instruction::Mult =>
            {
                write_to_file!(file,"    pop rbx");
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    imul rax,rbx");
                write_to_file!(file,"    push rax");
            }
            Instruction::Div =>
            {
                write_to_file!(file,"    pop rbx");
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    xor rdx,rdx");
                write_to_file!(file,"    div rbx");
                write_to_file!(file,"    push rax");
            }
            Instruction::Equals => 
            {
                write_to_file!(file,"    pop rbx");
                write_to_file!(file,"    pop rax");              
                write_to_file!(file,"    cmp rax,rbx");
                write_to_file!(file,"    sete al");
                write_to_file!(file,"    movzx rax,al");
                write_to_file!(file,"    push rax");  
            }
            Instruction::Greater => 
            {
                write_to_file!(file,"    pop rbx");
                write_to_file!(file,"    pop rax");              
                write_to_file!(file,"    cmp rax,rbx");
                write_to_file!(file,"    setg al");
                write_to_file!(file,"    movzx rax,al");
                write_to_file!(file,"    push rax");  
            }
            Instruction::Less => 
            {
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    pop rbx");              
                write_to_file!(file,"    cmp rax,rbx");
                write_to_file!(file,"    setl al");
                write_to_file!(file,"    movzx rax,al");
                write_to_file!(file,"    push rax");  
            }
            Instruction::Mem =>
            {
                write_to_file!(file,"    push mem");
            }
            Instruction::ShiftLeft => 
            {
                write_to_file!(file,"    pop rcx");
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    shl rax,cl");
                write_to_file!(file,"    push rax");
            }
            Instruction::ShiftRight => 
            {
                write_to_file!(file,"    pop rcx");
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    shr rax,cl");
                write_to_file!(file,"    push rax");
            }
            Instruction::BitAnd => 
            {
                write_to_file!(file,"    pop rbx");
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    and rax,rbx");
                write_to_file!(file,"    push rax");
            }
            Instruction::BitOr => 
            {
                write_to_file!(file,"    pop rbx");
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    or  rax,rbx");
                write_to_file!(file,"    push rax");
            }
            Instruction::Store => 
            {
                write_to_file!(file,"    pop rbx");
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    mov [rax],bl");
            }
            Instruction::Load => 
            {
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    xor rbx,rbx");
                write_to_file!(file,"    mov bl,[rax]");
                write_to_file!(file,"    push rbx");
            }
            Instruction::If(val) | Instruction::Do(val) => 
            {
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    test rax,rax");
                write_to_file!(file,format!("    jz instr_{}",val));
            }
            Instruction::Else(val) | Instruction::End(val) => 
            {
                write_to_file!(file,format!("    jmp instr_{}",val));
            }
            Instruction::While => {}
            Instruction::Dup => 
            {
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    push rax");
                write_to_file!(file,"    push rax");
            }
            Instruction::Dup2 => 
            {
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    pop rbx");
                write_to_file!(file,"    push rbx");
                write_to_file!(file,"    push rax");
            }
            Instruction::Swap => 
            {
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    pop rbx");
                write_to_file!(file,"    push rax");
                write_to_file!(file,"    push rbx");
            }
            Instruction::Dump => 
            {
                write_to_file!(file,"    pop rdi");
                write_to_file!(file,"    call print");
            }
            Instruction::Drop =>
            {
                write_to_file!(file,"    pop rax");
            }
            Instruction::Syscall1 =>
            {
                write_to_file!(file,"    pop rdi");
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    syscall");
            }
            Instruction::Syscall3 => 
            {
                write_to_file!(file,"    pop rdx");
                write_to_file!(file,"    pop rsi");
                write_to_file!(file,"    pop rdi");
                write_to_file!(file,"    pop rax");
                write_to_file!(file,"    syscall");
            }
        }
    }
    write_to_file!(file,format!("instr_{}:",program.len()));
    write_to_file!(file,"    mov rsp,rbp");
    write_to_file!(file,"    xor rax,rax");
    write_to_file!(file,"    mov rax,60");
    write_to_file!(file,"    mov rdi,0");
    write_to_file!(file,"    syscall");
    write_to_file!(file,"section \".bss\"");
    write_to_file!(file,"    mem: rb 64000");
    run_command!("fasm builds/output.asm");
    run_command!("ld builds/output.o -o builds/output");
    if run 
    {
        run_command!("builds/output");
    }
}