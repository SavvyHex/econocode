use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap_or(0))]
    Int(i64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Ident(String),

    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Star,
    #[token("/")] Slash,

    #[token("(")] LParen,
    #[token(")")] RParen,
}
