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
    #[token("resource")]
    KwResource,
    #[token("operation")]
    KwOperation,
    #[token("import")]
    KwImport,
    #[token("from")]
    KwFrom,
    #[token("extends")]
    KwExtends,
    #[token("for")]
    KwFor,
    #[token("create")]
    KwCreate,
    #[token("read")]
    KwRead,
    #[token("update")]
    KwUpdate,
    #[token("delete")]
    KwDelete,
    #[token("list")]
    KwList,
    #[token("put")]
    KwPut,
    #[token("struct")]
    KwStruct,

    #[token("string")]
    KwString,
    #[token("int")]
    KwInt,
    #[token("float")]
    KwFloat,

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
    #[token(">")]
    Arrow,
    #[token("-")]
    Dash,
    #[token("$")]
    Dollar,

    #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().parse())]
    Identifier(String),

    #[regex(r#"\d+"#, |lex| lex.slice().parse())]
    Integer(i64),

    #[regex(r#""[^"]*""#, |lex| lex.slice().parse())]
    String(String),

    Error,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
