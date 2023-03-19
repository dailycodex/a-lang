mod ast;
mod parser;

pub use ast::{
    Binary, Block, Expr, Item, ItemFn, Lit, LitBool, LitInt, Name, Param, Statement, Type,
};

use parser::Parser;

use crate::lexer::{Token, TokenKind};

pub fn parse(tokens: Vec<Token>) -> Result<Item, Vec<String>> {
    let stream = tokens.iter().peekable();
    Parser::new(stream).parse()
}
