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
    #[token(":")] Colon,
    #[token("i32")] I32,
    #[token("i64")] I64,

    // Skip whitespace automatically
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace, // This variant won't actually be produced due to logos::skip
}
