#[macro_use]
extern crate lalrpop_util;

pub mod ast;
pub mod lexer;
pub mod ir;

lalrpop_mod!(parser);

// Re-export parser for main.rs
pub use parser::*;
