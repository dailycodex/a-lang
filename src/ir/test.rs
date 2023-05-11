macro_rules! snapshot {
    ($name:tt, $path:tt) => {
        #[test]
        fn $name() {
            use super::*;
            use $crate::lexer::lex;
            use $crate::parse::parse;
            let contents = include_str!($path);
            let tokens = lex(contents).unwrap();
            let ast = parse(tokens).unwrap();
            let ir_code = code_gen(ast).unwrap();
            let result = ir_code
                .iter()
                .map(|i| format!("{i:#?}\n"))
                .collect::<String>();
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("testdata/output/");
            settings.bind(|| {
                insta::assert_snapshot!(result);
            });
        }
    };
}

snapshot!(binary, "testdata/snapshots/binary.a");
snapshot!(ifelse, "testdata/snapshots/ifelse.a");
