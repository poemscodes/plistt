use clap::{Parser, Subcommand};
use plist_modem::json;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help(true))]
    Json {
        // #[arg(default_value_t = Some("-".to_string()))]
        source: Option<String>,
        // #[arg(default_value_t = Some("-".to_string()))]
        destination: Option<String>,
    },
}
const DEFAULT_SOURCE_FILENAME: &'static str = "/dev/stdin";
const DEFAULT_DESTINATION_FILENAME: &'static str = "/dev/stdout";

fn default_source_filename() -> String {
    String::from(DEFAULT_SOURCE_FILENAME)
}
fn default_destination_filename() -> String {
    String::from(DEFAULT_DESTINATION_FILENAME)
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Json {
            source,
            destination,
        } => {
            json::transcode_xml_to_json_file(
                match source {
                    Some(source) => match source.trim() {
                        "" => default_source_filename(),
                        "-" => default_source_filename(),
                        _ => source.to_string(),
                    },
                    None => default_source_filename(),
                },
                match destination {
                    Some(destination) => match destination.trim() {
                        "" => default_destination_filename(),
                        "-" => default_destination_filename(),
                        _ => destination.to_string(),
                    },
                    None => default_destination_filename(),
                },
            );
        }
    }
}
