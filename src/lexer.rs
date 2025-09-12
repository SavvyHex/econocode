use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap_or(0))]
    Int(i64),

    // Keywords and types (must come before Ident)
    #[token("if")] If,
    #[token("else")] Else,
    #[token("while")] While,
    #[token("i32")] I32,
    #[token("i64")] I64,

    // Operators (longest first)
    #[token("==")] EqEq,
    #[token("!=")] BangEq,
    #[token("<=")] LtEq,
    #[token(">=")] GtEq,
    #[token("=")] Eq,
    #[token("<")] Lt,
    #[token(">")] Gt,
    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Star,
    #[token("/")] Slash,

    // Delimiters
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token(";")] Semicolon,
    #[token(":")] Colon,

    // Identifiers (after keywords)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Ident(String),

    // Skip whitespace automatically
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace, // This variant won't actually be produced due to logos::skip
}
