
format ELF64 executable 3
segment readable executable

entry _start

_start:
mov rax, 4
cmp rax, 2
setne al
movzx rax, al
mov rdi, rax

  mov rax, 60
  syscall

segment readable writable
