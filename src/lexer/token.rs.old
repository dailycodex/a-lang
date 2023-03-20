use super::{Span, TokenKind};

macro_rules! is_a {
    ($kind:ident, $id:ident, $name:ident) => {
        pub fn $name(&self) -> bool {
            match self {
                $kind::$id(..) => true,
                _ => false,
            }
        }
    };
}

#[derive(Debug, Clone)]
pub enum Token {
    KeyWord(String, Span),
    Id(String, Span),
    Int(String, Span),
    Op(String, Span),
    Ctrl(String, Span),
    String(String, Span),
    Char(char, Span),
    Error(String, Span),
    Eof(Span),
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        match &self {
            Self::KeyWord(ref word, ..) => match word.as_str() {
                "true" => TokenKind::True,
                "false" => TokenKind::False,
                "use" => TokenKind::Use,
                "if" => TokenKind::If,
                "else" => TokenKind::Else,
                "struct" => TokenKind::Struct,
                "fn" => TokenKind::Fn,
                "return" => TokenKind::Return,
                "let" => TokenKind::Let,
                _ => unreachable!(),
            },
            Self::Id(..) => TokenKind::Id,
            Self::Int(..) => TokenKind::Int,
            Self::Ctrl(ctrl, ..) => match ctrl.as_str() {
                ";" => TokenKind::SemiColon,
                ":" => TokenKind::Colon,
                "(" => TokenKind::LParan,
                ")" => TokenKind::RParan,
                "[" => TokenKind::LBracet,
                "]" => TokenKind::RBracet,
                "{" => TokenKind::LBrace,
                "}" => TokenKind::RBrace,
                "->" => TokenKind::RightArrow,
                "=>" => TokenKind::ThickRightArrow,
                "," => TokenKind::Comma,
                _ => unreachable!(),
            },
            Self::Op(op, ..) => match op.as_str() {
                "+" => TokenKind::Plus,
                "+=" => TokenKind::PlusEq,
                "-" => TokenKind::Minus,
                "*" => TokenKind::Star,
                "/" => TokenKind::Slash,
                "==" => TokenKind::EqEq,
                "=" => TokenKind::Eq,
                "!=" => TokenKind::Neq,
                ">=" => TokenKind::Geq,
                "<=" => TokenKind::Leq,
                "<" => TokenKind::Les,
                ">" => TokenKind::Grt,
                "!" => TokenKind::Not,
                "|" => TokenKind::Pipe,
                "." => TokenKind::Dot,
                _ => unimplemented!(),
            },
            Self::String(..) => TokenKind::String,
            Self::Char(..) => TokenKind::Char,
            Self::Eof(..) => TokenKind::Eof,
            _ => unimplemented!(),
        }
    }
    pub fn lexme(&self) -> String {
        match &self {
            Self::KeyWord(i, ..) => i.to_string(),
            Self::Id(i, ..) => i.to_string(),
            Self::Int(i, ..) => i.to_string(),
            Self::Ctrl(i, ..) => i.to_string(),
            Self::Op(i, ..) => i.to_string(),
            Self::String(i, ..) => i.to_string(),
            Self::Char(i, ..) => i.to_string(),
            Self::Error(i, ..) => i.to_string(),
            Self::Eof(..) => "Eof".to_string(),
        }
    }

    pub fn span(&self) -> Span {
        match &self {
            Self::KeyWord(.., span) => *span,
            Self::Int(.., span) => *span,
            Self::Id(.., span) => *span,
            Self::Ctrl(.., span) => *span,
            Self::Op(.., span) => *span,
            Self::String(.., span) => *span,
            Self::Char(.., span) => *span,
            Self::Eof(span) => *span,
            _ => unreachable!(),
        }
    }
    pub fn is_keyword_of(&self, expect: &str) -> bool {
        match self {
            Token::KeyWord(word, ..) if expect == word => true,
            _ => false,
        }
    }
    pub fn is_op_of(&self, expect: &str) -> bool {
        match self {
            Token::Op(op, ..) if expect == op => true,
            _ => false,
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            Token::Error(..) => true,
            _ => false,
        }
    }

    is_a!(Token, KeyWord, is_keyword);
    is_a!(Token, Int, is_int);
    is_a!(Token, Id, is_id);
    is_a!(Token, Op, is_op);
    is_a!(Token, Eof, is_eof);
    pub fn lookup(ident: &str, span: Span) -> Option<Self> {
        match ident {
            "fn" | "struct" | "if" | "else" | "use" | "return" | "let" | "true" | "false" => {
                Some(Self::KeyWord(ident.into(), span))
            }
            _ => None,
        }
    }
}
