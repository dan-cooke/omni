use std::fs::{create_dir_all, write};
use std::io::prelude::*;

use omni_codegen::{visitor::Visitor, Hooks};
use omni_parser::ast::*;

use crate::templates::Template;

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
        let template = Template::new("src/templates/server.package.json.hbs");
        template.render_to_file(handlebars_args, &path).unwrap();
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
    fn visit_file(&mut self, file: &File) {
        for statement in &file.body {
            self.visit_statement(statement);
        }

        let mut index_file =
            std::fs::File::create("./generated/node_modules/@omnidl/server-sdk/index.ts").unwrap();

        index_file.write_all(self.output.as_bytes()).unwrap();
    }
    /// export function getGetRandomJokeHandler<Context>(operation: Operation<GetRandomJokeInput, GetRandomJokeOutput, Context>): Handler<HttpRequest, HttpResponse, Context> {
    // 	return {
    // 		handle: (request, context) => {
    // 			return serializeHttpJsonResponse(operation(undefined, context));
    // 		}
    // 	}
    // }
    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::OperationDef {
                id,
                properties,
                span,
            } => {
                self.output.push_str("export function ");
                self.output.push_str("get");
                self.visit_identifier(id);
            }

            Statement::StructDef {
                id,
                properties,
                span,
            } => {
                self.output.push_str("export interface ");
                self.visit_identifier(id);
                self.output.push_str(" {");
                properties.iter().for_each(|property| {
                    self.visit_property(property);
                });
                self.output.push_str("}");
            }
            Statement::SimpleTypeDef { id, _type, span } => todo!(),
        }
    }

    fn visit_property(&mut self, property: &Property) {
        self.visit_identifier(&property.id);
        self.output.push_str(": ");
        self.visit_expression(&property.value);
        self.output.push_str(",");
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match &expression {
            Expression::Literal(val) => {
                self.visit_literal(val);
            }
            Expression::Identifier(val) => {
                self.visit_identifier(val);
            }
            Expression::ArrayExpression(val) => {
                self.output.push_str("[");
                val.elements.iter().for_each(|element| {
                    self.visit_identifier(element);
                });
                self.output.push_str("]");
            }
            Expression::ObjectExpression(val) => {
                self.output.push_str("{");
                val.properties.iter().for_each(|property| {
                    self.visit_property(property);
                });
                self.output.push_str("}");
            }
        }
    }

    fn visit_identifier(&mut self, identifier: &Identifier) {
        self.output.push_str(&format!("{}", identifier.name));
    }

    fn visit_literal(&mut self, literal: &Literal) {
        match &literal.value {
            LiteralType::String(val) => {
                self.output.push_str(&format!("\"{}\"", val));
            }
            LiteralType::Integer(val) => {
                self.output.push_str(&format!("{}", val));
            }
            LiteralType::Float(val) => {
                self.output.push_str(&format!("{}", val));
            }
            LiteralType::Boolean(val) => {
                self.output.push_str(&format!("{}", val));
            }
            LiteralType::Null => {
                self.output.push_str("null");
            }
        }
    }

    fn visit_type(&mut self, _type: &Type) {
        todo!()
    }
}
