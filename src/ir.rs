// Energy cost estimation for each IR operation
#[derive(Debug, Clone)]
pub enum IROp {
    Const(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    LoadVar(String),
}

impl IROp {
    /// Energy cost estimation based on approximate CPU cycles and power consumption
    /// References:
    /// - Intel x86 Instruction Latency Tables (https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html)
    pub fn energy_cost(&self) -> u32 {
        match self {
            IROp::Const(_) => 1,  
            IROp::Add(_, _) => 1, 
            IROp::Sub(_, _) => 1, 
            IROp::Mul(_, _) => 3, 
            IROp::Div(_, _) => 10, 
            IROp::LoadVar(_) => 2, 
        }
    }
}
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
