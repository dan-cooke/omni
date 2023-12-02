use std::fs::{self, create_dir_all, write};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use omni_codegen::{visitor::Visitor, Hooks};
use omni_parser::ast::*;
use serde_json::json;

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

    pub fn create_module<P: AsRef<Path>>(&self, path: &P) {
        create_dir_all(path).unwrap();
    }

    pub fn create_package_json<P: AsRef<Path>>(
        &self,
        path: &P,
        handlebars_args: serde_json::Value,
    ) {
        let template = Template::new("ssdk/package.json.hbs");
        template.render_to_file(handlebars_args, &path).unwrap();
    }
    pub fn create_tsconfig<P: AsRef<Path>>(&self, path: &P) {
        let tsconfig_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("templates")
            .join("ssdk")
            .join("tsconfig.json");

        fs::copy(tsconfig_path, path).unwrap();
    }
}

impl Hooks for TypescriptSSDKGenerator {
    fn setup(&mut self) -> Result<(), String> {
        let module_path = PathBuf::from("./generated/node_modules/@omnidl/server-sdk/");
        self.create_module(&module_path);

        let tsconfig_path = module_path.join("tsconfig.json");
        self.create_tsconfig(&tsconfig_path);
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
    //
    //
    //
    //
    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::OperationDef {
                id,
                properties,
                span,
            } => {
                let input_class_name = properties
                    .iter()
                    .find(|property| property.id.name == "input")
                    .map(|property| {
                        // TODO: we don't want codegen authors to handle this here
                        // Input will be guaranteed to be an Option<Identifier>
                        // Otherwise there has been a compilation error and we won't
                        // reach this point
                        if let Expression::Identifier(identifier) = &property.value {
                            identifier.name.as_str()
                        } else {
                            "null"
                        }
                    })
                    .unwrap_or("null");

                let output_class_name = properties
                    .iter()
                    .find(|property| property.id.name == "output")
                    .map(|property| {
                        // TODO: we don't want codegen authors to handle this here
                        // Output will be guaranteed to be an Option<Identifier>
                        // Otherwise there has been a compilation error and we won't
                        // reach this point
                        if let Expression::Identifier(identifier) = &property.value {
                            identifier.name.as_str()
                        } else {
                            "null"
                        }
                    })
                    .unwrap_or("null");

                let operation = Template::new("ssdk/operation.hbs");

                let operation = operation
                    .render_to_string(json!({
                        "operation_id": id.name,
                        "input_class_name": input_class_name,
                        "output_class_name": output_class_name,
                    }))
                    .unwrap();

                self.output.push_str(&operation);
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
                self.output.push_str("}; \n\n");
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
