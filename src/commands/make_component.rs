use std::fs;

use tera::{Context, Tera};

use crate::commands::MakeComponentOptions;

fn to_pascal_case(s: &str) -> String {
    s.split(|c| c == '-' || c == '_' || c == ' ')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}

struct TemplateRenderer {
    tera: Tera,
    context: Context,
    component_dir: String,
}

impl TemplateRenderer {
    fn new(options: &MakeComponentOptions) -> Result<Self, tera::Error> {
        let tera = Tera::new("./src/templates/**/*.tera")?;
        let mut context = Context::new();
        let pascal_name = to_pascal_case(&options.name);
        context.insert("name", &pascal_name);
        context.insert("with_children", &options.with_children);
        context.insert("with_props", &options.with_props);
        context.insert("with_styles", &options.with_styles);

        Ok(Self {
            tera,
            context,
            component_dir: format!("./components/{}", options.name),
        })
    }

    fn create(&self, template_name: &str, file_name: &str) {
        match self.tera.render(template_name, &self.context) {
            Ok(content) => {
                let file_path = format!("{}/{}", self.component_dir, file_name);

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

pub fn execute(options: MakeComponentOptions) {
    let template_renderer = match TemplateRenderer::new(&options) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error loading templates: {}", e);
            return;
        }
    };

    if let Err(e) = fs::create_dir_all(&template_renderer.component_dir) {
        eprintln!("Error creating directory: {}", e);
        return;
    }

    template_renderer.create("component.tsx.tera", "index.tsx");

    if options.with_styles {
        template_renderer.create("styles.module.css.tera", "styles.module.css");
    }
}
