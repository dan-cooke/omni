use std::{
    fmt::format,
    fs::{self, create_dir_all, read_to_string},
    io::Write,
    path::{self, Path},
};

use handlebars::Handlebars;
use omni_ast::*;
use omni_codegen::{visitor::Visitor, Hooks};
use serde_json::json;

pub struct TypescriptSSDKGenerator {
    output: String,
}

impl TypescriptSSDKGenerator {
    pub fn new() -> Self {
        Self {
            output: format!(""),
        }
    }

    pub fn create_module(&self) {
        let module_path = "./generated/node_modules/@omnidl/server-sdk/";
        let package_json_template_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("./src/templates/server.package.json.hbs");

        create_dir_all(module_path).unwrap();
        let reg = Handlebars::new();
        let template = read_to_string(package_json_template_path).unwrap();

        let package_json = reg
            .render_template(
                template.as_str(),
                &json!(
                    {

                        "name": "simple" ,// TODO: hardcode for now
                        "version": "0.0.1",
                        "tsVersion": "5.4.3"
                    }

                ),
            )
            .expect("Error rendering server package json template");

        let mut file = fs::File::create(format!("{}/package.json", module_path)).unwrap();

        file.write_all(&package_json.into_bytes()).unwrap();
    }
}

impl Hooks for TypescriptSSDKGenerator {
    fn setup(&mut self) -> Result<(), String> {
        self.create_module();
        todo!()
    }
}

impl Visitor for TypescriptSSDKGenerator {
    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::ServiceDef { id, properties } => todo!(),
            Statement::OperationDef { id, properties } => todo!(),
            Statement::StructDef { id, properties } => todo!(),
            Statement::SimpleTypeDef { id, _type } => todo!(),
        }
    }

    fn visit_property(&mut self, property: &Property) {
        todo!()
    }

    fn visit_expression(&mut self, expression: &Expression) {
        todo!()
    }

    fn visit_identifier(&mut self, identifier: &Identifier) {
        todo!()
    }

    fn visit_literal(&mut self, literal: &Literal) {
        todo!()
    }

    fn visit_type(&mut self, _type: &Type) {
        todo!()
    }
}
