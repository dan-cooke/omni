use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    StructDef {
        id: Identifier,
        properties: Vec<Property>,
    },
    SimpleTypeDef {
        id: Identifier,
        _type: Type,
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
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Identifier { name: String },
    ObjectExpression { properties: Vec<Property> },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
}
