use std::collections::HashMap;
use crate::ir::{Instr, BinOp};

pub struct Interpreter {
    vars: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn execute(&mut self, instrs: &[Instr]) -> Result<i64, String> {
        for instr in instrs {
            match instr {
                Instr::LoadConst(val, dest) => {
                    self.vars.insert(dest.clone(), *val);
                }
                Instr::Move(src, dest) => {
                    if let Some(val) = self.vars.get(src) {
                        self.vars.insert(dest.clone(), *val);
                    } else {
                        return Err(format!("Undefined variable: {}", src));
                    }
                }
                Instr::BinOp(op, left, right, dest) => {
                    let left_val = self.vars.get(left).ok_or(format!("Undefined variable: {}", left))?;
                    let right_val = self.vars.get(right).ok_or(format!("Undefined variable: {}", right))?;
                    let result = match op {
                        BinOp::Add => left_val + right_val,
                        BinOp::Sub => left_val - right_val,
                        BinOp::Mul => left_val * right_val,
                        BinOp::Div => {
                            if *right_val == 0 {
                                return Err("Division by zero".to_string());
                            }
                            left_val / right_val
                        }
                    };
                    self.vars.insert(dest.clone(), result);
                }
            }
        }
        // Return the value of the last temp (assuming it's the result)
        if let Some(last) = instrs.last() {
            match last {
                Instr::LoadConst(_, dest) | Instr::Move(_, dest) | Instr::BinOp(_, _, _, dest) => {
                    self.vars.get(dest).cloned().ok_or("No result".to_string())
                }
            }
        } else {
            Err("No instructions".to_string())
        }
    }
}
