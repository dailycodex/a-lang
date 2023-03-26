#![allow(unused)]
use super::{Span, Token};
use crate::parse::{
    keyword,
    CtrlColon,
    CtrlComma,
    CtrlDot,
    CtrlLBrace,
    CtrlLBracet,
    CtrlLParan,
    CtrlRBrace,
    CtrlRBracet,
    CtrlRParan,
    CtrlRightArrow,
    // CtrlStar,
    // CtrlSlash,
    CtrlSemiColon,
    Ident,
    LitBool,
    LitChar,
    LitInt,
    LitStr,
    OpAdd,
    OpDiv,
    OpEqual,
    OpEqualEqual,
    OpGeq,
    OpGrt,
    OpLeq,
    OpLes,
    OpMul,
    OpNeq,
    OpNot,
    OpSub,
};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
    len: usize,
    span: Span,
    last_chr_len: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            len: src.len(),
            src: src.chars().peekable(),
            span: Span::default(),
            last_chr_len: 0,
        }
    }

    fn tp(&self) -> usize {
        self.span.end
    }

    fn is_end(&mut self) -> bool {
        self.src.peek().is_none()
        // self.tp() >= self.len.saturating_sub(1)
    }

    fn peek(&mut self) -> char {
        self.src.peek().cloned().unwrap_or('\0')
    }

    fn next(&mut self) -> char {
        let ch = self.src.next().unwrap_or('\0');
        self.span.right_shift(ch);
        self.last_chr_len = ch.to_string().as_bytes().len();
        ch
    }

    fn next_if<F: FnOnce(char) -> bool>(&mut self, func: F) -> Option<char> {
        let c = self.peek();
        if func(c) {
            assert_eq!(c, self.next());
            return Some(c);
        }
        None
    }

    fn span(&mut self) -> Span {
        let span = self.span;
        self.span.start = self.span.end;
        span
    }

    fn number(&mut self, c: char) -> Box<dyn Token> {
        let mut number = c.to_string();
        while let Some(c) = self.next_if(|c| c.is_ascii_digit() || c == '_') {
            number.push(c);
        }
        Box::new(LitInt::new(number, self.span()))
    }

    fn ident(&mut self, c: char) -> Box<dyn Token> {
        let mut id = c.to_string();
        while let Some(c) = self.next_if(|c| c.is_ascii_alphanumeric() || c == '_') {
            id.push(c);
        }
        let span = self.span();
        match id.as_str() {
            "fn" => Box::new(keyword::Fn(span)),
            "struct" => Box::new(keyword::Struct(span)),
            "if" => Box::new(keyword::If(span)),
            "else" => Box::new(keyword::Else(span)),
            "use" => Box::new(keyword::Use(span)),
            "return" => Box::new(keyword::Return(span)),
            "let" => Box::new(keyword::Let(span)),
            "true" => Box::new(LitBool::new(id, span)),
            "false" => Box::new(LitBool::new(id, span)),
            _ => Box::new(Ident::new(id, span)),
        }
    }

    fn string(&mut self) -> Box<dyn Token> {
        let mut string = String::new();
        while let Some(c) = self.next_if(|c| c != '"') {
            string.push(c);
        }
        self.next();
        Box::new(LitStr::new(string, self.span()))
    }

    fn chr(&mut self) -> Box<dyn Token> {
        let mut string = String::new();
        while let Some(c) = self.next_if(|c| c != '\'') {
            string.push(c);
        }
        self.next();

        Box::new(LitChar::new(string, self.span()))
    }

    fn comment(&mut self) -> Box<dyn Token> {
        while let Some(_) = self.next_if(|c| c != '\n') {}
        let ch = self.next();
        self.parse(ch)
    }

    fn token<T>(&mut self, op: &str) -> Box<dyn Token>
    where
        T: Token,
    {
        for _ in 0..op.chars().count().saturating_sub(self.last_chr_len) {
            self.next();
        }
        Box::new(T::new(op.into(), self.span()))
    }

    fn parse(&mut self, ch: char) -> Box<dyn Token> {
        match ch {
            n @ '0'..='9' => self.number(n),
            i @ ('a'..='z' | 'A'..='Z') => self.ident(i),
            '"' => self.string(),
            '\'' => self.chr(),
            '/' if self.peek() == '/' => self.comment(),
            '-' if self.peek() == '>' => self.token::<CtrlRightArrow>("->"),
            '>' if self.peek() == '=' => self.token::<OpGeq>(">="),
            '<' if self.peek() == '=' => self.token::<OpLeq>("<="),
            '=' if self.peek() == '=' => self.token::<OpEqualEqual>("=="),
            // '|' if self.peek() == '|' => self.token::<>("||"),
            // '&' if self.peek() == '&' => self.token::<>("&&"),
            '!' if self.peek() == '=' => self.token::<OpNeq>("!="),
            // // '|' => self.op_token("|"),
            // // '&' => self.op_token("&"),
            '-' => self.token::<OpSub>("-"),
            '+' => self.token::<OpAdd>("+"),
            '*' => self.token::<OpMul>("*"),
            '/' => self.token::<OpDiv>("/"),
            '>' => self.token::<OpGrt>(">"),
            '<' => self.token::<OpLes>("<"),
            '=' => self.token::<OpEqual>("="),
            '!' => self.token::<OpNot>("!"),
            // '%' => self.op_token("%"),
            '.' => self.token::<CtrlDot>("."),
            ',' => self.token::<CtrlComma>(","),
            '(' => self.token::<CtrlLParan>("("),
            ')' => self.token::<CtrlRParan>(")"),
            '{' => self.token::<CtrlLBrace>("{"),
            '}' => self.token::<CtrlRBrace>("}"),
            '[' => self.token::<CtrlLBracet>("{"),
            ']' => self.token::<CtrlRBracet>("}"),
            ':' => self.token::<CtrlColon>(":"),
            ';' => self.token::<CtrlSemiColon>(";"),
            // 'λ' => self.op_token("λ"),
            '\n' | ' ' | '\0' => {
                let ch = self.next();
                self.span.start = self.span.end.saturating_sub(self.last_chr_len);
                self.parse(ch)
            }
            _ => panic!(),
            // '\0' => Token::Eof(self.span()),
            // c => Token::Error(format!("unknown char '{}'", c), self.span()),
        }
    }

    pub fn lex(mut self) -> Result<Vec<Box<dyn Token>>, Vec<String>> {
        let mut tokens = vec![];
        // let mut errors = vec![];
        while !self.is_end() {
            let ch = self.next();
            let token = self.parse(ch);
            tokens.push(token);
            // if !token.is_err() {
            //     tokens.push(token);
            // } else {
            //     errors.push(token.lexme().to_string())
            // }
        }
        // if !errors.is_empty() {
        //     return Err(errors);
        // }
        Ok(tokens)
    }
}
