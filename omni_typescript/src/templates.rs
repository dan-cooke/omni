use std::{
    fs::{self, read_to_string},
    io::Write,
    path::PathBuf,
};

use handlebars::{Handlebars, RenderError};

pub struct Template {
    path: PathBuf,
}

impl Template {
    pub fn new(path: &str) -> Self {
        let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_buf.push(path);
        Template { path: path_buf }
    }

    pub fn render_to_file(
        &self,
        args: serde_json::Value,
        out_path: &str,
    ) -> Result<(), RenderError> {
        let reg = Handlebars::new();
        println!("{}", &self.path.to_str().unwrap());
        let template = read_to_string(&self.path).unwrap();

        let content = reg.render_template(template.as_str(), &args)?;

        let mut file = fs::File::create(&out_path).unwrap();

        file.write_all(&content.into_bytes()).unwrap();
        Ok(())
    }
}
