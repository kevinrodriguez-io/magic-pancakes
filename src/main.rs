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
        #[clap(
            required = true,
            short = 'a',
            long = "amount",
            default_value = "5000",
            help = "The amount of pancakes(Generative Images) 🥞 to generate"
        )]
        amount: u32,
        #[clap(
            required = true,
            short = 'j',
            long = "json-template-path",
            default_value = "./template.json",
            help = "Path to the json template file"
        )]
        json_template_path: String,
        #[clap(
            required = true,
            short = 'c',
            long = "layers-config",
            default_value = "./config.json",
            help = "Path to the layers configuration file"
        )]
        config_path: String,
        #[clap(
            required = true,
            short = 'l',
            long = "layers-path",
            default_value = "./layers",
            help = "Path to the directory containing the layers; It must be a tree directory that matches the layers configuration file."
        )]
        layers_path: String,
        #[clap(
            required = true,
            short = 'p',
            long = "output-path",
            default_value = "./output",
            help = "Output directory in which the files will be generated."
        )]
        output_path: String,
        #[clap(
            required = true,
            short = 'f',
            long = "output-format",
            default_value = "jpeg",
            help = "Output format of the generated images. Right now jpeg, png and webp are supported."
        )]
        format: String,
        #[clap(
            required = false,
            short = 't',
            long = "threads",
            help = "[OPTIONAL] Number of threads to use for the generation. Defaults to the number of cores."
        )]
        threads: Option<String>,
        #[clap(
            required = false,
            long = "jpeg-quality",
            help = "[OPTIONAL] Only to be used with jpeg. Defaults to 90."
        )]
        jpeg_quality: Option<u8>,
        #[clap(
            required = false,
            long = "png-compression-type",
            help = "[OPTIONAL] Only to be used with png. Defaults to 'default'. Supported options are 'best', 'default', 'fast', 'huffman' and 'rle'."
        )]
        png_compression_type: Option<String>,
        #[clap(
            required = false,
            long = "png-filter-type",
            help = "[OPTIONAL] Only to be used with png. Defaults to 'none'. Supported options are 'avg', 'nofilter', 'paeth', 'sub' and 'up'."
        )]
        png_filter_type: Option<String>,
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
            jpeg_quality,
            png_compression_type,
            png_filter_type,
        } => {
            Commands::generate::exec(
                amount.to_owned(),
                json_template_path.to_owned(),
                config_path.to_owned(),
                layers_path.to_owned(),
                output_path.to_owned(),
                format.to_owned(),
                threads.to_owned(),
                jpeg_quality.to_owned(),
                png_compression_type.to_owned(),
                png_filter_type.to_owned(),
            );
        }
    }
}
