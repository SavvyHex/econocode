// Energy cost estimation for each IR operation
#[derive(Debug, Clone)]
pub enum IROp {
    // Integer operations (32-bit and 64-bit)
    Add32(String, String),
    Add64(String, String),
    Sub32(String, String),
    Sub64(String, String),
    Mul32(String, String),
    Mul64(String, String),
    Div32(String, String),
    Div64(String, String),
    
    // Floating point operations
    FAdd32(String, String),
    FAdd64(String, String),
    FSub32(String, String),
    FSub64(String, String),
    FMul32(String, String),
    FMul64(String, String),
    FDiv32(String, String),
    FDiv64(String, String),
    
    // Memory operations
    LoadVar32(String),
    LoadVar64(String),
    StoreVar32(String, String),
    StoreVar64(String, String),
    LoadMem(String),  // General memory load
    
    // Constants
    Const32(i32),
    Const64(i64),
    FConst32(f32),
    FConst64(f64),
}

impl IROp {
    /// Energy cost estimation based on CPU cycles and power consumption
    /// References:
    /// - Intel x86 Instruction Latency Tables
    /// - CPU architecture: 64-bit ops slightly more expensive than 32-bit
    /// - Floating point ~1.5-2x cost of integer
    /// - Division 10-20x multiplication
    /// - Memory access: L1 hit ~4 cycles, cache miss ~50 cycles
    pub fn energy_cost(&self) -> u32 {
        match self {
            // Integer arithmetic (32-bit)
            IROp::Add32(_, _) => 1,
            IROp::Sub32(_, _) => 1,
            IROp::Mul32(_, _) => 3,
            IROp::Div32(_, _) => 20,
            
            // Integer arithmetic (64-bit) - slightly higher due to wider data
            IROp::Add64(_, _) => 1,
            IROp::Sub64(_, _) => 1,
            IROp::Mul64(_, _) => 5,  // 64-bit mul is more expensive
            IROp::Div64(_, _) => 40, // 64-bit div much slower
            
            // Floating point (32-bit)
            IROp::FAdd32(_, _) => 2,
            IROp::FSub32(_, _) => 2,
            IROp::FMul32(_, _) => 4,
            IROp::FDiv32(_, _) => 40,
            
            // Floating point (64-bit) - higher precision, higher cost
            IROp::FAdd64(_, _) => 3,
            IROp::FSub64(_, _) => 3,
            IROp::FMul64(_, _) => 6,
            IROp::FDiv64(_, _) => 80,
            
            // Memory operations
            IROp::LoadVar32(_) => 4,  // L1 cache hit
            IROp::LoadVar64(_) => 5,  // Slightly heavier for 64-bit
            IROp::StoreVar32(_, _) => 4,
            IROp::StoreVar64(_, _) => 5,
            IROp::LoadMem(_) => 50,  // Average cache miss
            
            // Constants - minimal cost
            IROp::Const32(_) => 1,
            IROp::Const64(_) => 1,
            IROp::FConst32(_) => 1,
            IROp::FConst64(_) => 1,
        }
    }
}
use std::fmt;

#[derive(Debug, Clone)]
pub enum BinOp { 
    Add(crate::ast::Type), 
    Sub(crate::ast::Type), 
    Mul(crate::ast::Type), 
    Div(crate::ast::Type) 
}

#[derive(Debug, Clone)]
pub enum CmpIR {
    Eq, Ne, Lt, Le, Gt, Ge,
}

#[derive(Debug, Clone)]
pub enum Instr {
    LoadConst(i64, String, crate::ast::Type),
    Move(String, String, crate::ast::Type),
    BinOp(BinOp, String, String, String),
    Cmp(CmpIR, String, String, String),
    Label(String),
    BrIf(String, String, String), // cond, then_label, else_label
    Jmp(String),
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instr::LoadConst(v, d, _) => write!(f, "{} = const {}", d, v),
            Instr::Move(s, d, _) => write!(f, "{} = {}", d, s),
            Instr::BinOp(op, a, b, d) => {
                let (s, typ) = match op { 
                    BinOp::Add(t) => ("add", t), 
                    BinOp::Sub(t) => ("sub", t), 
                    BinOp::Mul(t) => ("mul", t), 
                    BinOp::Div(t) => ("div", t) 
                };
                write!(f, "{} = {} {}, {} ({:?})", d, s, a, b, typ)
            }
            Instr::Cmp(op, a, b, d) => {
                let s = match op { CmpIR::Eq=>"cmpeq", CmpIR::Ne=>"cmpne", CmpIR::Lt=>"cmplt", CmpIR::Le=>"cmple", CmpIR::Gt=>"cmpgt", CmpIR::Ge=>"cmpge" };
                write!(f, "{} = {} {}, {}", d, s, a, b)
            }
            Instr::Label(name) => write!(f, "{}:", name),
            Instr::BrIf(c, t, e) => write!(f, "br_if {}, {}, {}", c, t, e),
            Instr::Jmp(l) => write!(f, "jmp {}", l),
        }
    }
}
