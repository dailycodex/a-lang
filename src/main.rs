use std::{fs::OpenOptions, io::Write, process::Command};

mod asm_code_gen;
mod ast;
mod ir_code_gen;
mod lexer;
mod parser;
mod span;
mod token;

fn compile(filename: impl Into<String>) {
    let filename = filename.into();

    let src = std::fs::read_to_string(&filename).expect("failed to read from file");

    let Some(asm_code) = lexer::lex(&src)
        // .and_then(|tokens| Ok(dbg!(tokens)))
        .map_err(|err| dbg!(err))
        .and_then(|tokens| parser::parse(tokens))
        // .and_then(|ast| Ok(dbg!(ast)))
        .map_err(|err| dbg!(err))
        .and_then(|ast| Ok(ir_code_gen::code_gen(&ast)))
        // .and_then(|blocks| Ok(dbg!(blocks)))
        .map_err(|err| dbg!(err))
        .and_then(|blocks| Ok(asm_code_gen::code_gen(&blocks)))
        // .and_then(|asm| Ok(dbg!(asm)))
        .map_err(|err| dbg!(err))
        .ok() else {
            eprintln!("failed to compile");
            std::process::exit(1);
    };

    let Some((filename, _)) = filename.split_once('.') else {
        eprintln!("file name has no extension");
        std::process::exit(1);
    };

    let asm_file = format!("{filename}.asm");

    std::fs::write(&asm_file, asm_code).expect("failed to write to file");

    Command::new("fasm")
        .arg(&asm_file)
        .output()
        .and_then(|output| {
            eprintln!(
                "{}",
                String::from_utf8(output.stdout)
                    .unwrap_or("Failed to read from stdout".to_string())
            );
            eprintln!(
                "{}",
                String::from_utf8(output.stderr)
                    .unwrap_or("Failed to read from stdout".to_string())
            );
            if !output.status.success() {
                std::process::exit(1)
            }
            Ok(())
        })
        .expect("failed to run fasm command");
}

fn main() {
    let Some(filename) = std::env::args().nth(1) else {
        eprintln!("expected a file");
        return;
    };
    compile(filename)
}
