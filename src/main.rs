use clap::{Parser, Subcommand};
use map::{common::index_step::IndexStep, gui::gui, index::index::index, parse};

#[derive(Parser)]
#[command(name = "map")]
#[command(about = "Rust map renderer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Import,
    Show,
    Index {
        filename: String,

        #[arg(short, long)]
        step: IndexStep,

        #[clap(short, long, default_value = None)]
        zoom: Option<u8>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Index { filename, step, zoom } => {
            println!("Indexing: {}", filename);

            index(filename, step, zoom)?;

            println!("Indexing finished");
        }

        Commands::Import => {
            println!("Import OSM...");

            parse::parse()?;

            println!("Import terminé");
        }

        Commands::Show => {
            gui::main();
        }
    }

    Ok(())
}
