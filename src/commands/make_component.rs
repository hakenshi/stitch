use std::fs;

use tera::Context;

use crate::commands::generator::CommandGenerator;
use crate::utils::renderer::TemplateOptions;
use crate::utils::{to_pascal_case, TemplateRenderer};
pub struct MakeComponentOptions {
    pub name: String,
    pub with_children: bool,
    pub with_props: bool,
    pub with_styles: bool,
}

impl TemplateOptions for MakeComponentOptions {
    fn build_context(&self) -> tera::Context {
        let mut context = Context::new();
        let pascal_name = to_pascal_case(&self.name);
        context.insert("name", &pascal_name);
        context.insert("with_children", &self.with_children);
        context.insert("with_props", &self.with_props);
        context.insert("with_styles", &self.with_styles);
        return context;
    }

    fn result_dir(&self) -> String {
        format!("./src/components/{}", self.name)
    }
}

impl CommandGenerator for MakeComponentOptions {
    fn execute(&self) {
        let template_renderer = match TemplateRenderer::new(self) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error loading templates: {}", e);
                return;
            }
        };

        if let Err(e) = fs::create_dir_all(&template_renderer.result_dir) {
            eprintln!("Error creating directory: {}", e);
            return;
        }

        template_renderer.create("components.tsx.tera", "index.tsx");

        if self.with_styles {
            template_renderer.create("styles.module.css.tera", "styles.module.css");
        }
    }
}
