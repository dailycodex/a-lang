use super::Span;
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

type Token = Box<dyn super::Token>;

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
    span: Span,
    last_chr_len: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars().peekable(),
            span: Span::default(),
            last_chr_len: 0,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.src.peek()
    }

    fn next(&mut self) -> Option<char> {
        let Some(ch) = self.src.next() else {
            return None;
        };
        self.span.right_shift(ch);
        self.last_chr_len = ch.to_string().as_bytes().len();
        Some(ch)
    }

    fn next_if<F>(&mut self, func: F) -> Option<char>
    where
        F: FnOnce(char) -> bool,
    {
        let Some(c) = self.peek() else {
            return None;
        };
        if func(*c) {
            return self.next();
        }
        None
    }

    fn span(&mut self) -> Span {
        let span = self.span;
        self.span.reset(None);
        span
    }

    fn number(&mut self, c: char) -> Option<Token> {
        let mut number = c.to_string();
        while let Some(c) = self.next_if(|c| c.is_ascii_digit() || c == '_') {
            number.push(c);
        }
        Some(Box::new(LitInt::new(number, self.span())))
    }

    fn ident(&mut self, c: char) -> Option<Token> {
        let mut id = c.to_string();
        while let Some(c) = self.next_if(|c| c.is_ascii_alphanumeric() || c == '_') {
            id.push(c);
        }
        let span = self.span();
        Some(match id.as_str() {
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
        })
    }

    fn string(&mut self) -> Option<Token> {
        let mut string = String::new();
        while let Some(c) = self.next_if(|c| c != '"') {
            string.push(c);
        }
        self.next();
        Some(Box::new(LitStr::new(string, self.span())))
    }

    fn chr(&mut self) -> Option<Token> {
        let mut string = String::new();
        while let Some(c) = self.next_if(|c| c != '\'') {
            string.push(c);
        }
        self.next();

        Some(Box::new(LitChar::new(string, self.span())))
    }
    fn take_while(&mut self, expected: char) {
        while self.next_if(|c| c != expected).is_some() {}
    }

    fn comment(&mut self) -> Option<Token> {
        self.take_while('\n');
        let Some(ch) = self.next() else {
            return None;
        };
        self.parse(ch)
    }

    fn token<T>(&mut self, op: &str) -> Option<Token>
    where
        T: super::Token,
    {
        for _ in 0..op.chars().count().saturating_sub(self.last_chr_len) {
            self.next();
        }
        Some(Box::new(T::new(op.into(), self.span())))
    }

    fn matched(&mut self, ch: char) -> bool {
        matches!(self.peek(), Some(c) if c == &ch)
    }

    fn parse(&mut self, ch: char) -> Option<Token> {
        match ch {
            n @ '0'..='9' => self.number(n),
            i @ ('a'..='z' | 'A'..='Z') => self.ident(i),
            '"' => self.string(),
            '\'' => self.chr(),
            '/' if self.matched('/') => self.comment(),
            '-' if self.matched('>') => self.token::<CtrlRightArrow>("->"),
            '>' if self.matched('=') => self.token::<OpGeq>(">="),
            '<' if self.matched('=') => self.token::<OpLeq>("<="),
            '=' if self.matched('=') => self.token::<OpEqualEqual>("=="),
            '!' if self.matched('=') => self.token::<OpNeq>("!="),
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
            '\n' | '\r' | ' ' | '\0' => {
                let Some(ch) = self.next() else {
                    return None;
                };
                self.span.reset(Some(self.last_chr_len));
                self.parse(ch)
            }
            _ => panic!("unknown char {ch:?}"),
        }
    }

    pub fn lex(mut self) -> Result<Vec<Token>, Vec<String>> {
        let mut tokens = vec![];
        while let Some(ch) = self.next() {
            let Some(token) = self.parse(ch) else {
                break;
            };
            tokens.push(token);
        }
        Ok(tokens)
    }
}
