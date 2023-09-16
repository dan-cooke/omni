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
        body: Vec<Prop>,
    },
    Resource {
        id: Ident,
        body: Vec<Prop>,
        parents: Option<Vec<Ident>>,
    },
    Operation {
        id: Ident,
        body: Vec<Prop>,
    },
    Output {
        id: Ident,
        body: Vec<Prop>,
    },
    Create {
        id: Ident,
        body: Vec<Prop>,
    },
    Read {
        id: Ident,
        body: Vec<Prop>,
    },
    Update {
        id: Ident,
        body: Vec<Prop>,
    },
    Delete {
        id: Ident,
        body: Vec<Prop>,
    },
    List {
        id: Ident,
        body: Vec<Prop>,
    },
    Put {
        id: Ident,
        body: Vec<Prop>,
    },
    Struct {
        id: Ident,
        body: Vec<Prop>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Decorator {
    Id,
    Required,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Prop {
    pub key: String,
    pub value: Value,
    pub decorators: Option<Vec<Decorator>>,
}

pub type TypeDef = Vec<TypeProp>;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeProp {
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
    Map(Vec<Prop>),
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
