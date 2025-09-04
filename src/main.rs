use econocode::{lexer::Token, ExprParser};
use crate::lower::Lower;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use logos::Logos;

mod lower;

/// Toy language compiler (AST -> IR)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input source file
    input: PathBuf,

    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Dump AST before IR
    #[arg(long)]
    ast: bool,
}

fn main() {
    let args = Args::parse();

    let src = fs::read_to_string(&args.input)
        .expect("Failed to read input file");

    // Tokenize the input
    let lexer = Token::lexer(&src);
    let tokens: Vec<_> = lexer.enumerate()
        .filter_map(|(pos, token)| match token {
            Ok(token) => Some(Ok((pos, token, pos + 1))), // (start, token, end)
            Err(_) => None, // Skip errors
        }).collect();

    let parser = ExprParser::new();
    match parser.parse(tokens) {
        Ok(ast) => {
            if args.ast {
                println!("AST: {:#?}", ast);
            }

            let mut lower = Lower::new();
            let result = lower.lower_expr(&ast);

            let mut out = String::new();
            for instr in &lower.code {
                out.push_str(&format!("{}\n", instr));
            }
            out.push_str(&format!("Result in {}\n", result));

            // write IR to file or stdout
            if let Some(path) = args.output {
                fs::write(path, out).expect("Failed to write output file");
            } else {
                let mut stdout = io::stdout();
                stdout.write_all(out.as_bytes()).unwrap();
            }
        }
        Err(e) => eprintln!("Parse error: {:?}", e),
    }
}
