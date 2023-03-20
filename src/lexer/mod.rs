mod scanner;
mod span;
mod token;
mod token_stream;
// mod tokenkind;

use scanner::Lexer;
pub use span::Span;
pub use token::Token;
pub use token_stream::TokenStream;
// pub use tokenkind::TokenKind;

pub fn lex(src: &str) -> Result<TokenStream, Vec<String>> {
    Lexer::new(src).lex().map(TokenStream::new)
}
