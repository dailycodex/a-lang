use std::process::Command;

mod ir;
mod lexer;
mod parse;
mod x86_64_linux;

const HELP_MESSAGE: &str = "
Usage: a <inputfile>.a [<flags>*]

        SHORT   LONG            DESCRIPTION
        -h    | --help          print this message out
        -dtk  | --debug-tokens  print out token stream created by compiler
        -dast | --debug-ast     print out ast created by compiler
        -dir  | --debug-ir      print out ir code created by compiler
        -dasm | --debug-asm     print out assembly code created by compiler
";

fn print_output<T>(output: bool) -> impl FnOnce(T) -> Result<T, Vec<String>>
where
    T: std::fmt::Debug,
{
    move |t: T| {
        if output {
            return Ok(dbg!(t));
        }
        Ok(t)
    }
}

fn print_output_asm(output: bool) -> impl FnOnce(String) -> Result<String, Vec<String>> {
    move |t: String| {
        if output {
            for line in t.lines() {
                eprintln!("{line}");
            }
        }
        Ok(t)
    }
}

fn print_error_message(err: Vec<String>) -> Vec<String> {
    for e in err.iter() {
        eprintln!("{e}");
    }
    err
}

fn compile(flags: Flags) -> Result<(), Vec<String>> {
    std::fs::read_to_string(&flags.filename)
        .map_err(|e| vec![e.to_string()])
        .and_then(lexer::lex)
        .and_then(print_output(flags.debug_tokens))
        .and_then(parse::parse)
        .and_then(print_output(flags.debug_ast))
        .and_then(ir::code_gen)
        .and_then(print_output(flags.debug_ir))
        .and_then(x86_64_linux::compile_ir_code)
        .and_then(print_output(flags.debug_asm))
        .and_then(x86_64_linux::instruction_to_string)
        .and_then(print_output_asm(flags.debug_asm))
        .map(|asm| (flags.filename, asm))
        .and_then(write_asm_to_file)
        .and_then(compile_asm_with_fasm)
        .map_err(print_error_message)
}
fn start_func_assembly() -> String {
    use x86_64_linux::{Instruction, X86Reg64};
    [
        Instruction::DefLabel("_start".into()),
        Instruction::MoveReg(X86Reg64::RDI.into(), X86Reg64::RAX.into()),
        Instruction::MoveImm(X86Reg64::RAX.into(), 60),
        Instruction::Syscall,
    ]
    .iter()
    .map(ToString::to_string)
    .collect()
}

fn write_asm_to_file((filename, asm_code): (String, String)) -> Result<String, Vec<String>> {
    let Some((filename, _)) = filename.split_once('.') else {
        eprintln!("file name has no extension");
        std::process::exit(1);
    };

    let asm_file = format!("{filename}.asm");
    let header = if cfg!(target_os = "windows") {
        "format pe64 gui"
    } else {
        "format ELF64 executable 3
segment readable executable
"
    };
    let footer = if cfg!(target_os = "windows") {
        ""
    } else {
        "segment readable writable"
    };
    let code = format!(
        "{header}
entry _start__
{asm_code}

{}
{footer}",
        start_func_assembly()
    );

    std::fs::write(&asm_file, code)
        .map(|_| asm_file)
        .map_err(|e| vec![e.to_string()])
}

fn compile_asm_with_fasm(asm_file: String) -> Result<(), Vec<String>> {
    let fasm = if cfg!(target_os = "windows") {
        "./fasm"
    } else {
        "fasm"
    };
    Command::new(fasm)
        .arg(asm_file)
        .output()
        .map(|output| {
            eprintln!(
                "{}",
                String::from_utf8(output.stdout)
                    .unwrap_or("Failed to read from stdout".to_string())
                    .trim()
            );
            eprintln!(
                "{}",
                String::from_utf8(output.stderr)
                    .unwrap_or("Failed to read from stdout".to_string())
                    .trim()
            );
            if !output.status.success() {
                std::process::exit(1);
            }
        })
        .map_err(|e| vec![e.to_string()])
}

#[derive(Debug, Clone)]
struct Flags {
    pub filename: String,
    pub debug_tokens: bool,
    pub debug_ast: bool,
    pub debug_ir: bool,
    pub debug_asm: bool,
}

impl Flags {
    fn new() -> Result<Self, String> {
        let mut debug_tokens = false;
        let mut debug_ast = false;
        let mut debug_ir = false;
        let mut debug_asm = false;
        let Some(filename) = std::env::args().nth(1) else {
            return Err("No file given to parse".into());
        };
        for arg in std::env::args().skip(2) {
            match arg.as_str() {
                "-dtk" | "--debug-tokens" => debug_tokens = true,
                "-dast" | "--debug-ast" => debug_ast = true,
                "-dir" | "--debug-ir" => debug_ir = true,
                "-dasm" | "--debug-asm" => debug_asm = true,
                "-h" | "--help" => return Err(HELP_MESSAGE.into()),
                i => return Err(format!("'{i}' Unknow argument given")),
            }
        }
        Ok(Self {
            filename,
            debug_tokens,
            debug_ast,
            debug_ir,
            debug_asm,
        })
    }
}

fn main() {
    let flags = match Flags::new() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    if let Err(errs) = compile(flags) {
        for i in errs.into_iter() {
            eprintln!("{i}");
        }
        std::process::exit(1);
    }
}
