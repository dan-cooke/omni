use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    ServiceDef {
        id: Identifier,
        properties: Vec<Property>,
    },
    OperationDef {
        id: Identifier,
        properties: Vec<Property>,
    },
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
    // Currently only support list of identifiers
    // Not sure if we need list of literals yet
    // We don't want to allow [{ someProperty }] as its too dynamic
    // We require all array elements to be named types eg [SomeStruct]
    // So its not Vec<Expression>
    ArrayExpression { elements: Vec<Identifier> },
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
