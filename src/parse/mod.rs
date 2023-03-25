#![allow(unused)]
mod ast;
mod parser;

pub use ast::keyword;
pub use ast::*;

use parser::Parser;

use crate::lexer::{Token, TokenStream};

pub fn parse(stream: TokenStream) -> Result<Vec<Item>, Vec<String>> {
    Parser::new(stream).parse()
}
