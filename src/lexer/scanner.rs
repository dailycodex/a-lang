use super::{Span, Token};
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

    fn is_end(&self) -> bool {
        self.tp() >= self.len.saturating_sub(1)
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

    fn number(&mut self, c: char) -> Token {
        let mut number = c.to_string();
        while let Some(c) = self.next_if(|c| c.is_ascii_digit() || c == '_') {
            number.push(c);
        }
        Token::Int(number, self.span())
    }

    fn ident(&mut self, c: char) -> Token {
        let mut id = c.to_string();
        while let Some(c) = self.next_if(|c| c.is_ascii_alphanumeric() || c == '_') {
            id.push(c);
        }
        let span = self.span();
        Token::lookup(&id, span).map_or_else(|| Token::Id(id, span), |t| t)
    }

    fn string(&mut self) -> Token {
        let mut string = String::new();
        while let Some(c) = self.next_if(|c| c != '"') {
            string.push(c);
        }
        self.next();
        Token::String(string, self.span())
    }

    fn chr(&mut self, ch: char) -> Token {
        self.next();
        Token::Char(ch, self.span())
    }

    fn comment(&mut self) -> Token {
        while let Some(_) = self.next_if(|c| c != '\n') {}
        let ch = self.next();
        self.parse(ch)
    }

    fn op_token(&mut self, op: &str) -> Token {
        for _ in 0..op.chars().count().saturating_sub(self.last_chr_len) {
            self.next();
        }
        Token::Op(op.into(), self.span())
    }

    fn ctrl_token(&mut self, op: &str) -> Token {
        for _ in 0..op.chars().count().saturating_sub(self.last_chr_len) {
            self.next();
        }
        Token::Ctrl(op.into(), self.span())
    }

    fn parse(&mut self, ch: char) -> Token {
        match ch {
            n @ '0'..='9' => self.number(n),
            i @ ('a'..='z' | 'A'..='Z') => self.ident(i),
            '"' => self.string(),
            '\'' => self.chr(ch),
            '/' if self.peek() == '/' => self.comment(),
            '-' if self.peek() == '>' => self.ctrl_token("->"),
            '>' if self.peek() == '=' => self.op_token(">="),
            '>' if self.peek() == '>' => self.op_token(">>"),
            '<' if self.peek() == '=' => self.op_token("<="),
            '<' if self.peek() == '<' => self.op_token("<<"),
            '=' if self.peek() == '=' => self.op_token("=="),
            '|' if self.peek() == '|' => self.op_token("||"),
            '&' if self.peek() == '&' => self.op_token("&&"),
            '!' if self.peek() == '=' => self.op_token("!="),
            '|' => self.op_token("|"),
            '&' => self.op_token("&"),
            '-' => self.op_token("-"),
            '+' => self.op_token("+"),
            '*' => self.op_token("*"),
            '/' => self.op_token("/"),
            '>' => self.op_token(">"),
            '<' => self.op_token("<"),
            '=' => self.op_token("="),
            '!' => self.op_token("!"),
            '%' => self.op_token("%"),
            '.' => self.op_token("."),
            ',' => self.ctrl_token(","),
            '(' => self.ctrl_token("("),
            ')' => self.ctrl_token(")"),
            '{' => self.ctrl_token("{"),
            '}' => self.ctrl_token("}"),
            '[' => self.ctrl_token("{"),
            ']' => self.ctrl_token("}"),
            ':' => self.ctrl_token(":"),
            ';' => self.ctrl_token(";"),
            'λ' => self.op_token("λ"),
            '\n' | ' ' => {
                let ch = self.next();
                self.span.start = self.span.end.saturating_sub(self.last_chr_len);
                self.parse(ch)
            }
            '\0' => Token::Eof(self.span()),
            c => Token::Error(format!("unknown char '{}'", c), self.span()),
        }
    }

    pub fn lex(mut self) -> Result<Vec<Token>, Vec<String>> {
        let mut tokens = vec![];
        let mut errors = vec![];
        while !self.is_end() {
            let ch = self.next();
            let token = self.parse(ch);
            if !token.is_err() {
                tokens.push(token);
            } else {
                errors.push(token.lexme().to_string())
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(tokens)
    }
}
