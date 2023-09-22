pub mod python;
#[derive(Debug, PartialEq, Clone)]
pub struct File {
    pub imports: Option<Vec<Import>>,
    pub body: Vec<Def>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Import {
    Local { id: Ident, path: String },
    Remote { id: Ident, path: String },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Def {
    Service {
        id: Ident,
        resources: Vec<Ident>,
    },
    Resource {
        id: Ident,
        body: Vec<TypeDef>,
        parents: Option<Vec<Ident>>,
    },
    Operation {
        id: Ident,
        body: Vec<ValueDef>,
    },
    Output {
        id: Ident,
        body: Vec<ValueDef>,
    },
    Create {
        id: Ident,
        body: Vec<ValueDef>,
    },
    Read {
        id: Ident,
        body: Vec<ValueDef>,
    },
    Update {
        id: Ident,
        body: Vec<ValueDef>,
    },
    Delete {
        id: Ident,
        body: Vec<ValueDef>,
    },
    List {
        id: Ident,
        body: Vec<ValueDef>,
    },
    Put {
        id: Ident,
        body: Vec<ValueDef>,
    },
    Struct {
        id: Ident,
        body: Vec<ValueDef>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Decorator {
    Id,
    Required,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ValueDef {
    pub key: String,
    pub value: Value,
    pub decorators: Option<Vec<Decorator>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDef {
    pub key: String,
    pub value: Type,
    pub decorators: Option<Vec<Decorator>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Identifier(Ident),
    String(String),
    Integer(Int),
    Float(Float),
    Array(Vec<Value>),
    Map(Vec<ValueDef>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    String,
    Int,
    Float,
    Array(Box<Type>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident {
    pub id: String,
}

pub type Int = i64;
pub type Float = u64;

