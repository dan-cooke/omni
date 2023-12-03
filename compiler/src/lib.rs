use std::path::Path;

mod context;
use context::SymbolMap;
use omni_parser::parse_file;

pub fn compile_project<P: AsRef<Path>>(entry: P) {
    // this will just be an entry file for now
    // but in future this will take a Manifest

    let file = parse_file(entry);
    let symbols = SymbolMap::new();
}
#[cfg(test)]
mod tests {}
