use super::*;
use crate::lexer::lex;
use pretty_assertions::assert_eq;

pub fn snapshot_lexing(input: &str) -> String {
    let tokens = match lex(input) {
        Ok(l) => l,
        Err(e) => return e.into_iter().map(|i| format!("{i}\n")).collect(),
    };
    let mut tokens = std::collections::VecDeque::from(tokens.stream);
    let mut output = String::new();
    for (row, line) in input.lines().enumerate() {
        output += line;
        output += "\n";
        while let Some(tok) = tokens.pop_front() {
            // if tok.span().row_start != tok.span().row_end {
            //     panic!("We haven't handled this yet");
            // }

            if tok.span().row_end != row {
                tokens.push_front(tok);
                break;
            }

            output += &" ".repeat(tok.span().col_start);
            output += &"^".repeat(tok.span().len()); // tok.span().col_end - tok.span().col_start);
            output += &format!(" {tok:?}");
            output += "\n"
        }
    }

    output
}

macro_rules! snapshot {
    ($name:tt, $path:tt) => {
        #[test]
        fn $name() {
            let contents = include_str!($path);
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("testdata/output/");
            settings.bind(|| {
                insta::assert_snapshot!(snapshot_lexing(contents));
            });
        }
    };
}

snapshot!(test_lexer, "testdata/snapshots/lexer.a");
snapshot!(lexer_if_else, "testdata/snapshots/ifelse.a");
