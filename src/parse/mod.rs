mod ast;
mod parser;

pub use ast::{
    Item,
    Statement,
    Lit,
    LitInt,
    LitBool,
    Expr,
    Binary,
};

use parser::Parser;

use crate::lexer::{Token, TokenKind};

pub fn parse(tokens: Vec<Token>) -> Result<Item, Vec<String>> {
    let stream = tokens.iter().peekable();
    Parser::new(stream).parse()
}
