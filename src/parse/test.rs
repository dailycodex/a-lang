use pretty_assertions::assert_eq;
use super::*;
use crate::lexer::lex;

macro_rules! snapshot {
    ($name:tt, $path:tt) => {
        #[test]
        fn $name() {
            use crate::lexer::lex;
            use super::parse;
            let contents = include_str!($path);
            let tokens = lex(contents).unwrap();
            let ast = parse(tokens).unwrap();
            let ast_string = ast.iter().map(|node| {
                format!("{node}\n")
            }).collect::<String>();
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("testdata/output/");
            settings.bind(|| {
                insta::assert_snapshot!(ast_string);
            });
        }
    };
}

snapshot!(parse_binary, "testdata/snapshots/binary.a");
