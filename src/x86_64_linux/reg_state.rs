#![warn(clippy::upper_case_acronyms)]
use super::{X86Reg, X86RegParam, X86RegRet};
use crate::ir::Reg;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct RegState {
    args: [bool; 6],
    // _scratch: [bool; 9],
    // _preserved: [bool; 7],
    ret: [bool; 2],
    in_use: HashMap<Reg, X86Reg>,
    last_used: Option<(Reg, X86Reg)>,
}

impl RegState {
    fn push_reg(&mut self, reg: &Reg, xreg: X86Reg) {
        self.last_used = Some((*reg, xreg));
        self.in_use.insert(*reg, xreg);
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }

    pub fn release_reg(&mut self, reg: &Reg) {
        self.in_use.get(reg).and_then(|r| {
            match r {
                X86Reg::RegRet(reg) => self.ret[(*reg) as usize] = false,
                X86Reg::RegParam(reg) => self.args[(*reg) as usize] = false,
                X86Reg::Reg64(..) => unreachable!(),
                X86Reg::Reg32(..) => unreachable!(),
                X86Reg::Reg16(..) => unreachable!(),
                X86Reg::RegHigh8(..) => unreachable!(),
                X86Reg::RegLow8(..) => unreachable!(),
            }
            None::<X86Reg>
        });
    }

    pub fn get_reg(&mut self, reg: &Reg) -> X86Reg {
        self.in_use.get(reg).copied().unwrap_or_else(|| {
            let xreg = self.get_param_reg(reg);
            self.push_reg(reg, xreg);
            xreg
        })
    }

    pub fn get_param_reg(&mut self, reg: &Reg) -> X86Reg {
        let xreg = self
            .args
            .iter_mut()
            .enumerate()
            .find(|(_, r)| !**r)
            .map(|(i, r)| {
                *r = true;
                X86RegParam::from(i).into()
            })
            .unwrap();
        self.push_reg(reg, xreg);
        xreg
    }

    pub fn get_ret_reg(&mut self) -> X86Reg {
        X86RegRet::RAX.into()
        // self.ret
        //     .iter_mut()
        //     .enumerate()
        //     .find(|(_, r)| !**r)
        //     .and_then(|(i, r)| {
        //         *r = true;
        //         Some(X86RegRet::from(i).into())
        //     })
        //     .unwrap_or(X86RegRet::RAX.into())
    }
}
