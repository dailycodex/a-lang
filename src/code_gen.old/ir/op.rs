use crate::parse;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<parse::Op> for Op {
    fn from(op: parse::Op) -> Self {
        match op {
            parse::Op::Add(..) => Self::Add,
            parse::Op::Sub(..) => Self::Sub,
            parse::Op::Mul(..) => Self::Mult,
            parse::Op::Div(..) => Self::Div,
            parse::Op::Grt(..) => Self::Grt,
            parse::Op::Les(..) => Self::Les,
            parse::Op::Geq(..) => Self::Geq,
            parse::Op::Leq(..) => Self::Leq,
            parse::Op::Neq(..) => Self::Neq,
            // parse::Op::Not(..) => Self::Not,
            _ => unimplemented!(),
        }
    }
}
