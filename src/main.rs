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

#[macroquad::main("OSM Renderer")]
async fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Import => {
            println!("Import OSM...");

            if let Err(e) = parse() {
                eprintln!("Erreur import : {}", e);
                return;
            }

            println!("Import terminé");
        }

        Commands::Show => {
            gui::run().await;
        }
    }
}
