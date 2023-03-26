use super::ast::*;
use crate::lexer::TokenStream;


// declaration
// statement
// expression
// equality
// comparison
// term
// factor
// unary
// primary

type Result<T> = std::result::Result<T, String>;

struct ParseStream {
    ast: Vec<Box<dyn Parse>>
}

impl Parser for ParseStream {
    fn parse(self, tokens: TokenStream) -> Result<Self::Output> {
        
    }
}

trait Parser: Sized {
    type Output;
    fn parse(self, tokens: TokenStream) -> Result<Self::Output>;
}

trait Parse: Sized {
    fn parse(tokens: &mut TokenStream) -> Result<Self>;
}



impl Parse for Lit {
    fn parse(tokens: &mut TokenStream) -> Result<Self> {
        if tokens.peek::<LitInt>().is_some() {
            return tokens
                .next_as::<LitInt>()
                .map(Lit::from)
                .ok_or::<String>("not a lit".into());
        } else if tokens.peek::<LitBool>().is_some() {
            return tokens
                .next_as::<LitBool>()
                .map(Lit::from)
                .ok_or::<String>("not a lit".into());
        } else if tokens.peek::<LitStr>().is_some() {
            return tokens
                .next_as::<LitStr>()
                .map(Lit::from)
                .ok_or::<String>("not a lit".into());
        } else if tokens.peek::<LitChar>().is_some() {
            return tokens
                .next_as::<LitStr>()
                .map(Lit::from)
                .ok_or::<String>("not a lit".into());
        }
        Err("not a lit".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{lex, Span};
    use pretty_assertions::assert_eq;
    #[test]
    fn test_lit() {
        let mut tokens = lex("1").unwrap();
        let left = Lit::parse(&mut tokens).unwrap();
        let right = Lit::Int(LitInt::new("1", Span::new(0,0,1)));
        assert_eq!(left, right);
    }
}


pub fn parse(tokens: TokenStream) -> std::result::Result<Vec<Item>, Vec<String>> {
    todo!()
}

// type ParseResult<T> = Result<(T, TokenStream), (TokenStream, String)>;

// fn parse_factor(mut tokens: TokenStream) -> ParseResult<ExprBinary> {
//     if tokens.peek_nth::<OpMul>(2).is_none() || tokens.peek_nth::<OpDiv>(2).is_none() {
//         return Err((tokens, "Not a factor bin".into()));
//     }
//     let (lhs, tokens) = parse_expr_lit(tokens)?;
//     let op = Op::from(tokens.next());
//     let (rhs, tokens) = parse_expr_lit(tokens)?;
//     // OpMul, OpDiv
//     todo!()
// }
//
// fn parse_expr_lit(mut tokens: TokenStream) -> ParseResult<ExprLit> {
//     parse_lit(tokens)
//         .map(|(i, t)| (ExprLit::from(i), t))
// }
//
// fn parse_lit(mut tokens: TokenStream) -> ParseResult<Lit> {
//     parse_lit_int(tokens)
//         .map(|(i, t)| (Lit::from(i), t))
//         .or_else(|(t, _)| parse_lit_bool(t)
//                  .map(|(i, t)| (Lit::from(i), t)))
//         .or_else(|(t, _)| parse_lit_str(t)
//                  .map(|(i, t)| (Lit::from(i), t)))
//         .or_else(|(t, _)| parse_lit_char(t)
//                  .map(|(i, t)| (Lit::from(i), t)))
// }
//
// fn parse_lit_int(mut tokens: TokenStream) -> ParseResult<LitInt> {
//     let peek = tokens.peek::<LitInt>().cloned();
//     let Some(litint) = peek else {
//         return Err((tokens, "not a litint".into()));
//     };
//     tokens.next();
//     Ok((litint.clone(), tokens))
// }
//
// fn parse_lit_bool(mut tokens: TokenStream) -> ParseResult<LitBool> {
//     let peek = tokens.peek::<LitBool>().cloned();
//     let Some(litbool) = peek else {
//         return Err((tokens, "not a lit bool".into()));
//     };
//     tokens.next();
//     Ok((litbool.clone(), tokens))
// }
//
// fn parse_lit_str(mut tokens: TokenStream) -> ParseResult<LitStr> {
//     let peek = tokens.peek::<LitStr>().cloned();
//     let Some(litstr) = peek else {
//         return Err((tokens, "not a lit str".into()));
//     };
//     tokens.next();
//     Ok((litstr.clone(), tokens))
// }
//
// fn parse_lit_char(mut tokens: TokenStream) -> ParseResult<LitChar> {
//     let peek = tokens.peek::<LitChar>().cloned();
//     let Some(litchar) = peek else {
//         return Err((tokens, "not a litchar".into()));
//     };
//     tokens.next();
//     Ok((litchar.clone(), tokens))
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::lexer::{lex, Span};
//     use pretty_assertions::assert_eq;
//
//     fn setup(input: &str) -> String {
//         lex(input)
//             .and_then(parse)
//             .unwrap()
//             .iter()
//             .map(ToString::to_string)
//             .collect::<String>()
//     }
//     #[test]
//     fn parse_binary() {
//         let src = "1 + 2 * 3";
//         let left = setup(src);
//         let right = "(+ 1 (* 2 3))";
//         assert_eq!(left, right);
//     }
//
//     #[test]
//     fn test_lit_int() {
//         let src = "1";
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_lit_int(tokens).unwrap();
//         let right = LitInt::new("1", Span::new(0, 0, 1));
//         assert_eq!(left, right);
//     }
//
//     #[test]
//     fn test_lit_bool() {
//         let src = "true";
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_lit_bool(tokens).unwrap();
//         let right = LitBool::new("true", Span::new(0, 0, 4));
//         assert_eq!(left, right);
//     }
//
//     #[test]
//     fn test_lit_str() {
//         let src = r#""Hello World""#;
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_lit_str(tokens).unwrap();
//         let right = LitStr::new("Hello World", Span::new(0, 0, 13));
//         assert_eq!(left, right);
//     }
//
//     #[test]
//     fn test_lit_char() {
//         let src = "'H'";
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_lit_char(tokens).unwrap();
//         let right = LitChar::new("H", Span::new(0, 0, 3));
//         assert_eq!(left, right);
//     }
//
//     #[test]
//     fn test_lit() {
//         let src = "1";
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_lit(tokens).unwrap();
//         let right = Lit::from(LitInt::new("1", Span::new(0, 0, 1)));
//         assert_eq!(left, right, "Lit LitInt");
//
//         let src = "true";
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_lit(tokens).unwrap();
//         let right = Lit::from(LitBool::new("true", Span::new(0, 0, 4)));
//         assert_eq!(left, right, "Lit LitBool");
//
//         let src = r#""Hello World""#;
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_lit(tokens).unwrap();
//         let right = Lit::from(LitStr::new("Hello World", Span::new(0, 0, 13)));
//         assert_eq!(left, right, "Lit LitStr");
//
//         let src = "'H'";
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_lit(tokens).unwrap();
//         let right = Lit::from(LitChar::new("H", Span::new(0, 0, 3)));
//         assert_eq!(left, right, "Lit LitChar");
//     }
//
//     #[test]
//     fn test_expr_lit() {
//         let src = "1";
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_expr_lit(tokens).unwrap();
//         let right = ExprLit::from(Lit::from(LitInt::new("1", Span::new(0, 0, 1))));
//         assert_eq!(left, right, "ExprLit Lit LitInt");
//
//         let src = "true";
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_expr_lit(tokens).unwrap();
//         let right = ExprLit::from(Lit::from(LitBool::new("true", Span::new(0, 0, 4))));
//         assert_eq!(left, right, "ExprLit Lit LitBool");
//
//         let src = r#""Hello World""#;
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_expr_lit(tokens).unwrap();
//         let right = ExprLit::from(Lit::from(LitStr::new("Hello World", Span::new(0, 0, 13))));
//         assert_eq!(left, right, "ExprLit Lit LitStr");
//
//         let src = "'H'";
//         let tokens = lex(src).unwrap();
//         let (left, _) = parse_expr_lit(tokens).unwrap();
//         let right = ExprLit::from(Lit::from(LitChar::new("H", Span::new(0, 0, 3))));
//         assert_eq!(left, right, "ExprLit Lit LitChar");
//     }
// }
