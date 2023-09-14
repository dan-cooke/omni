pub enum Mod {
    Module { body: Vec<Stmt> },
}

pub enum Stmt {
    ServiceDef {
        name: String,
        version: String,
        resources: Vec<String>,
    },
}

pub type Ident = String;
pub type Int = usize;
pub type Float = isize;
