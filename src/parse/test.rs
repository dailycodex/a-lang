use super::*;
use crate::lexer::lex;
use pretty_assertions::assert_eq;

macro_rules! snapshot {
    ($name:tt, $path:tt) => {
        #[test]
        fn $name() {
            use super::parse;
            use crate::lexer::lex;
            let contents = include_str!($path);
            let tokens = lex(contents).unwrap();
            let ast = parse(tokens).unwrap();
            let ast_string = ast
                .iter()
                .map(|node| format!("{node}\n"))
                .collect::<String>();
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("testdata/output/");
            settings.bind(|| {
                insta::assert_snapshot!(ast_string);
            });
        }
    };
}

snapshot!(parse_binary, "testdata/snapshots/binary.a");
snapshot!(parse_ifelse, "testdata/snapshots/ifelse.a");
