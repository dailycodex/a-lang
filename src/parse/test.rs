use pretty_assertions::assert_eq;
use super::*;
use crate::lexer::lex;

pub fn snapshot_lexing(input: &str) -> String {
    let tokens = match lex(input) {
        Ok(l) => l,
        Err(e) => return e.into_iter().map(|i| format!("{i}\n")).collect(),
    };
    let mut tokens = std::collections::VecDeque::from(tokens.stream);
    let mut output = String::new();
    let mut idx = 0;
    for (row, line) in input.lines().enumerate() {
        output += line;
        output += "\n";
        let mut line_count = 0;
        while let Some(tok) = tokens.pop_front() {
            // if tok.span().start != tok.span().end {
            //     panic!("We haven't handled this yet");
            // }

            if tok.span().line > idx && tok.span().line != row {
                tokens.push_front(tok);
                break;
            }
            output += &" ".repeat(tok.span().start - idx);
            output += &"^".repeat(tok.span().end - tok.span().start);
            output += &format!(" {tok:?}");
            line_count += tok.span().len();
            output += "\n"
        }
        idx += line_count;
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

// #[test]
// fn test_primary() {
//     let tokens = lex("1").unwrap();
//     let ast = parse(tokens).unwrap();
//     insta::assert_yaml_snapshot!(ast);
// }
