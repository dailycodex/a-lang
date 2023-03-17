use crate::ast::*;
use crate::token::{Token, TokenKind};
use std::iter::Peekable;
use std::slice::Iter;

pub fn parse(tokens: Vec<Token>) -> Result<Expr, Vec<String>> {
    let stream = tokens.iter().peekable();
    Parser::new(stream).parse()
}

pub struct Parser<'a> {
    stream: Peekable<Iter<'a, Token>>,
    errors: Vec<String>,
}

// expression
// equality
// comparison
// term
// factor
// unary
// primary

impl<'a> Parser<'a> {
    fn new(stream: Peekable<Iter<'a, Token>>) -> Self {
        Self {
            stream,
            errors: vec![],
        }
    }

    fn parse(mut self) -> Result<Expr, Vec<String>> {
        if !self.errors.is_empty() {
            return Err(self.errors);
        }
        Ok(self.expression())
    }

    fn match_on(&mut self, expected: &[TokenKind]) -> bool {
        let kind = self
            .stream
            .peek()
            .as_ref()
            .map(|t| t.kind())
            .unwrap_or(TokenKind::Eof);
        for token_kind in expected {
            if token_kind == &kind {
                return true;
            }
        }
        false
    }

    fn expression(&mut self) -> Expr {
        self.comparison()
    }

    fn comparison(&mut self) -> Expr {
        use TokenKind::{EqEq, Geq, Grt, Leq, Les, Neq};
        let mut expr = self.term();
        while self.match_on(&[Grt, Les, Geq, Leq, EqEq, Neq]) {
            let op = self.stream.next().unwrap();
            let right = self.term();
            expr = Expr::from(Binary::from((expr, right, op)))
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_on(&[TokenKind::Plus, TokenKind::Minus]) {
            let op = self.stream.next().unwrap();
            let right = self.factor();
            expr = Expr::from(Binary::from((expr, right, op)))
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.primary();
        while self.match_on(&[TokenKind::Star, TokenKind::Slash]) {
            let op = self.stream.next().unwrap();
            let right = self.primary();
            expr = Expr::from(Binary::from((expr, right, op)))
        }
        expr
    }

    fn primary(&mut self) -> Expr {
        self.stream
            .next()
            .and_then(|token| {
                Some(match token.kind() {
                    TokenKind::Int => Expr::from(LitInt::from(token)),
                    TokenKind::True => Expr::from(LitBool::from(token)),
                    TokenKind::False => Expr::from(LitBool::from(token)),
                    _ => unimplemented!(),
                })
            })
            .unwrap()
    }
}
