use core::fmt;
use std::fmt::Display;

use logos::Logos;

use crate::errors::LexicalError;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"//.*\n?")]
#[logos(skip r"[ \t\n\f]+")]
#[logos(error = LexicalError)]
pub enum Token {
    #[token("service")]
    KwService,
    #[token("operation")]
    KwOperation,
    #[token("import")]
    KwImport,
    #[token("from")]
    KwFrom,

    // types
    #[token("struct")]
    KwStruct,
    #[token("string")]
    KwString,
    #[token("timestamp")]
    KwTimestamp,
    #[token("boolean")]
    KwBoolean,
    #[token("byte")]
    KwByte,
    #[token("short")]
    KwShort,
    #[token("integer")]
    KwInt,
    #[token("long")]
    KwLong,
    #[token("float")]
    KwFloat,
    #[token("double")]
    KwDouble,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("@")]
    At,
    #[token("-")]
    Dash,
    #[token("$")]
    Dollar,

    #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().parse())]
    Identifier(String),

    #[regex(r#"\d+"#, |lex| lex.slice().parse())]
    Integer(i64),

    #[regex(r#"\d+\.\d+"#, |lex| lex.slice().parse())]
    Float(f64),

    #[regex(r#""[^"]*""#, |lex| lex.slice().parse())]
    String(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
