use std::{
    fs::{self, read_to_string},
    io::Write,
    path::{Path, PathBuf},
};

use handlebars::{Handlebars, RenderError};

pub struct Template {
    path: PathBuf,
}

impl Template {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/templates");
        path_buf.push(path);
        Template { path: path_buf }
    }

    pub fn render_to_string(&self, args: serde_json::Value) -> Result<String, RenderError> {
        let reg = Handlebars::new();
        let template = read_to_string(&self.path).unwrap();

        let content = reg.render_template(template.as_str(), &args)?;

        Ok(content)
    }

    pub fn render_to_file<P: AsRef<Path>>(
        &self,
        args: serde_json::Value,
        out_path: P,
    ) -> Result<(), RenderError> {
        let content = self.render_to_string(args)?;
        let mut file = fs::File::create(&out_path).unwrap();

        file.write_all(&content.into_bytes()).unwrap();
        Ok(())
    }
}
