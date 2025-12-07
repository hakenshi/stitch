mod cli;
mod commands;

use clap::Parser;
use cli::{Commands, CLI};

use crate::commands::{MakeComponentOptions, make_component};

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Commands::MakeComponent {
            name,
            with_children,
            with_props,
            with_styles,
        } => {
            make_component(MakeComponentOptions {
                name,
                with_children,
                with_props,
                with_styles,
            });
        }
    }
}
