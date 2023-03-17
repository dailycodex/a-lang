use crate::ast::*;
use crate::token::TokenKind;
use std::fmt;

pub fn code_gen(ast: &Expr) -> Vec<BasicBlock> {
    let mut basicblocks = vec![];
    let mut blocks = vec![];
    let mut visitor = IrGenerator::default();
    visitor.visit(ast, &mut blocks);
    basicblocks.push(BasicBlock { blocks });
    basicblocks
}

trait Visitor {
    fn visit_litbool(&mut self, lit_bool: &LitBool) -> Value;
    fn visit_litint(&mut self, lit_int: &LitInt) -> Value;
    fn visit_binary(&mut self, binary: &Binary, blocks: &mut Vec<BlockType>) -> Reg;
    fn visit_lit(&mut self, lit: &Lit) -> Input;
    fn visit(&mut self, expr: &Expr, blocks: &mut Vec<BlockType>) {
        match expr {
            Expr::Lit(..) => unimplemented!(),
            Expr::Binary(ref binary) => {
                self.visit_binary(binary, blocks);
            }
        }
    }
}

#[derive(Debug, Default)]
struct IrGenerator {
    reg_counter: usize,
}

impl IrGenerator {
    fn get_reg(&mut self) -> Reg {
        let r = self.reg_counter;
        self.reg_counter += 1;
        Reg(r)
    }
}
impl Visitor for IrGenerator {
    fn visit_litbool(&mut self, lit_bool: &LitBool) -> Value {
        Value((lit_bool.value.parse::<bool>().unwrap() as usize).to_string())
    }

    fn visit_litint(&mut self, lit_int: &LitInt) -> Value {
        Value(lit_int.value.to_string())
    }

    fn visit_binary(&mut self, binary: &Binary, blocks: &mut Vec<BlockType>) -> Reg {
        let mut unfold_expr = |expr| match expr {
            Expr::Lit(ref lit) => self.visit_lit(lit),
            Expr::Binary(ref binary) => Input::Reg(self.visit_binary(binary, blocks)),
        };
        let lhs = unfold_expr(Clone::clone(&binary.left));
        let rhs = unfold_expr(Clone::clone(&binary.right));
        let des = self.get_reg();
        blocks.push(BlockType::Assignment(Assignment {
            des: des.clone(),
            op: Op::from(binary.op.kind()),
            x: lhs,
            y: rhs,
        }));
        des
    }

    fn visit_lit(&mut self, lit: &Lit) -> Input {
        match lit {
            Lit::Int(int) => Input::Value(self.visit_litint(int)),
            Lit::Bool(boolean) => Input::Value(self.visit_litbool(boolean)),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reg(pub usize);

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Value(pub String);

#[derive(Debug, PartialEq, Eq)]
pub struct Label(pub String);

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Div,
    Equal,
    Grt,
    Les,
    Geq,
    Leq,
    Neq,
}

impl From<TokenKind> for Op {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Sub,
            TokenKind::Star => Self::Mult,
            TokenKind::Slash => Self::Div,
            TokenKind::EqEq => Self::Equal,
            TokenKind::Grt => Self::Grt,
            TokenKind::Les => Self::Les,
            TokenKind::Geq => Self::Geq,
            TokenKind::Leq => Self::Leq,
            TokenKind::Neq => Self::Neq,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    Reg(Reg),
    Value(Value),
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlockType {
    Assignment(Assignment),
    Copy { des: Reg, from: Input },
    Conditional { x: Input, y: Input, jump: Label },
    Jump { label: Label },
    Label { label: Label },
}

impl BlockType {
    pub fn is_exit(&self) -> bool {
        match self {
            Self::Conditional { .. } | Self::Jump { .. } => true,
            _ => false,
        }
    }
    pub fn is_enter(&self) -> bool {
        match self {
            Self::Label { .. } => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Assignment {
    pub des: Reg,
    pub op: Op,
    pub x: Input,
    pub y: Input,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BasicBlock {
    pub blocks: Vec<BlockType>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;
    use crate::token::Token;

    #[test]
    fn test_basic_walk_ast() {
        let mut blocks = vec![];
        let mut visitor = IrGenerator::default();
        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Lit(Lit::Int(LitInt {
                value: "1".into(),
                span: Span::default(),
            }))),
            right: Box::new(Expr::Lit(Lit::Int(LitInt {
                value: "2".into(),
                span: Span::default(),
            }))),
            op: Token::Op("+".into(), Span::default()),
        });
        visitor.visit(&expr, &mut blocks);
        let right = vec![BlockType::Assignment(Assignment {
            des: Reg(0),
            op: Op::Add,
            x: Input::Value(Value("1".into())),
            y: Input::Value(Value("2".into())),
        })];
        assert_eq!(blocks, right);
    }

    #[test]
    fn test_nested_binary_walk_ast() {
        let mut blocks = vec![];

        let mut visitor = IrGenerator::default();

        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Lit(Lit::Int(LitInt {
                value: "1".into(),
                span: Span::default(),
            }))),
            right: Box::new(Expr::Binary(Binary {
                left: Box::new(Expr::Lit(Lit::Int(LitInt {
                    value: "1".into(),
                    span: Span::default(),
                }))),
                right: Box::new(Expr::Lit(Lit::Int(LitInt {
                    value: "2".into(),
                    span: Span::default(),
                }))),
                op: Token::Op("+".into(), Span::default()),
            })),
            op: Token::Op("+".into(), Span::default()),
        });

        visitor.visit(&expr, &mut blocks);

        let right = vec![
            BlockType::Assignment(Assignment {
                des: Reg(0),
                op: Op::Add,
                x: Input::Value(Value("1".into())),
                y: Input::Value(Value("2".into())),
            }),
            BlockType::Assignment(Assignment {
                des: Reg(1),
                op: Op::Add,
                x: Input::Value(Value("1".into())),
                y: Input::Reg(Reg(0)),
            }),
        ];

        assert_eq!(blocks, right);
    }
}
