mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::{Commands, CLI};

use crate::commands::{
    generator::CommandGenerator, make_component::MakeComponentOptions, make_route::MakeRouteOptions,
};

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
        Commands::MakeRoute {
            name,
            layout,
            is_async,
            api,
            get,
            post,
            put,
            delete,
        } => {
            MakeRouteOptions::execute(&MakeRouteOptions {
                name,
                api,
                layout,
                is_async,
                get,
                post,
                put,
                delete,
            });
        }
        #[allow(unused_variables)]
        Commands::Init { project_name, docker } => {}
    }
}
