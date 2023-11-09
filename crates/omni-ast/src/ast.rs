pub enum File {
    Body(Vec<Statement>),
}

pub enum Statement {
    Import(Import),
    ResourceDef(ResourceDef),
    ServiceDef(ServiceDef),
    ReadDef(ReadDef),
    WriteDef(WriteDef),
    UpdateDef(UpdateDef),
    DeleteDef(DeleteDef),
}

pub struct Import {
    path: String,
}
