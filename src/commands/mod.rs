mod make_component;

pub struct MakeComponentOptions {
    pub name: String,
    pub with_children: bool,
    pub with_props: bool,
    pub with_styles: bool,
}

pub use make_component::execute as make_component;
