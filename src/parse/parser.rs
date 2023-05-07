#![allow(unused)]
use super::{
    keyword, CtrlColon, CtrlComma, CtrlDot, CtrlLBrace, CtrlLBracet, CtrlLParan, CtrlRBrace,
    CtrlRBracet, CtrlRParan, CtrlRightArrow, CtrlSemiColon, CtrlSlash, CtrlStar,
    CtrlThickRightArrow, Expr, ExprBinary, ExprBlock, ExprCall, ExprIf, ExprLit, ExprVar, Ident,
    Item, ItemFn, Lit, LitBool, LitChar, LitInt, LitStr, Op, OpAdd, OpDiv, OpEqual, OpEqualEqual,
    OpGeq, OpGrt, OpLeq, OpLes, OpMul, OpNeq, OpNot, OpSub, Param, Statement, Type,
};

use crate::lexer::{Span, Token, TokenStream};

pub struct Parser {
    stream: TokenStream,
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

impl Parser {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            stream,
            errors: vec![],
        }
    }

    pub fn parse(mut self) -> Result<Vec<Item>, Vec<String>> {
        let mut ast = vec![];
        while self.stream.is_not_at_end() {
            match self.program() {
                Ok(item) => {
                    ast.push(item);
                }
                Err(error) => {
                    self.errors.push(error);
                }
            }
        }
        if !self.errors.is_empty() {
            return Err(self.errors);
        }
        Ok(ast)
    }

    fn expr_next_if<Expected>(&mut self) -> Option<Expr>
    where
        Expected: Token + Clone,
        Expr: From<Expected>,
    {
        self.stream
            .next_if::<Expected>()
            .cloned()
            .map(|i| Expr::from(i))
    }

    fn program(&mut self) -> Result<Item, String> {
        self.declaration()
    }

    fn declaration(&mut self) -> Result<Item, String> {
        self.item_fn()
    }

    fn item_fn(&mut self) -> Result<Item, String> {
        let keyword_fn = self
            .stream
            .next_if::<keyword::Fn>()
            .cloned()
            .ok_or::<String>("expected fn".into())?;
        let name = self
            .stream
            .next_if::<Ident>()
            .ok_or::<String>("expected a ident".into())?
            .clone();
        let params = self.params()?;
        let ret_type = self.ret_type()?;
        let block = self.block()?;
        Ok(Item::Fn(ItemFn::new(
            keyword_fn, name, params, block, ret_type,
        )))
    }

    fn ret_type(&mut self) -> Result<Option<Type>, String> {
        let Some(_) = self.stream.next_if::<CtrlRightArrow>() else {
            return Ok(None);
        };
        let Some(t) = self.stream.next_if::<Ident>() else {
            return Err("expected return type".into());
        };
        Ok(Some(t.into()))
    }

    fn params(&mut self) -> Result<Vec<Param>, String> {
        self.stream
            .next_if::<CtrlLParan>()
            .ok_or::<String>("expected '('".into())?;
        let mut params = vec![];
        loop {
            if self.stream.is_peek_a::<CtrlRParan>() {
                break;
            }
            let name = match self.stream.next_if::<Ident>() {
                Some(t) => t.clone(),
                None => break,
            };

            self.stream
                .next_if::<CtrlColon>()
                .ok_or::<String>("expected ':' after function param id".into())?;

            let kind = match self.stream.next_if::<Ident>() {
                Some(t) => t,
                None => break,
            };
            params.push((&name, kind).into());
            if let None = self.stream.next_if::<CtrlComma>() {
                break;
            }
        }

        if self.stream.next_if::<CtrlRParan>().is_none() {
            return Err("functions params end with ')'".into());
        }
        Ok(params)
    }

    fn block(&mut self) -> Result<ExprBlock, String> {
        let left_brace = self
            .stream
            .next_if::<CtrlLBrace>()
            .cloned()
            .ok_or::<String>("expected '{'".into())?;
        let mut stmts = vec![];
        while !self.stream.is_peek_a::<CtrlRBrace>() {
            let stmt = self.statement()?;
            stmts.push(stmt);
        }
        let right_brace = self
            .stream
            .next_if::<CtrlRBrace>()
            .cloned()
            .ok_or::<String>("expected '}'".into())?;
        Ok(ExprBlock::new(left_brace, right_brace, stmts))
    }

    fn statement(&mut self) -> Result<Statement, String> {
        let stmt = self.expression();
        let span = stmt.span();
        self.stream
            .next_if::<CtrlSemiColon>()
            .ok_or::<String>("statements end in ';'".into())?;
        Ok(Statement { stmt, span })
    }

    fn expression(&mut self) -> Expr {
        self.if_expression()
    }

    // NOTE: Probably best that these functions return a Option over a Result cause then functions
    // will do there own error reporting at the point of the error.
    // Something like
    // ```
    // self.report(Error::MissingSimiColon(span))
    // ```
    fn if_expression(&mut self) -> Expr {
        if self.stream.peek::<keyword::If>().is_some() {
            // HACK: this implemention is a bit of a hack with all the funcitons not returning a
            // Result.
            let if_token = self.stream.next_as::<keyword::If>().cloned().unwrap();
            let cond = Box::new(self.comparison());
            let then_branch = self.block().expect("failed to get block");
            let else_branch = self.else_branch();
            return ExprIf::new(if_token, cond, then_branch, else_branch).into();
        }
        self.comparison()
    }

    fn else_branch(&mut self) -> Option<(keyword::Else, Box<Expr>)> {
        let Some(keyword_else) = self.stream.next_if::<keyword::Else>().cloned() else {
            return None;
        };
        let block = if self.stream.peek::<keyword::If>().is_some() {
            self.if_expression()
        } else {
            // HACK: Fix this expected
            Expr::Block(self.block().expect("failed to get block"))
        };
        Some((keyword_else, Box::new(block)))
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        loop {
            let op = self
                .stream
                .next_if::<OpGrt>()
                .map(|i| (*i).clone().into())
                .or(self.stream.next_if::<OpLes>().map(|i| (*i).clone().into()))
                .or(self.stream.next_if::<OpGeq>().map(|i| (*i).clone().into()))
                .or(self.stream.next_if::<OpLeq>().map(|i| (*i).clone().into()))
                .or(self
                    .stream
                    .next_if::<OpEqualEqual>()
                    .map(|i| (*i).clone().into()))
                .or(self.stream.next_if::<OpNeq>().map(|i| (*i).clone().into()));
            if op.is_none() {
                break;
            }
            let right = self.term();
            expr = Expr::from(ExprBinary::from((expr, right, op.unwrap())))
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        loop {
            let op = self
                .stream
                .next_if::<OpSub>()
                .map(|i| (*i).clone().into())
                .or(self.stream.next_if::<OpAdd>().map(|i| (*i).clone().into()));
            if op.is_none() {
                break;
            }
            let right = self.factor();
            expr = Expr::from(ExprBinary::from((expr, right, op.unwrap())))
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.call();
        loop {
            let op = self
                .stream
                .next_if::<OpMul>()
                .map(|i| (*i).clone().into())
                .or(self.stream.next_if::<OpDiv>().map(|i| (*i).clone().into()));
            if op.is_none() {
                break;
            }
            let right = self.call();
            expr = Expr::from(ExprBinary::from((expr, right, op.unwrap())))
        }
        expr
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();

        loop {
            let Some(left_paran) = self.stream.next_if::<CtrlLParan>().cloned() else {
                break;
            };
            expr = self.finish_call(expr, left_paran);
        }

        expr
    }

    fn finish_call(&mut self, caller: Expr, left_paran: CtrlLParan) -> Expr {
        let mut args = vec![];
        if !self.stream.is_peek_a::<CtrlRParan>() {
            while !self.stream.is_peek_a::<CtrlRParan>() {
                args.push(self.expression());
                if self.stream.next_if::<CtrlComma>().is_none() {
                    break;
                };
            }
        }
        let Some(right_paran) = self.stream.next_if::<CtrlRParan>().cloned() else {
                // TODO: make this report an error
                panic!("expected a right paran");
        };
        Expr::Call(ExprCall::new(
            Box::new(caller),
            left_paran,
            args,
            right_paran,
        ))
    }

    fn primary(&mut self) -> Expr {
        let Some(expr) = self.expr_next_if::<LitInt>()
            .or(self.expr_next_if::<LitBool>())
            .or(self.expr_next_if::<LitStr>())
            .or(self.expr_next_if::<LitChar>())
            .or(self.expr_next_if::<Ident>()) else {
                // TODO: make this report an error
                panic!("unknown expression '{:?}'", self.stream.peek_blind());
        };
        expr
    }
}
