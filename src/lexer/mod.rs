mod scanner;
mod span;
mod token;
mod token_stream;

use scanner::Lexer;
pub use span::Span;
pub use token::Token;
pub use token_stream::TokenStream;

pub fn lex(src: impl Into<String>) -> Result<TokenStream, Vec<String>> {
    Lexer::new(src.into().as_str()).lex().map(TokenStream::new)
}
