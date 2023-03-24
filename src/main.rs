use std::process::Command;

mod x86_64_linux;
mod ir;
mod lexer;
mod parse;

fn compile(filename: impl Into<String>) {
    let filename = filename.into();

    let src = std::fs::read_to_string(&filename).expect("failed to read from file");

    let Some(asm_code) = lexer::lex(&src)
        .and_then(|tokens| Ok(dbg!(tokens)))
        .and_then(parse::parse)
        .and_then(|ast| Ok(dbg!(ast)))
        .and_then(ir::code_gen)
        .and_then(|ir| Ok(dbg!(ir)))
        .and_then(x86_64_linux::code_gen)
        .and_then(|asm| Ok(dbg!(asm)))
        .and_then(|asm| Ok((filename, asm)))
        .and_then(write_asm_to_file)
        .and_then(compile_asm_with_fasm)
        .map_err(|err| dbg!(err))
        .ok() else {
            eprintln!("failed to compile");
            std::process::exit(1);
    };
}

fn write_asm_to_file((filename, asm_code) : (String, String)) -> Result<String, Vec<String>>{
    let Some((filename, _)) = filename.split_once('.') else {
        eprintln!("file name has no extension");
        std::process::exit(1);
    };

    let asm_file = format!("{filename}.asm");

    std::fs::write(&asm_file, asm_code)
        .and_then(|_| Ok(asm_file))
        .map_err(|e| vec![e.to_string()])
}

fn compile_asm_with_fasm(asm_file: String) -> Result<(), Vec<String>> {

    Command::new("fasm")
        .arg(asm_file)
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
                std::process::exit(1);
            }
            Ok(())
        })
        .map_err(|e| vec![e.to_string()])
}

fn main() {
    let Some(filename) = std::env::args().nth(1) else {
        eprintln!("expected a file");
        return;
    };
    compile(filename)
}
