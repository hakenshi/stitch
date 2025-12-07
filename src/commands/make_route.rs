use std::fs;

use tera::Context;

use crate::{
    commands::generator::CommandGenerator,
    utils::{renderer::TemplateOptions, to_pascal_case, TemplateRenderer},
};

pub struct MakeRouteOptions {
    pub name: String,
    pub api: bool,
    pub layout: bool,
    pub is_async: bool,
    pub get: bool,
    pub post: bool,
    pub put: bool,
    pub delete: bool,
}

impl MakeRouteOptions {
    fn route_name(&self) -> String {
        let last_segment = self.name.split('/').last().unwrap_or(&self.name);

        return to_pascal_case(last_segment);
    }
}

impl TemplateOptions for MakeRouteOptions {
    fn build_context(&self) -> tera::Context {
        let mut context = Context::new();
        context.insert("name", &self.route_name());
        context.insert("is_async", &self.is_async);

        if self.api {
            context.insert("api", &self.api);
            context.insert("get", &self.get);
            context.insert("post", &self.post);
            context.insert("put", &self.put);
            context.insert("delete", &self.delete);
        }

        return context;
    }
    fn result_dir(&self) -> String {
        if self.api {
            return "./src/app/api/".to_string();
        }

        return format!("./src/app/{}", self.name);
    }
}

impl CommandGenerator for MakeRouteOptions {
    fn execute(&self) {
        let template_renderer = match TemplateRenderer::new(self) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error loading templates: {}", e);
                return;
            }
        };

        if let Err(e) = fs::create_dir_all(&template_renderer.result_dir) {
            eprintln!("Error creating directory: {}", e);
            return;
        }

        if self.api {
            template_renderer.create("route.ts.tera", "route.ts");
            return;
        }

        template_renderer.create("page.tsx.tera", "page.tsx");

        if self.layout {
            template_renderer.create("layout.tsx.tera", "layout.tsx");
        }
    }
}
