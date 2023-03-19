use super::{
    Binary, Block, Expr, Item, ItemFn, LitBool, LitInt, Name, Param, Statement, Token, TokenKind,
    Type,
};

use crate::lexer::Span;
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
        match self.program() {
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

    fn program(&mut self) -> Result<Item, String> {
        self.declaration()
    }

    fn declaration(&mut self) -> Result<Item, String> {
        self.item_fn()
    }

    fn item_fn(&mut self) -> Result<Item, String> {
        let start_span = self
            .next_if_kind_is(TokenKind::Fn)
            .ok_or::<String>("expected fn".into())?
            .span();
        let name = self.id()?.into();
        let params = self.params()?;
        let ret_type = self.ret_type()?;
        let block = self.block()?;
        let end_span = block.span;
        let span = Span::new(start_span.line, start_span.start, end_span.end);
        Ok(Item::Fn(ItemFn {
            name,
            params,
            block,
            ret_type,
            span,
        }))
    }

    fn next_if_kind_is(&mut self, tk: TokenKind) -> Option<&Token> {
        self.stream.next_if(|t| t.kind() == tk)
    }

    fn id(&mut self) -> Result<&Token, String> {
        self.next_if_kind_is(TokenKind::Id)
            .ok_or("expected ID".into())
    }

    fn ret_type(&mut self) -> Result<Option<Type>, String> {
        let Some(_) = self.next_if_kind_is(TokenKind::RightArrow) else {
            return Ok(None);
        };
        let Some(t) = self.next_if_kind_is(TokenKind::Id) else {
            return Err("expected return type".into());
        };
        Ok(Some(t.into()))
    }

    fn params(&mut self) -> Result<Vec<Param>, String> {
        self.next_if_kind_is(TokenKind::LParan)
            .ok_or::<String>("expected '('".into())?;
        let mut params = vec![];
        while self
            .stream
            .peek()
            .map(|t| t.kind() != TokenKind::RParan)
            .unwrap_or(false)
        {
            let name = match self.stream.next_if(|t| t.kind() == TokenKind::Id) {
                Some(t) => t,
                None => break,
            };

            self.stream
                .next_if(|t| t.kind() == TokenKind::Colon)
                .ok_or::<String>("expected ':' after function param id".into())?;

            let kind = match self.stream.next_if(|t| t.kind() == TokenKind::Id) {
                Some(t) => t,
                None => break,
            };
            params.push((name, kind).into());
            if let None = self.stream.next_if(|t| t.kind() == TokenKind::Comma) {
                break;
            }
        }
        self.next_if_kind_is(TokenKind::RParan)
            .ok_or::<String>("functions params end with ')'".into())?;
        Ok(params)
    }

    fn block(&mut self) -> Result<Block, String> {
        let start_span = self
            .next_if_kind_is(TokenKind::LBrace)
            .ok_or::<String>("expected '{'".into())?
            .span();
        let mut stmts = vec![];
        while self
            .stream
            .peek()
            .map(|t| t.kind() != TokenKind::RBrace)
            .unwrap_or(false)
        {
            let stmt = self.statement()?;
            stmts.push(stmt);
        }
        let end_span = self
            .next_if_kind_is(TokenKind::RBrace)
            .ok_or::<String>("expected '}'".into())?
            .span();
        let span = Span::new(start_span.line, start_span.start, end_span.end);
        Ok(Block { stmts, span })
    }

    fn statement(&mut self) -> Result<Statement, String> {
        let stmt = self.expression();
        let span = stmt.span();
        self.next_if_kind_is(TokenKind::SemiColon)
            .ok_or::<String>("statements end in ';'".into())?;
        Ok(Statement { stmt, span })
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
                    t => unimplemented!("{t:?}"),
                })
            })
            .unwrap()
    }
}
