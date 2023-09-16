#[derive(Debug)]
pub enum Mod {
    Module { body: Vec<Def> },
}

#[derive(Debug)]
pub enum Def {
    ServiceDef { id: Ident, body: Vec<Prop> },
    ResourceDef { id: Ident, body: Vec<Prop> },
    OperationDef { id: Ident, body: Vec<Prop> },
}
#[derive(Debug)]
pub struct Prop {
    pub id: Ident,
    pub value: Value,
}

#[derive(Debug)]
pub enum Value {
    Identifier(Ident),
    String(String),
    Integer(Int),
    Float(Float),
    Array(Vec<Value>),
    Object(Vec<Prop>),
}

pub type Ident = String;
pub type Int = i64;
pub type Float = u64;
