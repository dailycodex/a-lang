mod scanner;
mod span;
mod token;
mod tokenkind;

use scanner::Lexer;
pub use span::Span;
pub use token::Token;
pub use tokenkind::TokenKind;

pub fn lex(src: &str) -> Result<Vec<Token>, Vec<String>> {
    Lexer::new(src).lex()
}
