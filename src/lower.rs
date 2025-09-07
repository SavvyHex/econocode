// Energy estimation for instructions with types
pub fn estimate_energy(instrs: &[crate::ir::Instr]) -> u32 {
    let mut total = 0;
    for instr in instrs {
        total += match instr {
            crate::ir::Instr::LoadConst(_, _, typ) => match typ {
                crate::ast::Type::I32 => 1,
                crate::ast::Type::I64 => 1,
            },
            crate::ir::Instr::Move(_, _, typ) => match typ {
                crate::ast::Type::I32 => 4,
                crate::ast::Type::I64 => 5,
            },
            crate::ir::Instr::BinOp(op, _, _, _) => match op {
                crate::ir::BinOp::Add(typ) | crate::ir::BinOp::Sub(typ) => match typ {
                    crate::ast::Type::I32 => 1,
                    crate::ast::Type::I64 => 1, // Same cost for ALU ops
                },
                crate::ir::BinOp::Mul(typ) => match typ {
                    crate::ast::Type::I32 => 3,
                    crate::ast::Type::I64 => 5, // Higher for 64-bit
                },
                crate::ir::BinOp::Div(typ) => match typ {
                    crate::ast::Type::I32 => 20,
                    crate::ast::Type::I64 => 40, // Much higher for 64-bit
                },
            },
        };
    }
    total
}

use super::ast::{Expr, Type};
use super::ir::{Instr, BinOp};

pub struct Lower {
    next_temp: usize,
    pub code: Vec<Instr>,
}

impl Lower {
    pub fn new() -> Self {
        Self { next_temp: 0, code: Vec::new() }
    }

    fn fresh(&mut self) -> String {
        let t = format!("t{}", self.next_temp);
        self.next_temp += 1;
        t
    }

    pub fn lower_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::Int(v, typ) => {
                let t = self.fresh();
                self.code.push(Instr::LoadConst(*v, t.clone(), typ.clone()));
                t
            }
            Expr::Var(name, typ) => {
                let t = self.fresh();
                self.code.push(Instr::Move(name.clone(), t.clone(), typ.clone()));
                t
            }
            Expr::Add(a, b) => self.lower_binop(a, b, |typ| BinOp::Add(typ)),
            Expr::Sub(a, b) => self.lower_binop(a, b, |typ| BinOp::Sub(typ)),
            Expr::Mul(a, b) => self.lower_binop(a, b, |typ| BinOp::Mul(typ)),
            Expr::Div(a, b) => self.lower_binop(a, b, |typ| BinOp::Div(typ)),
        }
    }

    fn lower_binop(&mut self, a: &Expr, b: &Expr, op_fn: impl Fn(Type) -> BinOp) -> String {
        let la = self.lower_expr(a);
        let lb = self.lower_expr(b);
        let rd = self.fresh();
        let typ = self.get_type(a).clone();
        let op = op_fn(typ);
        self.code.push(Instr::BinOp(op, la, lb, rd.clone()));
        rd
    }

    fn get_type<'a>(&self, e: &'a Expr) -> &'a Type {
        match e {
            Expr::Int(_, typ) | Expr::Var(_, typ) => typ,
            Expr::Add(a, _) | Expr::Sub(a, _) | Expr::Mul(a, _) | Expr::Div(a, _) => self.get_type(a),
        }
    }
}
