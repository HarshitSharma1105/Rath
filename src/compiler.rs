use crate::parser::*;


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


macro_rules! run_command {
    ($command:expr) => {{
        use std::process::Command;
        let _ = Command::new("sh").arg("-c").arg($command).status().unwrap();
    }};
}



pub fn compile(program: Vec<Instruction>)
{
    let mut file = create_file!("output.asm");
    write_to_file!(file,"format ELF64\n");
    write_to_file!(file,"section \".text\" executable\n");
    write_to_file!(file,"public main\n");
    write_to_file!(file,"print:\n");
    write_to_file!(file,"    sub     rsp, 264\n");
    write_to_file!(file,"    mov     ecx, 2\n");
    write_to_file!(file,"    mov     BYTE  [rsp], 10\n");
    write_to_file!(file,"    mov     rsi, rsp\n");
    write_to_file!(file,".L2:\n");
    write_to_file!(file,"    mov     rax,rdi\n");
    write_to_file!(file,"    mov     edx, edi\n");
    write_to_file!(file,"    imul    rax, rax, 1717986919\n");
    write_to_file!(file,"    sar     edx, 31\n");
    write_to_file!(file,"    sar     rax, 34\n");
    write_to_file!(file,"    sub     eax, edx\n");
    write_to_file!(file,"    lea     edx, [rax+rax*4]\n");
    write_to_file!(file,"    add     edx, edx\n");
    write_to_file!(file,"    sub     edi, edx\n");
    write_to_file!(file,"    mov     rdx, rcx\n");
    write_to_file!(file,"    add     edi, 48\n");
    write_to_file!(file,"    mov     BYTE  [rsp-1+rcx], dil\n");
    write_to_file!(file,"    mov     edi, eax\n");
    write_to_file!(file,"    lea     rcx, [rcx+1]\n");
    write_to_file!(file,"    test    eax, eax\n");
    write_to_file!(file,"    jne     .L2\n");
    write_to_file!(file,"    mov     r9d, edx\n");
    write_to_file!(file,"    lea     ecx, [rdx-1]\n");
    write_to_file!(file,"    mov     rax, rsi\n");
    write_to_file!(file,"    sar     r9d,1\n");
    write_to_file!(file,"    add     rcx, rsi\n");
    write_to_file!(file,"    add     r9, rsi\n");
    write_to_file!(file,".L3:\n");
    write_to_file!(file,"    movzx   r8d, BYTE  [rcx]\n");
    write_to_file!(file,"    movzx   edi, BYTE  [rax]\n");
    write_to_file!(file,"    add     rax, 1\n");
    write_to_file!(file,"    sub     rcx, 1\n");
    write_to_file!(file,"    mov     BYTE  [rax-1], r8b\n");
    write_to_file!(file,"    mov     BYTE  [rcx+1], dil\n");
    write_to_file!(file,"    cmp     rax, r9\n");
    write_to_file!(file,"    jne     .L3\n");
    write_to_file!(file,"    mov     rdx, rdx\n");
    write_to_file!(file,"    mov     edi, 1\n");
    write_to_file!(file,"    mov     rax,1\n");
    write_to_file!(file,"    syscall\n");
    write_to_file!(file,"    add     rsp, 264\n");
    write_to_file!(file,"    ret\n");
    write_to_file!(file,"main:\n");
    for instruction in program
    {
        match instruction
        {
            Instruction::Push(val) => {write_to_file!(file,format!("    push {}\n",val));}
            Instruction::Add =>
            {
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    add rax,rbx\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::Sub =>
            {
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    sub rax,rbx\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::Mult =>
            {
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    imul rax,rbx\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::Div =>
            {
                write_to_file!(file,"    pop rax\n");
                write_to_file!(file,"    pop rbx\n");
                write_to_file!(file,"    div rbx\n");
                write_to_file!(file,"    push rax\n");
            }
            Instruction::Dump => 
            {
                write_to_file!(file,"    pop rdi\n");
                write_to_file!(file,"    call print\n");
            }
        }
    }
    write_to_file!(file,"    xor rax,rax\n");
    write_to_file!(file,"    ret\n");
    run_command!("fasm output.asm");
    run_command!("cc output.o -o output");
    run_command!("./output");
    run_command!("rm output.o output.asm")
}