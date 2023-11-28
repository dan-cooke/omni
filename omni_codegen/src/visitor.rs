use omni_parser::ast::*;

pub trait Visitor {
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
