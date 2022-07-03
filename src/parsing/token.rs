use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, Hash, PartialEq, Eq)]
pub enum Token {
    #[error]
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"[ \t\r\n]", logos::skip)]
    Error,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[token("=")]
    Eq,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(";")]
    SemiColon,

    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("pub")]
    Pub,

    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*", cb_ident)]
    Ident(String),
}

fn cb_ident(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_owned()
}
