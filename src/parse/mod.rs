mod ast;
mod parser;
#[cfg(test)]
mod test;

pub use ast::keyword;
pub use ast::*;

use parser::Parser;

use crate::lexer::TokenStream;

pub fn parse(stream: TokenStream) -> Result<Vec<Item>, Vec<String>> {
    Parser::new(stream).parse()
}
