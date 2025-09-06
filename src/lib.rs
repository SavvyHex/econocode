pub mod lower;
#[macro_use]
extern crate lalrpop_util;

pub mod ast;
pub mod lexer;
pub mod ir;
pub mod interpreter;

lalrpop_mod!(parser);

pub use parser::*;
