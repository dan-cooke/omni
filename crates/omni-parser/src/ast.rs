#[derive(Debug, PartialEq)]
pub struct File {
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Property {
    pub id: Identifier,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    ObjectExpression { properties: Vec<Property> },
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(i64),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}
