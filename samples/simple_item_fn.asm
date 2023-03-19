
format ELF64 executable 3
segment readable executable

entry _start

; enter block
__main__:

  push rbp
  mov rbp, rsp
  mov rax, 2
  imul rax, 3
  mov rdi, 1
  add rdi, rax
  mov rsp, rbp
  pop rbp
  ret


_start:
  call __main__
  mov rax, 60
  syscall

segment readable writable
