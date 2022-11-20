use crate::graphql::GatewaySchema;
use clap::{Parser, Subcommand};
use std::io::Write;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    name: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    SDL,
}

pub fn listener(schema: &GatewaySchema) {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::SDL) => {
            let sdl = &schema.sdl();
            let mut schema_file = std::fs::File::create("graphql/schema.gql").unwrap();
            schema_file.write(sdl.as_bytes()).unwrap();
        }
        None => {}
    }
}
