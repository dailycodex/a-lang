use super::{IrBlock, Assignment, Label, BasicBlock, BlockType, Input, Op, Reg, Value};

use crate::parse::{
    Binary, Block, Expr, Item, ItemFn, Lit, LitBool, LitInt, Name, Param, Statement,
};

#[derive(Debug, Default)]
pub struct IrGenerator {
    reg_counter: usize,
    code: Vec<BasicBlock>,
    current_block: Vec<BlockType>,
}

impl IrGenerator {
    fn enter(&mut self) {
        self.push(BlockType::Enter);
    }

    fn leave(&mut self) {
        self.push(BlockType::Leave);
        self.push_block();
    }

    fn push(&mut self, block: BlockType) {
        if block.is_enter() {
            self.code.push(BasicBlock::from((self.current_block.clone(), "enter block".into())));
            self.current_block.clear();
            self.current_block.push(block);
            return;
        }
        let is_exit = block.is_exit();
        self.current_block.push(block);
        if is_exit {
            self.code.push(BasicBlock::from((self.current_block.clone(), "exit".into())));
            self.current_block.clear();
        }
    }

    fn push_block(&mut self) {
        if self.current_block.is_empty() {
            return;
        }
        self.code.push(BasicBlock::new(self.current_block.clone()));
        self.current_block.clear();
    }

    fn get_reg(&mut self) -> Reg {
        let r = self.reg_counter;
        self.reg_counter += 1;
        Reg(r)
    }

    fn visit_litbool(&mut self, lit_bool: &LitBool) -> Value {
        Value((lit_bool.value.parse::<bool>().unwrap() as usize).to_string())
    }

    fn visit_litint(&mut self, lit_int: &LitInt) -> Value {
        Value(lit_int.value.to_string())
    }

    fn visit_binary(&mut self, binary: &Binary) -> Reg {
        let mut unfold_expr = |expr| match expr {
            Expr::Lit(ref lit) => self.visit_lit(lit),
            Expr::Binary(ref binary) => Input::Reg(self.visit_binary(binary)),
        };
        let lhs = unfold_expr(Clone::clone(&binary.left));
        let rhs = unfold_expr(Clone::clone(&binary.right));
        let des = self.get_reg();
        self.push(
        IrBlock::default()
            .des(des)
            .op(Op::from(binary.op.kind()))
            .arg1(lhs)
            .arg2(rhs)
            .build());
        // self.current_block.push(BlockType::Assignment(Assignment {
        //     des: des.clone(),
        //     op: Op::from(binary.op.kind()),
        //     x: lhs,
        //     y: rhs,
        // }));
        des
    }

    fn visit_lit(&mut self, lit: &Lit) -> Input {
        match lit {
            Lit::Int(int) => Input::Value(self.visit_litint(int)),
            Lit::Bool(boolean) => Input::Value(self.visit_litbool(boolean)),
        }
    }
    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Lit(..) => unimplemented!(),
            Expr::Binary(ref binary) => {
                self.visit_binary(binary);
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &Statement) {
        let Statement { stmt, .. } = stmt;
        self.visit_expr(stmt)
    }

    fn visit_block(&mut self, block: &Block) {
        for stmt in block.stmts.iter() {
            self.visit_stmt(stmt);
        }
    }

    fn visit_item_fn(&mut self, item_fn: &ItemFn) {
        // prolog
        // Make a label for function
        // pull in all params in local vars
        // get BasicBlock
        // set return
        // epilog
        //
        let ItemFn {
            name,
            params,
            block,
            ret_type,
            ..
        } = item_fn;
        self.push(IrBlock::default().label(name.into()).build());
        self.enter();
        self.visit_block(block);
        self.leave();
        // let basic_block = BlockType::Procedure(Proedure {
        //     label: name.into(),
        //     params: params.iter().map(From::from).collect(),
        //     ret: ret_type.is_some(),
        //     body: BasicBlock { blocks: self.current_block.clone() },
        // });
        // self.current_block.clear();
        // self.current_block.push(basic_block);
    }

    pub fn compile(mut self, item: &Item) -> Vec<BasicBlock> {
        match item {
            Item::Fn(ref item_fn) => self.visit_item_fn(item_fn),
        }
        // self.push_block();
        self.code
    }
}
