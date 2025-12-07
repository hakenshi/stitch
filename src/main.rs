mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::{Commands, CLI};

use crate::commands::{generator::CommandGenerator, make_component::MakeComponentOptions};

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Commands::MakeComponent {
            name,
            with_children,
            with_props,
            with_styles,
        } => {
            MakeComponentOptions::execute(&MakeComponentOptions {
                name,
                with_children,
                with_props,
                with_styles,
            });
        }
    }
}
