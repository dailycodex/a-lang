use super::{
    Item,
    Statement,
    LitInt,
    LitBool,
    Expr,
    Binary,
    Token,
    TokenKind
};
use std::iter::Peekable;
use std::slice::Iter;


pub struct Parser<'a> {
    stream: Peekable<Iter<'a, Token>>,
    errors: Vec<String>,
}

// declaration
// statement
// expression
// equality
// comparison
// term
// factor
// unary
// primary

impl<'a> Parser<'a> {
    pub fn new(stream: Peekable<Iter<'a, Token>>) -> Self {
        Self {
            stream,
            errors: vec![],
        }
    }

    pub fn parse(mut self) -> Result<Item, Vec<String>> {
        match self.statement() {
            Ok(item) => {
                if !self.errors.is_empty() {
                    return Err(self.errors);
                }
                Ok(item)
            }
            Err(error) => {
                self.errors.push(error);
                Err(self.errors)
            }
        }
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

    fn statement(&mut self) -> Result<Item, String> {
        let stmt = self.expression();
        let span = stmt.span();
        let Some(TokenKind::SemiColon) = self.stream.peek().map(|i| i.kind()) else {
            return Err(format!("Statements end in ';' {stmt:?}"));
        };
        Ok(Item::Statement(Statement {
            stmt,
            span,
        }))
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
