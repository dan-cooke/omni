use omni_parser::parse_file;

use crate::{visitor::Visitor, Hooks};

pub struct CodeGenerator<Generator: Visitor + Hooks> {
    generator: Generator,
}

impl<Generator: Visitor + Hooks> CodeGenerator<Generator> {
    pub fn new(generator: Generator) -> Self {
        Self { generator }
    }
    pub fn run(&mut self) -> Result<(), String> {
        // Hardcode main.omni for now
        let file = parse_file("main.omni");

        self.generator.setup().unwrap();

        self.generator.visit_file(&file);
        Ok(())
    }
}
