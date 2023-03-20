mod ast;
mod parser;

pub use ast::keyword;
pub use ast::{
    Block, CtrlColon, CtrlComma, CtrlDot, CtrlLBrace, CtrlLBracet, CtrlLParan, CtrlRBrace,
    CtrlRBracet, CtrlRParan, CtrlRightArrow, CtrlSemiColon, CtrlSlash, CtrlStar,
    CtrlThickRightArrow, Expr, ExprBinary, ExprCall, ExprLit, ExprVar, Ident, Item, ItemFn, Lit,
    LitBool, LitChar, LitInt, LitStr, Op, OpAdd, OpDiv, OpEqual, OpEqualEqual, OpGeq, OpGrt, OpLeq,
    OpLes, OpMul, OpNeq, OpNot, OpSub, Param, Statement, Type,
};

use parser::Parser;

use crate::lexer::{Token, TokenStream};

pub fn parse(stream: TokenStream) -> Result<Item, Vec<String>> {
    Parser::new(stream).parse()
}
