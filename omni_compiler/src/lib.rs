use std::path::Path;

use omni_parser::parse_file;

pub fn compile_project<P: AsRef<Path>>(entry: P) {
    // this will just be an entry file for now
    // but in future this will take a Manifest

    parse_file(entry);
}
#[cfg(test)]
mod tests {}
