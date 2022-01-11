pub mod commands;
pub mod state;
pub mod tools;
use clap::{Parser, Subcommand};
use commands as Commands;

#[derive(Parser)]
#[clap(name = "pancakes", about, version, author)]
struct Cli {
    #[clap(subcommand)]
    command: ProgramCommand,
}

#[derive(Subcommand)]
enum ProgramCommand {
    #[clap()]
    Generate {
        #[clap(required = true, short, long)]
        amount: u32,
        #[clap(required = true, short, long)]
        json_template_path: String,
        #[clap(required = true, short, long)]
        config_path: String,
        #[clap(required = true, short, long)]
        layers_path: String,
        #[clap(required = true, short, long)]
        output_path: String,
        #[clap(required = true, short, long)]
        format: String,
        #[clap(required = false, short, long)]
        threads: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        ProgramCommand::Generate {
            amount,
            json_template_path,
            config_path,
            layers_path,
            output_path,
            format,
            threads,
        } => {
            Commands::generate::exec(
                amount,
                json_template_path,
                config_path,
                layers_path,
                output_path,
                format,
                threads,
            );
        }
    }
}
