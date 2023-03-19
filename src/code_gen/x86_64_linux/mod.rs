mod discripter;
mod generator;
mod instruction;
mod mnemonic;
mod reg64;
mod reg8;
mod reg_preserved_64;
mod x86reg;

use discripter::Discripter;
use instruction::Instruction;
use mnemonic::Mnemonic;
use reg64::Reg64;
use reg8::Reg8;
use reg_preserved_64::RegPreserved64;
use x86reg::X86Reg;

use crate::code_gen::ir::BasicBlock;
use generator::AsmGenerator;

pub fn code_gen(blocks: &[BasicBlock]) -> String {
    let mut asm_generator = AsmGenerator::default();
    for bb in blocks.iter() {
        if let Some(comment) = &bb.comment {
            asm_generator.comment(comment);
        }
        for b in bb.blocks.iter() {

            asm_generator.compile(b)
        }
    }
    asm_generator.return_last_reg_in_exit();
    format!(
        "
format ELF64 executable 3
segment readable executable

entry _start

{asm_generator}

_start:
  call __main__
  mov rax, 60
  syscall

segment readable writable
"
    )
}
