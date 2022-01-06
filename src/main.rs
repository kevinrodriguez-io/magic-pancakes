pub mod commands;
pub mod state;
use commands as Commands;
use clap::{/*AppSettings, */ Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "pancakes")]
struct Cli {
    #[clap(subcommand)]
    command: ProgramCommand,
}

#[derive(Subcommand)]
enum ProgramCommand {
    #[clap()]
    Generate {
        #[clap(required = true)]
        amount: u32,
        #[clap(required = true)]
        json_template_path: String,
        #[clap(required = true)]
        layers_config_path: String,
        #[clap(required = true)]
        layers_path: String,
        #[clap(required = true)]
        output_path: String,
        #[clap(required = true)]
        output_format: String,
    },
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        ProgramCommand::Generate {
            amount,
            json_template_path,
            layers_config_path,
            layers_path,
            output_path,
            output_format
        } => {
            Commands::generate::exec(
                amount,
                json_template_path,
                layers_config_path,
                layers_path,
                output_path,
                output_format
            );
        }
    }
}
