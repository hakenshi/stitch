use crate::utils::renderer::TemplateOptions;

pub trait CommandGenerator: TemplateOptions + Sized {
    fn execute(&self);
}
