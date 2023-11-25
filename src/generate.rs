use clap::Command;
use omni_ast::*;
use omni_parser::parse_file;

pub struct Generate {}

impl Generate {
    pub fn run(&self) -> Result<(), String> {
        // Hardcode main.omni for now
        let file = parse_file("main.omni");

        print!("{:?}", file);
        Ok(())
    }
    pub fn command() -> Command {
        Command::new("generate").visible_alias("g")
    }
}

trait Visitor {
    fn visit_file(&mut self, file: &File) {
        for statement in &file.body {
            self.visit_statement(statement);
        }
    }
    fn visit_statement(&mut self, statement: &Statement);
    fn visit_property(&mut self, property: &Property);
    fn visit_expression(&mut self, expression: &Expression);
    fn visit_identifier(&mut self, identifier: &Identifier);
    fn visit_literal(&mut self, literal: &Literal);
    fn visit_type(&mut self, _type: &Type);
}
