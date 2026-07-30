use std::collections::HashMap;
use std::fs;

use map::object::{
    node::Node,
    way::Way,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut results: Vec<(String, usize, usize)> = Vec::new();

    for entry in fs::read_dir("data/tiles")? {
        let entry = entry?;

        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let name = path.file_name().unwrap().to_string_lossy().to_string();

        let nodes_path = path.join("nodes.bin");
        let ways_path = path.join("ways.bin");

        let nodes_count = if nodes_path.exists() {
            let bytes = fs::read(nodes_path)?;

            let nodes: HashMap<i64, Node> = bincode::deserialize(&bytes)?;

            nodes.len()
        } else {
            0
        };

        let ways_count = if ways_path.exists() {
            let bytes = fs::read(ways_path)?;

            let ways: HashMap<i64, Way> = bincode::deserialize(&bytes)?;

            ways.len()
        } else {
            0
        };

        results.push((name, nodes_count, ways_count));
    }

    results.sort_by(|a, b| (b.1 + b.2).cmp(&(a.1 + a.2)));

    println!("Top 20 tiles :");

    for (i, tile) in results.iter().take(20).enumerate() {
        println!(
            "{} - {} : nodes={} ways={} total={}",
            i + 1,
            tile.0,
            tile.1,
            tile.2,
            tile.1 + tile.2
        );
    }

    Ok(())
}
