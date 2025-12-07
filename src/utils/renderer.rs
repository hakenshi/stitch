use std::fs;

use tera::{Context, Tera};

pub trait TemplateOptions {
    fn result_dir(&self) -> String;
    fn build_context(&self) -> Context;
}

pub struct TemplateRenderer {
    tera: Tera,
    context: Context,
    pub result_dir: String,
}

impl TemplateRenderer {
    pub fn new<T: TemplateOptions>(options: &T) -> Result<Self, tera::Error> {
        let tera = Tera::new("./src/templates/**/*.tera")?;

        return Ok(Self {
            tera,
            context: options.build_context(),
            result_dir: options.result_dir(),
        });
    }

    pub fn create(&self, template_name: &str, file_name: &str) {
        match self.tera.render(template_name, &self.context) {
            Ok(content) => {
                let file_path = format!("{}/{}", self.result_dir, file_name);

                if let Err(e) = fs::write(&file_path, content) {
                    eprintln!("Error writing file: {}\nError: {}", file_path, e);
                } else {
                    println!("✓ Created: {}", file_path);
                }
            }
            Err(e) => {
                eprintln!("Error rendering template: {}", e);
            }
        }
    }
}
