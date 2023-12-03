use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Span {
    // name of the file
    file_name: PathBuf,
    //  start offset in bytes from beginning of file
    start: usize,
    //  end offset in bytes from beginning of file
    end: usize,
}

impl Span {
    pub fn new<P: AsRef<Path>>(file_name: P, start: usize, end: usize) -> Self {
        Self {
            file_name: file_name.as_ref().into(),
            start,
            end,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    OperationDef {
        id: Identifier,
        properties: Vec<Property>,
        span: Span,
    },
    StructDef {
        id: Identifier,
        properties: Vec<Property>,
        span: Span,
    },
    SimpleTypeDef {
        id: Identifier,
        _type: Type,
        span: Span,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    String,
    Timestamp,
    Boolean,
    Byte,
    Short,
    Integer,
    Long,
    Float,
    Double,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub id: Identifier,
    pub value: Expression,
    pub span: Span,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    // Currently only support list of identifiers
    // Not sure if we need list of literals yet
    // We don't want to allow [{ someProperty }] as its too dynamic
    // We require all array elements to be named types eg [SomeStruct]
    // So its not Vec<Expression>
    ArrayExpression(ArrayExpression),
    ObjectExpression(ObjectExpression),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LiteralType {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ArrayExpression {
    pub elements: Vec<Identifier>,
    pub span: Span,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectExpression {
    pub properties: Vec<Property>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Literal {
    pub value: LiteralType,
    pub span: Span,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}
