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
}
