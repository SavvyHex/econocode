use std::collections::HashMap;
use crate::ir::{Instr, BinOp, CmpIR};
use std::io::{self, Write};

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
        // Pre-index labels
        let mut labels: HashMap<String, usize> = HashMap::new();
        for (i, instr) in instrs.iter().enumerate() {
            if let Instr::Label(name) = instr { labels.insert(name.clone(), i); }
        }
        let mut pc: usize = 0;
        let mut last_dest: Option<String> = None;
        while pc < instrs.len() {
            match &instrs[pc] {
                Instr::LoadConst(val, dest, _) => { self.vars.insert(dest.clone(), *val); last_dest = Some(dest.clone()); pc += 1; }
                Instr::Move(src, dest, _) => {
                    let v = *self.vars.get(src).ok_or_else(|| format!("Undefined variable: {}", src))?;
                    self.vars.insert(dest.clone(), v);
                    last_dest = Some(dest.clone());
                    pc += 1;
                }
                Instr::BinOp(op, l, r, d) => {
                    let lv = *self.vars.get(l).ok_or_else(|| format!("Undefined variable: {}", l))?;
                    let rv = *self.vars.get(r).ok_or_else(|| format!("Undefined variable: {}", r))?;
                    let res = match op {
                        BinOp::Add(_) => lv + rv,
                        BinOp::Sub(_) => lv - rv,
                        BinOp::Mul(_) => lv * rv,
                        BinOp::Div(_) => { if rv == 0 { return Err("Division by zero".into()); } lv / rv }
                    };
                    self.vars.insert(d.clone(), res);
                    last_dest = Some(d.clone());
                    pc += 1;
                }
                Instr::Cmp(op, l, r, d) => {
                    let lv = *self.vars.get(l).ok_or_else(|| format!("Undefined variable: {}", l))?;
                    let rv = *self.vars.get(r).ok_or_else(|| format!("Undefined variable: {}", r))?;
                    let res = match op { CmpIR::Eq=> lv==rv, CmpIR::Ne=> lv!=rv, CmpIR::Lt=> lv<rv, CmpIR::Le=> lv<=rv, CmpIR::Gt=> lv>rv, CmpIR::Ge=> lv>=rv } as i64;
                    self.vars.insert(d.clone(), res);
                    last_dest = Some(d.clone());
                    pc += 1;
                }
                Instr::Label(_) => { pc += 1; }
                Instr::BrIf(c, t, e) => {
                    let cv = *self.vars.get(c).ok_or_else(|| format!("Undefined variable: {}", c))?;
                    let target = if cv != 0 { t } else { e };
                    pc = *labels.get(target).ok_or_else(|| format!("Unknown label: {}", target))?;
                }
                Instr::Jmp(l) => {
                    pc = *labels.get(l).ok_or_else(|| format!("Unknown label: {}", l))?;
                }
                Instr::Read(name, _) => {
                    // Prompt and read a single line from stdin for each read instruction
                    print!("Input {}: ", name);
                    io::stdout().flush().ok();
                    let mut line = String::new();
                    io::stdin().read_line(&mut line).map_err(|e| e.to_string())?;
                    let trimmed = line.trim();
                    let val: i64 = trimmed.parse().map_err(|_| format!("Failed to parse input '{}' as i64", trimmed))?;
                    self.vars.insert(name.clone(), val);
                    last_dest = Some(name.clone());
                    pc += 1;
                }
            }
        }
        if let Some(d) = last_dest { self.vars.get(&d).cloned().ok_or("No result".into()) } else { Err("No instructions".into()) }
    }
}
