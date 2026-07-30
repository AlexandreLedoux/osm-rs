use clap::{Parser, Subcommand};
use map::{gui, object::storage::Storage, parse::parse, save};

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
    let cli = Cli::parse();

    match cli.command {
        Commands::Import => {
            println!("Import OSM...");

            let storage: Storage = parse::parse()?;

            for ((x, y), tile) in &storage.tiles {
                save::save_tile(*x, *y, tile)?;
            }

            println!("Import terminé");
        }

        Commands::Show => {
            println!("Lancement GUI");

            gui::run::run();
        }
    }

    Ok(())
}
