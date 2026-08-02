use std::{fs::File, io::BufReader, time::Instant};

use osmpbf::{Element, ElementReader};
use redb::{Database, TableDefinition};

const NODES: TableDefinition<u64, (i32, i32)> = TableDefinition::new("nodes");

const BATCH_SIZE: usize = 1_000_000;

pub fn test_index() -> Result<(), Box<dyn std::error::Error>> {
    let start: Instant = Instant::now();
    let db: Database = Database::create("nodes.redb")?;
    let reader: ElementReader<BufReader<File>> = ElementReader::from_path("paris_centre.pbf")?;
    let mut batch: Vec<(u64, i32, i32)> = Vec::with_capacity(BATCH_SIZE);
    let mut count: u64 = 0u64;

    reader.for_each(|element| {
        if let Element::DenseNode(node) = element {
            batch.push((
                node.id() as u64,
                (node.lat() * 10_000_000.0) as i32,
                (node.lon() * 10_000_000.0) as i32,
            ));

            count += 1;

            if batch.len() == BATCH_SIZE {
                insert_batch(&db, &batch).expect("Erreur écriture redb");

                batch.clear();

                println!("{} nodes importés", count);
            }
        }
    })?;

    if !batch.is_empty() {
        insert_batch(&db, &batch)?;
    }

    println!("Terminé : {} nodes en {:.2?}", count, start.elapsed());

    Ok(())
}

fn insert_batch(
    db: &Database,
    nodes: &[(u64, i32, i32)],
) -> Result<(), Box<dyn std::error::Error>> {
    let txn = db.begin_write()?;

    {
        let mut table = txn.open_table(NODES)?;

        for &(id, lat, lon) in nodes {
            table.insert(id, (lat, lon))?;
        }
    }

    txn.commit()?;

    Ok(())
}
