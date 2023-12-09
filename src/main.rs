// #[global_allocator]
// static ALLOCATOR: emballoc::Allocator<20_000> = emballoc::Allocator::new();

mod db;
mod error;
mod serializer;
mod stream;
mod structures;
mod utils;
mod file;

extern crate alloc;
use alloc::collections::BTreeSet;

use std::path;
use std::time::Duration;
use std::time::Instant;

use db::Database;
use error::DBError;
use structures::DBEntry;
use structures::DBIterator;

use serde::Deserialize;
use serde::Serialize;

const BLOCK_SIZE: usize = 4;
const CACHE_SIZE: usize = 2048;
const EOE_BLOCK: [u8; BLOCK_SIZE] = [0xFF, 0xFE, 0xFD, 0xFC];

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
struct ExampleStruct {
    uid: u128,
    start_t: [usize; 2],
    end_t: [usize; 2],
    week: [bool; 7],
}

fn write_items_at_once(path: &path::PathBuf) {
    println!("\n[WRITE ITEMS AT ONCE]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);

    let mut items: BTreeSet<ExampleStruct> = BTreeSet::new();

    for idx in 0..100_000 + 1 {
        let my_struct = ExampleStruct {
            uid: idx as u128,
            start_t: [idx, idx],
            end_t: [idx, idx],
            week: [true; 7],
        };
        items.insert(my_struct);
        print!("Idx: {}    \r", idx);
    }

    db.add_items(items);
    println!();
}

fn write_per_item(path: &path::PathBuf) {
    println!("\n[WRITE PER ITEM]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);

    for idx in 0..100_000 {
        let my_struct: ExampleStruct = ExampleStruct {
            uid: idx as u128,
            start_t: [idx, idx],
            end_t: [idx, idx],
            week: [true; 7],
        };
        db.add_item(&my_struct);
        print!("Idx: {}     \r", idx);
    }
    println!();
}

fn find_entry_test(path: &path::PathBuf) {
    println!("\n[FIND ENTRY TEST]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);

    let idx: usize = 49_000;
    let my_struct: ExampleStruct = ExampleStruct {
        uid: idx as u128,
        start_t: [idx, idx],
        end_t: [idx, idx],
        week: [true; 7],
    };

    let time: Instant = Instant::now();
    println!("Contains: {}", db.contains(&my_struct));
    let taken: Duration = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
}

fn get_entry_test(path: &path::PathBuf) {
    println!("\n[GET ENTRY TEST]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);
    let time: Instant = Instant::now();
    let item: Result<DBEntry<ExampleStruct>, DBError> = db.get_entry(49_000);
    let taken: Duration = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
    println!("Item: {:?}", item);
}

fn database_benchmark(path: &path::PathBuf) {
    println!("\n[DATABASE BENCHMARK]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);
    let db_iterator: DBIterator<'_, BTreeSet<ExampleStruct>> = db.get_iterator();

    let mut uid: u32 = 0;
    let instant: Instant = Instant::now();
    for entry in db_iterator.into_iter() {
        if let Ok(entry) = entry {
            uid = entry.uid;
        }
    }
    let taken: Duration = instant.elapsed();
    println!(
        "Taken: {}ms | Per Entry: {:.2}us | Entries: {}",
        taken.as_millis(),
        taken.as_micros() as f64 / uid as f64,
        uid,
    );
}

fn database_integrity_test(path: &path::PathBuf) {
    println!("\n[DATABASE INTEGRITY TEST]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);
    let db_iterator: DBIterator<'_, BTreeSet<ExampleStruct>> = db.get_iterator();
    let mut uid: u32 = 0;

    for entry in db_iterator.into_iter() {
        if let Ok(entry) = entry {
            if uid == entry.uid {
                // print!(
                //     "EntryUID: {} | UsedMem: {}KB       \r",
                //     entry.uid,
                //     ALLOCATOR.get_used_memory() / 1024
                // );
                uid += 1;
                continue;
            } else {
                println!("DATABASE INTEGRITY FAIL");
                println!("FAIL AT ENTRY: {:?} | CORRECT UID: {}", entry, uid);
                return;
            }
        }
    }
    println!("DATABASE INTEGRITY SUCCESS");
}

pub fn remove_test(path: &path::PathBuf) {
    println!("\n[REMOVE TEST]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);

    let uid: u32 = 2;
    let instant: Instant = Instant::now();
    let _ = db.remove_entry(uid);
    let taken: Duration = instant.elapsed();
    println!("Taken: {}ms", taken.as_millis());
}

pub fn print_database(path: &path::PathBuf) {
    println!("\n[PRINT DATABASE]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);
    let db_iterator: DBIterator<'_, BTreeSet<ExampleStruct>> = db.get_iterator();

    for entry in db_iterator.into_iter() {
        if let Ok(entry) = entry {
            println!("Entry: {:?}", entry);
        }
    }
}

pub fn refresh_database(path: &path::PathBuf) {
    println!("\n[REFRESH DATABASE]");
    let _ = std::fs::remove_file(path);
    write_items_at_once(path);
}

fn main() {
    let path = path::PathBuf::from("C:/Users/shady/Desktop/micro-db/database.mdb");
    refresh_database(&path);
    // write_items_at_once(&path);
    // write_per_item(&path);
    // find_entry_test(&path);
    // get_entry_test(&path);

    database_benchmark(&path);
    remove_test(&path);
    // print_database(&path);
    database_integrity_test(&path);
}
