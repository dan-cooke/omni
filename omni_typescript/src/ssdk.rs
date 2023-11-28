use std::{
    fmt::format,
    fs::{self, create_dir_all, read_to_string},
    io::Write,
    path::{self, Path},
};

use handlebars::Handlebars;
use omni_codegen::{visitor::Visitor, Hooks};
use omni_parser::ast::*;
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

    pub fn create_module(&self, path: &str) {
        create_dir_all(path).unwrap();
    }

    pub fn create_package_json(&self, path: &str, handlebars_args: serde_json::Value) {
        let reg = Handlebars::new();
        let package_json_template_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("./src/templates/server.package.json.hbs");
        let template = read_to_string(package_json_template_path).unwrap();

        let package_json = reg
            .render_template(template.as_str(), &handlebars_args)
            .expect("Error rendering server package json template");

        let mut file = fs::File::create(format!("{}/package.json", path)).unwrap();

        file.write_all(&package_json.into_bytes()).unwrap();
    }
}

impl Hooks for TypescriptSSDKGenerator {
    fn setup(&mut self) -> Result<(), String> {
        let module_path = "./generated/node_modules/@omnidl/server-sdk/";
        self.create_module(module_path);
        Ok(())
    }
}

impl Visitor for TypescriptSSDKGenerator {
    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::ServiceDef {
                id,
                properties,
                span,
            } => {
                let package_details = json!(
                    {

                        "name": "simple" ,// TODO: hardcode for now
                        "version": "1.0.0",
                        "tsVersion": "5.4.3"
                    }

                );
                self.create_package_json(
                    "./generated/node_modules/@omnidl/server-sdk/",
                    package_details,
                );
            }
            Statement::OperationDef {
                id,
                properties,
                span,
            } => todo!(),
            Statement::StructDef {
                id,
                properties,
                span,
            } => todo!(),
            Statement::SimpleTypeDef { id, _type, span } => todo!(),
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
