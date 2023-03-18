
format ELF64 executable 3
segment readable executable

entry _start

_start:
mov rax, 2
imul rax, 3
mov rdi, 1
add rdi, rax

  mov rax, 60
  syscall

segment readable writable
