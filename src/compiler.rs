use crate::tokenizer::*;
use crate::{write_to_file,create_file,run_command};





pub fn compile(program: Vec<Instruction>,run: bool)
{
    run_command!("mkdir -p builds");
    let mut file = create_file!("builds/output.asm");
    write_to_file!(file,"format ELF64\n");
    write_to_file!(file,"section \".text\" executable\n");
    write_to_file!(file,"print:\n");
    write_to_file!(file,"    mov     r9, -3689348814741910323\n");
    write_to_file!(file,"    sub     rsp, 40\n");
    write_to_file!(file,"    mov     BYTE [rsp+31], 10\n");
    write_to_file!(file,"    lea     rcx, [rsp+30]\n");
    write_to_file!(file,".L2:\n");
    write_to_file!(file,"    mov     rax, rdi\n");
    write_to_file!(file,"    lea     r8, [rsp+32]\n");
    write_to_file!(file,"    mul     r9\n");
    write_to_file!(file,"    mov     rax, rdi\n");
    write_to_file!(file,"    sub     r8, rcx\n");
    write_to_file!(file,"    shr     rdx, 3\n");
    write_to_file!(file,"    lea     rsi, [rdx+rdx*4]\n");
    write_to_file!(file,"    add     rsi, rsi\n");
    write_to_file!(file,"    sub     rax, rsi\n");
    write_to_file!(file,"    add     eax, 48\n");
    write_to_file!(file,"    mov     BYTE [rcx], al\n");
    write_to_file!(file,"    mov     rax, rdi\n");
    write_to_file!(file,"    mov     rdi, rdx\n");
    write_to_file!(file,"    mov     rdx, rcx\n");
    write_to_file!(file,"    sub     rcx, 1\n");
    write_to_file!(file,"    cmp     rax, 9\n");
    write_to_file!(file,"    ja      .L2\n");
    write_to_file!(file,"    lea     rax, [rsp+32]\n");
    write_to_file!(file,"    mov     edi, 1\n");
    write_to_file!(file,"    sub     rdx, rax\n");
    write_to_file!(file,"    xor     eax, eax\n");
    write_to_file!(file,"    lea     rsi, [rsp+32+rdx]\n");
    write_to_file!(file,"    mov     rdx, r8\n");
    write_to_file!(file,"    mov     rax, 1\n");
    write_to_file!(file,"    syscall\n");
    write_to_file!(file,"    add     rsp, 40\n");
    write_to_file!(file,"    ret\n");
    write_to_file!(file,"public _start\n");
    write_to_file!(file,"_start:\n");
    write_to_file!(file,"    mov rbp,rsp\n");
    for (idx,instruction) in program.iter().enumerate()
    {
        write_to_file!(file,format!("instr_{}:\n",idx));
        match instruction
        {
            Instruction::Push(val) => {write_to_file!(file,format!("    push {}\n",val));}
            Instruction::Add =>
            {
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    add rax,rbx\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::Sub =>
            {
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    sub rax,rbx\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::Mult =>
            {
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    imul rax,rbx\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::Div =>
            {
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    xor rdx,rdx\n");
                write_to_file!(file,"    div rbx\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::Equals => 
            {
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    pop rax\n");              
                write_to_file!(file,"    cmp rax,rbx\n");
                write_to_file!(file,"    sete al\n");
                write_to_file!(file,"    movzx rax,al\n");
                write_to_file!(file,"    push rax\n");  
            }
            Instruction::Greater => 
            {
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    pop rax\n");              
                write_to_file!(file,"    cmp rax,rbx\n");
                write_to_file!(file,"    setg al\n");
                write_to_file!(file,"    movzx rax,al\n");
                write_to_file!(file,"    push rax\n");  
            }
            Instruction::Less => 
            {
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    pop rbx\n");              
                write_to_file!(file,"    cmp rax,rbx\n");
                write_to_file!(file,"    setl al\n");
                write_to_file!(file,"    movzx rax,al\n");
                write_to_file!(file,"    push rax\n");  
            }
            Instruction::Mem =>
            {
                write_to_file!(file,"    push mem\n");
            }
            Instruction::ShiftLeft => 
            {
                write_to_file!(file,"    pop rcx\n");
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    shl rax,cl\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::ShiftRight => 
            {
                write_to_file!(file,"    pop rcx\n");
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    shr rax,cl\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::BitAnd => 
            {
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    and rax,rbx\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::BitOr => 
            {
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    or  rax,rbx\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::Store => 
            {
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    mov [rax],bl\n");
            }
            Instruction::Load => 
            {
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    xor rbx,rbx\n");
                write_to_file!(file,"    mov bl,[rax]\n");
                write_to_file!(file,"    push rbx\n");
            }
            Instruction::If(val) | Instruction::Do(val) => 
            {
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    test rax,rax\n");
                write_to_file!(file,format!("    jz instr_{}\n",val));
            }
            Instruction::Else(val) | Instruction::End(val) => 
            {
                write_to_file!(file,format!("    jmp instr_{}\n",val));
            }
            Instruction::While => {}
            Instruction::Dup => 
            {
                write_to_file!(file,"    push QWORD [rsp]\n");
            }
            Instruction::Dump => 
            {
                write_to_file!(file,"    pop rdi\n");
                write_to_file!(file,"    call print\n");
            }
            Instruction::Syscall1 =>
            {
                write_to_file!(file,"    pop rdi\n");
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    syscall\n");
            }
            Instruction::Syscall3 => 
            {
                write_to_file!(file,"    pop rdx\n");
                write_to_file!(file,"    pop rsi\n");
                write_to_file!(file,"    pop rdi\n");
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    syscall\n");
            }
        }
    }
    write_to_file!(file,format!("instr_{}:\n",program.len()));
    write_to_file!(file,"    mov rsp,rbp\n");
    write_to_file!(file,"    xor rax,rax\n");
    write_to_file!(file,"    mov rax,60\n");
    write_to_file!(file,"    mov rdi,0\n");
    write_to_file!(file,"    syscall\n");
    write_to_file!(file,"section \".bss\"\n");
    write_to_file!(file,"    mem: rb 64000\n");
    run_command!("fasm builds/output.asm");
    run_command!("ld builds/output.o -o builds/output");
    if run 
    {
        run_command!("builds/output");
    }
}