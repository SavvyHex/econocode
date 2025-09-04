use std::fmt;

#[derive(Debug, Clone)]
pub enum BinOp { Add, Sub, Mul, Div }

#[derive(Debug, Clone)]
pub enum Instr {
    LoadConst(i64, String),
    Move(String, String),
    BinOp(BinOp, String, String, String),
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instr::LoadConst(v, d) => write!(f, "{} = const {}", d, v),
            Instr::Move(s, d) => write!(f, "{} = {}", d, s),
            Instr::BinOp(op, a, b, d) => {
                let s = match op { BinOp::Add => "add", BinOp::Sub => "sub", BinOp::Mul => "mul", BinOp::Div => "div" };
                write!(f, "{} = {} {}, {}", d, s, a, b)
            }
        }
    }
}
