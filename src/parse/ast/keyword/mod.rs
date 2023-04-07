macro_rules! keyword {
    ($name:ident) => {
        #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(pub crate::lexer::Span);

        impl crate::lexer::Token for $name {
            fn new(_: String, span: crate::lexer::Span) -> Self {
                Self(span)
            }
            fn value(&self) -> String {
                stringify!($name).to_lowercase()
            }
            fn span(&self) -> crate::lexer::Span {
                self.0
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}
keyword!(Use);
keyword!(Let);
keyword!(Struct);
keyword!(True);
keyword!(False);
keyword!(Fn);
keyword!(If);
keyword!(Else);
keyword!(Return);
