use super::{
    keyword, Block, CtrlColon, CtrlComma, CtrlDot, CtrlLBrace, CtrlLBracet, CtrlLParan, CtrlRBrace,
    CtrlRBracet, CtrlRParan, CtrlRightArrow, CtrlSemiColon, CtrlSlash, CtrlStar,
    CtrlThickRightArrow, Expr, ExprBinary, ExprCall, ExprLit, ExprVar, Ident, Item, ItemFn, Lit,
    LitBool, LitChar, LitInt, LitStr, Op, OpAdd, OpDiv, OpEqual, OpEqualEqual, OpGeq, OpGrt, OpLeq,
    OpLes, OpMul, OpNeq, OpNot, OpSub, Param, Statement, Type,
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

    //     fn match_on(&mut self, expected: &[TokenKind]) -> bool {
    //         let kind = self
    //             .stream
    //             .peek()
    //             .as_ref()
    //             .map(|t| t.kind())
    //             .unwrap_or(TokenKind::Eof);
    //         for token_kind in expected {
    //             if token_kind == &kind {
    //                 return true;
    //             }
    //         }
    //         false
    //     }
    //
    fn program(&mut self) -> Result<Item, String> {
        self.declaration()
    }

    fn declaration(&mut self) -> Result<Item, String> {
        self.item_fn()
    }

    fn item_fn(&mut self) -> Result<Item, String> {
        let start_span = self
            .stream
            .next_if::<keyword::Fn>()
            .ok_or::<String>("expected fn".into())?
            .span();
        let name = self
            .stream
            .next_if::<Ident>()
            .ok_or::<String>("expected a ident".into())?
            .clone();
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
            dbg!(&self.stream);
            return Err("functions params end with ')'".into());
        }
        // .ok_or::<String>({
        //     "functions params end with ')'".into()
        // })?;
        Ok(params)
    }

    fn block(&mut self) -> Result<Block, String> {
        let start_span = self
            .stream
            .next_if::<CtrlLBrace>()
            .ok_or::<String>("expected '{'".into())?
            .span();
        let mut stmts = vec![];
        while !self.stream.is_peek_a::<CtrlRBrace>() {
            let stmt = self.statement()?;
            stmts.push(stmt);
        }
        let end_span = self
            .stream
            .next_if::<CtrlRBrace>()
            .ok_or::<String>("expected '}'".into())?
            .span();
        let span = Span::new(start_span.line, start_span.start, end_span.end);
        Ok(Block { stmts, span })
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
        self.comparison()
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        // while self.match_on(&[Grt, Les, Geq, Leq, EqEq, Neq]) {
        fn is_one_of<A, B, C, D, E, F>(stream: &mut TokenStream) -> Option<Op>
        where
            A: Token + Clone + Into<Op>,
            B: Token + Clone + Into<Op>,
            C: Token + Clone + Into<Op>,
            D: Token + Clone + Into<Op>,
            E: Token + Clone + Into<Op>,
            F: Token + Clone + Into<Op>,
        {
            stream
                .next_if::<A>()
                .map(|i| (*i).clone().into())
                .or(stream.next_if::<B>().map(|i| (*i).clone().into()))
                .or(stream.next_if::<C>().map(|i| (*i).clone().into()))
                .or(stream.next_if::<D>().map(|i| (*i).clone().into()))
                .or(stream.next_if::<E>().map(|i| (*i).clone().into()))
                .or(stream.next_if::<F>().map(|i| (*i).clone().into()))
        }
        loop {
            let op = is_one_of::<OpGrt, OpLes, OpGeq, OpLeq, OpEqualEqual, OpNeq>(&mut self.stream);
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
        fn is_one_of<A, B>(stream: &mut TokenStream) -> Option<Op>
        where
            A: Token + Clone + Into<Op>,
            B: Token + Clone + Into<Op>,
        {
            stream
                .next_if::<A>()
                .map(|i| (*i).clone().into())
                .or(stream.next_if::<B>().map(|i| (*i).clone().into()))
        }
        loop {
            let op = is_one_of::<OpSub, OpAdd>(&mut self.stream);
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
        fn is_one_of<A, B>(stream: &mut TokenStream) -> Option<Op>
        where
            A: Token + Clone + Into<Op>,
            B: Token + Clone + Into<Op>,
        {
            stream
                .next_if::<A>()
                .map(|i| (*i).clone().into())
                .or(stream.next_if::<B>().map(|i| (*i).clone().into()))
        }
        loop {
            let op = is_one_of::<OpMul, OpDiv>(&mut self.stream);
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
            if self.stream.next_if::<CtrlLParan>().is_none() {
                break;
            }
            expr = self.finish_call(expr);
        }

        expr
    }

    fn finish_call(&mut self, caller: Expr) -> Expr {
        let start_span = caller.span();
        let mut args = vec![];
        if !self.stream.is_peek_a::<CtrlRParan>() {
            while !self.stream.is_peek_a::<CtrlRParan>() {
                args.push(self.expression());
                if self.stream.next_if::<CtrlComma>().is_none() {
                    break;
                };
            }
        }
        // FIXME: this should be fixed on the unwrap
        let end_span = self
            .stream
            .next_if::<CtrlRParan>()
            .map(|t| t.span())
            .unwrap();
        let span = Span::new(start_span.line, start_span.start, end_span.end);
        Expr::Call(ExprCall {
            caller: Box::new(caller),
            args,
            span,
        })
    }

    fn primary(&mut self) -> Expr {
        self.stream
            .next_if::<LitInt>()
            .map(|i| Expr::from(i.clone()))
            .or(self
                .stream
                .next_if::<LitBool>()
                .map(|i| Expr::from(i.clone())))
            .or(self
                .stream
                .next_if::<LitStr>()
                .map(|i| Expr::from(i.clone())))
            .or(self
                .stream
                .next_if::<LitChar>()
                .map(|i| Expr::from(i.clone())))
            .or(self
                .stream
                .next_if::<Ident>()
                .map(|i| Expr::from(i.clone())))
            .unwrap()
    }
}
