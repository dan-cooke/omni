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
    Output(ResourceOperation),
    Input(ResourceOperation),
    Create(ResourceOperation),
    Read(ResourceOperation),
    Update(ResourceOperation),
    Delete(ResourceOperation),
    List(ResourceOperation),
    Put(ResourceOperation),
    Struct {
        id: Ident,
        body: Vec<Prop>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResourceOperation {
    pub id: Ident,
    pub resource: Ident,
    pub body: Vec<Prop>,
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
