mod scanner;
mod span;
mod token;
mod tokenkind;

pub use span::Span;
pub use token::Token;
pub use tokenkind::TokenKind;
use scanner::Lexer;

pub fn lex(src: &str) -> Result<Vec<Token>, Vec<String>> {
    Lexer::new(src).lex()
}

