use std::collections::HashMap;

use omni_parser::ast::{self};

pub struct SymbolTable<'a> {
    pub symbols: HashMap<&'a str, &'a ast::Statement>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn scan_file(&mut self, file: &ast::File) -> Self {
        let mut symbols: HashMap<&'_ str, &'_ ast::Statement> = HashMap::new();
        for statement in &file.body {
            match statement {
                ast::Statement::OperationDef {
                    id,
                    properties,
                    span,
                }
                | ast::Statement::StructDef {
                    id,
                    properties,
                    span,
                } => {
                    symbols.insert(
                        &id.name,
                        &ast::Statement::StructDef {
                            id: *id,
                            properties: *properties,
                            span: *span,
                        },
                    );
                }
                ast::Statement::SimpleTypeDef { id, _type, span } => {
                    symbols.insert(
                        &id.name,
                        &ast::Statement::SimpleTypeDef {
                            id: *id,
                            _type: *_type,
                            span: *span,
                        },
                    );
                }
            }
        }
        Self { symbols }
    }

    pub fn insert(&mut self, id: &str, statement: &ast::Statement) {
        self.symbols.insert(id, statement);
    }

    pub fn resolve(&self, id: &str) -> Option<&ast::Statement> {
        self.symbols.get(id)
    }
}
