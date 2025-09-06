
// Energy estimation for real lowering (step2) - updated for 64-bit integer operations
pub fn estimate_energy(instrs: &[Instr]) -> u32 {
    let mut total = 0;
    for instr in instrs {
        total += match instr {
            Instr::LoadConst(_, _) => 1, // Immediate load
            Instr::Move(_, _) => 5, // Memory load (64-bit variable access, cache hit)
            Instr::BinOp(op, _, _, _) => match op {
                BinOp::Add | BinOp::Sub => 1, // 64-bit ALU
                BinOp::Mul => 5, // 64-bit multiplication
                BinOp::Div => 40, // 64-bit division
            },
        };
    }
    total
}
use super::ast::Expr;
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
            Expr::Int(v) => {
                let t = self.fresh();
                self.code.push(Instr::LoadConst(*v, t.clone()));
                t
            }
            Expr::Var(name) => {
                let t = self.fresh();
                self.code.push(Instr::Move(name.clone(), t.clone()));
                t
            }
            Expr::Add(a,b) | Expr::Sub(a,b) | Expr::Mul(a,b) | Expr::Div(a,b) => {
                let la = self.lower_expr(a);
                let lb = self.lower_expr(b);
                let rd = self.fresh();
                let op = match e {
                    Expr::Add(_,_) => BinOp::Add,
                    Expr::Sub(_,_) => BinOp::Sub,
                    Expr::Mul(_,_) => BinOp::Mul,
                    Expr::Div(_,_) => BinOp::Div,
                    _ => unreachable!(),
                };
                self.code.push(Instr::BinOp(op, la, lb, rd.clone()));
                rd
            }
        }
    }
}
