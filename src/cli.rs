use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "stich")]
#[command(about = "Uma CLI para scaffolding de componentes rust", long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scaffolds a new react project
    Init {
        project_name: String,
        #[arg(long = "docker", short = 'd')]
        docker: bool,
    },
    /// Cria um novo componente
    #[command(name = "make:component")]
    MakeComponent {
        /// Nome do componente
        name: String,

        /// Incluir children
        #[arg(long = "with-children", short = 'c')]
        with_children: bool,

        /// Incluir props
        #[arg(long = "with-props", short = 'p')]
        with_props: bool,

        /// Incluir styles
        #[arg(long = "with-styles", short = 's')]
        with_styles: bool,
    },
    #[command(name = "make:route")]
    MakeRoute {
        name: String,
        #[arg(long = "layout", short = 'l')]
        layout: bool,
        #[arg(long = "async", conflicts_with = "api")]
        is_async: bool,

        #[arg(long = "api", short = 'a', conflicts_with_all=["layout", "is_async"])]
        api: bool,

        #[arg(long = "get", requires = "api")]
        get: bool,

        #[arg(long = "post", requires = "api")]
        post: bool,

        #[arg(long = "put", requires = "api")]
        put: bool,

        #[arg(long = "delete", requires = "api")]
        delete: bool,
    },
}
