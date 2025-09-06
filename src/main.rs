use econocode::{lexer::Token, ExprParser};
use econocode::lower::{Lower, estimate_energy};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use logos::Logos;



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

    /// Execute the code and print result
    #[arg(long)]
    run: bool,
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
                                let energy = estimate_energy(&lower.code);
                for (i, instr) in lower.code.iter().enumerate() {
                    println!("t{} = {}", i, instr);
                }
                println!("Result in {}", result);
                println!("Estimated total energy: {} units", energy);

                if args.run {
                    let mut interpreter = econocode::interpreter::Interpreter::new();
                    match interpreter.execute(&lower.code) {
                        Ok(final_result) => println!("Execution result: {}", final_result),
                        Err(e) => eprintln!("Execution error: {}", e),
                    }
                }

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
