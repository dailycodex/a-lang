#![allow(unused)]
mod ast;
// mod new_parser;
mod parser;

pub use ast::keyword;
pub use ast::*;

use parser::Parser;

use crate::lexer::{Token, TokenStream};

pub fn parse(stream: TokenStream) -> Result<Vec<Item>, Vec<String>> {
    // new_parser::parse(stream)
    Parser::new(stream).parse()
}
