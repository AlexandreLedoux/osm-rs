use clap::{Parser, Subcommand};
use map::{gui::gui, import::parse::parse};

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Import => {
            println!("Import OSM...");

            parse()?;

            println!("Import terminé");
        }

        Commands::Show => {
            println!("Chargement du Storage...");

            gui::main()?;
        }
    }

    Ok(())
}
