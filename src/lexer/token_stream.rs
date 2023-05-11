use crate::lexer::Token;

#[derive(Debug)]
pub struct TokenStream {
    pub(crate) stream: Vec<Box<dyn Token>>,
    idx: usize,
}

impl TokenStream {
    pub(crate) fn new(stream: Vec<Box<dyn Token>>) -> Self {
        Self { stream, idx: 0 }
    }

    pub fn is_not_at_end(&self) -> bool {
        self.idx < self.stream.len() - 1
    }

    // pub fn next(&mut self) -> Option<&dyn Token> {
    //     let result = self.stream.get(self.idx).map(|t| &**t)
    //     self.idx += 1;
    //     result
    // }

    pub fn next_as<Expected>(&mut self) -> Option<&Expected>
    where
        Expected: Token,
    {
        let result = self
            .stream
            .get(self.idx)
            .and_then(|i| i.as_any().downcast_ref::<Expected>());
        self.idx += 1;
        result
    }

    pub fn next_if<Expected>(&mut self) -> Option<&Expected>
    where
        Expected: Token,
    {
        let result = self
            .stream
            .get(self.idx)
            .and_then(|i| i.as_any().downcast_ref::<Expected>());
        // if result.is_none() {
        //     return None;
        // }
        result?;
        self.idx += 1;
        result
    }

    // pub fn previous<'a>(&'a mut self) -> Option<&'a Box<dyn Token>> {
    //     if self.idx == 0 {
    //         return None;
    //     }
    //     self.stream.get(self.idx.saturating_sub(1))
    // }

    pub fn is_peek_a<Expected: 'static>(&self) -> bool
    where
        Expected: Token,
    {
        self.stream
            .get(self.idx)
            .and_then(|i| i.as_any().downcast_ref::<Expected>())
            .is_some()
    }

    pub fn peek_blind(&mut self) -> Option<&dyn Token> {
        self.stream.get(self.idx).map(|i| &**i)
    }

    pub fn peek<Expected>(&self) -> Option<&Expected>
    where
        Expected: Token,
    {
        self.stream
            .get(self.idx)
            .and_then(|i| i.as_any().downcast_ref::<Expected>())
    }

    // pub fn peek_nth<'a, Expected>(&'a self, nth: usize) -> Option<&'a Expected>
    // where
    //     Expected: Token,
    // {
    //     self.stream
    //         .get(self.idx + nth.saturating_sub(1))
    //         .and_then(|i| i.as_any().downcast_ref::<Expected>())
    // }

    // pub fn reset(&mut self) {
    //     self.idx = 0;
    // }
}

// #[test]
// fn test_peek() {
//     use super::lex;
//     use crate::lexer::Span;
//     use crate::parse::{LitInt, OpAdd};
//     use pretty_assertions::assert_eq;
//     let tokens = lex("1 + 1").unwrap();
//     assert!(tokens.is_peek_a::<LitInt>());
//     assert_eq!(
//         tokens.peek::<LitInt>(),
//         Some(&LitInt::new("1", Span::new(0, 0, 1)))
//     );
//     assert_eq!(
//         tokens.peek_nth::<OpAdd>(2),
//         Some(&OpAdd::new("+", Span::new(0, 2, 3)))
//     );
//     assert_eq!(
//         tokens.peek_nth::<LitInt>(3),
//         Some(&LitInt::new("1", Span::new(0, 4, 5)))
//     );
// }
