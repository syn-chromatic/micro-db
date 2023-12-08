#[global_allocator]
static ALLOCATOR: emballoc::Allocator<80_000> = emballoc::Allocator::new();

mod db;
mod error;
mod serializer;
mod stream;
mod structures;

extern crate alloc;
use alloc::collections::BTreeSet;

use std::path;
use std::time::Duration;
use std::time::Instant;

use db::Database;

use serde::{Deserialize, Serialize};

const BLOCK_SIZE: usize = 8;
const CACHE_SIZE: usize = 128;
const EOE_BLOCK: [u8; BLOCK_SIZE] = [0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8];

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
struct ExampleStruct {
    uid: u128,
    start_t: [usize; 2],
    end_t: [usize; 2],
    week: [bool; 7],
}

fn write_items(path: &path::PathBuf) {
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);

    let mut items: BTreeSet<ExampleStruct> = BTreeSet::new();

    for idx in 1..50_000 {
        let my_struct = ExampleStruct {
            uid: idx as u128,
            start_t: [idx, idx],
            end_t: [idx, idx],
            week: [true; 7],
        };
        items.insert(my_struct);
    }

    db.add_items(items);
}

fn write_item(path: &path::PathBuf) {
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);

    for idx in 50..100 {
        let my_struct = ExampleStruct {
            uid: idx as u128,
            start_t: [idx, idx],
            end_t: [idx, idx],
            week: [true; 7],
        };
        db.add_item(&my_struct);
    }
}

fn find_item(path: &path::PathBuf) {
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);

    let idx = 49_000;
    let my_struct = ExampleStruct {
        uid: idx as u128,
        start_t: [idx, idx],
        end_t: [idx, idx],
        week: [true; 7],
    };

    let time = Instant::now();
    println!("Contains: {}", db.contains(&my_struct));
    let taken = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
}

fn get_entry(path: &path::PathBuf) {
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);
    let time = Instant::now();
    let item = db.get_entry(49_000);
    let taken = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
    println!("Item: {:?}", item);
}

fn database_benchmark(path: &path::PathBuf) {
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);
    let db_iterator = db.get_iterator();

    let mut uid: usize = 0;
    let instant: Instant = Instant::now();
    for entry in db_iterator.into_iter() {
        if let Ok(entry) = entry {
            uid = entry.uid;
        }
    }
    let taken: Duration = instant.elapsed();
    println!(
        "Taken: {}ms | Per Entry: {}us | Entries: {}",
        taken.as_millis(),
        taken.as_micros() / uid as u128,
        uid,
    );
}

fn test_database_integrity(path: &path::PathBuf) {
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);
    let db_iterator = db.get_iterator();
    let mut uid: usize = 0;

    for entry in db_iterator.into_iter() {
        if let Ok(entry) = entry {
            if uid == entry.uid {
                print!(
                    "EntryUID: {} | UsedMem: {}KB       \r",
                    entry.uid,
                    ALLOCATOR.get_used_memory() / 1024
                );
                uid += 1;
                continue;
            } else {
                println!("DATABASE INTEGRITY FAIL");
                println!("ENTRY: {:?} | CORRECT UID: {}", entry, uid);
                return;
            }
        }
    }
    println!("\nDATABASE INTEGRITY SUCCESS");
}

fn main() {
    let path = path::PathBuf::from("C:/Users/shady/Desktop/micro-db/database.mdb");
    // write_items(&path);
    // find_item(&path);
    // get_entry(&path);
    test_database_integrity(&path);
    database_benchmark(&path);
}
