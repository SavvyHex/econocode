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
            crate::ir::Instr::Cmp(_, _, _, _) => 1,
            crate::ir::Instr::Read(_, _) => 50, // treat as a relatively expensive I/O
            crate::ir::Instr::Label(_) => 0,
            crate::ir::Instr::BrIf(_, _, _) => 1,
            crate::ir::Instr::Jmp(_) => 1,
        };
    }
    total
}

use super::ast::{Expr, Type, CmpOp};
use super::ir::{Instr, BinOp, CmpIR};

pub struct Lower {
    next_temp: usize,
    next_label: usize,
    pub code: Vec<Instr>,
}

impl Lower {
    pub fn new() -> Self {
        Self { next_temp: 0, next_label: 0, code: Vec::new() }
    }

    fn fresh(&mut self) -> String {
        let t = format!("t{}", self.next_temp);
        self.next_temp += 1;
        t
    }

    fn fresh_label(&mut self, prefix: &str) -> String {
        let l = format!("{}_{}", prefix, self.next_label);
        self.next_label += 1;
        l
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
            Expr::Read(name, typ) => {
                // Generate an IR read into the named variable
                self.code.push(Instr::Read(name.clone(), typ.clone()));
                name.clone()
            }
            Expr::Assign(name, expr, typ) => {
                let src = self.lower_expr(expr);
                // Move computed temp into the named variable
                self.code.push(Instr::Move(src.clone(), name.clone(), typ.clone()));
                name.clone()
            }
            Expr::Add(a, b) => self.lower_binop(a, b, |typ| BinOp::Add(typ)),
            Expr::Sub(a, b) => self.lower_binop(a, b, |typ| BinOp::Sub(typ)),
            Expr::Mul(a, b) => self.lower_binop(a, b, |typ| BinOp::Mul(typ)),
            Expr::Div(a, b) => self.lower_binop(a, b, |typ| BinOp::Div(typ)),
            Expr::Cmp(op, a, b) => {
                let la = self.lower_expr(a);
                let lb = self.lower_expr(b);
                let rd = self.fresh();
                let irop = match op {
                    CmpOp::Eq=>CmpIR::Eq, CmpOp::Ne=>CmpIR::Ne, CmpOp::Lt=>CmpIR::Lt,
                    CmpOp::Le=>CmpIR::Le, CmpOp::Gt=>CmpIR::Gt, CmpOp::Ge=>CmpIR::Ge,
                };
                self.code.push(Instr::Cmp(irop, la, lb, rd.clone()));
                rd
            }
            Expr::Block(stmts) => {
                let mut last = String::new();
                for s in stmts {
                    last = self.lower_expr(s);
                }
                last
            }
            Expr::IfElse{cond, then_branch, else_branch} => {
                let cond_t = self.lower_expr(cond);
                let then_l = self.fresh_label("then");
                let else_l = self.fresh_label("else");
                let end_l = self.fresh_label("endif");
                self.code.push(Instr::BrIf(cond_t.clone(), then_l.clone(), else_l.clone()));
                // else block first
                self.code.push(Instr::Label(else_l.clone()));
                if let Some(es) = else_branch { for s in es { self.lower_expr(s); } }
                self.code.push(Instr::Jmp(end_l.clone()));
                // then block
                self.code.push(Instr::Label(then_l.clone()));
                for s in then_branch { self.lower_expr(s); }
                self.code.push(Instr::Jmp(end_l.clone()));
                // end
                self.code.push(Instr::Label(end_l.clone()));
                cond_t
            }
            Expr::While{cond, body} => {
                let head = self.fresh_label("while_head");
                let body_l = self.fresh_label("while_body");
                let end = self.fresh_label("while_end");
                self.code.push(Instr::Label(head.clone()));
                let cond_t = self.lower_expr(cond);
                self.code.push(Instr::BrIf(cond_t.clone(), body_l.clone(), end.clone()));
                self.code.push(Instr::Label(body_l.clone()));
                for s in body { self.lower_expr(s); }
                self.code.push(Instr::Jmp(head.clone()));
                self.code.push(Instr::Label(end.clone()));
                cond_t
            }
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
            Expr::Int(_, typ) | Expr::Var(_, typ) | Expr::Assign(_, _, typ) => typ,
            Expr::Read(_, typ) => typ,
            Expr::Add(a, _) | Expr::Sub(a, _) | Expr::Mul(a, _) | Expr::Div(a, _) | Expr::Cmp(_, a, _) => self.get_type(a),
            Expr::Block(stmts) => stmts.last().map(|e| self.get_type(e)).unwrap_or(&Type::I64),
            Expr::IfElse{..} | Expr::While{..} => &Type::I64,
        }
    }
}
