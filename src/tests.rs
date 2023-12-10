extern crate alloc;
use alloc::collections::BTreeSet;

use std::path;
use std::time::Duration;
use std::time::Instant;

use crate::db::Database;
use crate::error::DBError;
use crate::structures::DBEntry;
use crate::structures::DBIterator;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
struct ExampleStruct {
    id: u128,
    start_t: [usize; 2],
    end_t: [usize; 2],
    week: [bool; 7],
}

pub fn write_entries_at_once(path: &path::PathBuf) {
    println!("\n[WRITE ENTRIES AT ONCE]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);

    let mut items: BTreeSet<ExampleStruct> = BTreeSet::new();

    for idx in 0..100_000 + 1 {
        let my_struct = ExampleStruct {
            id: idx as u128,
            start_t: [idx, idx],
            end_t: [idx, idx],
            week: [true; 7],
        };
        items.insert(my_struct);
        print!("Idx: {}    \r", idx);
    }

    db.add_entries(items);
    println!();
}

pub fn write_per_entry(path: &path::PathBuf) {
    println!("\n[WRITE PER ENTRY]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);

    for idx in 0..100_000 {
        let item: ExampleStruct = ExampleStruct {
            id: idx as u128,
            start_t: [idx, idx],
            end_t: [idx, idx],
            week: [true; 7],
        };
        db.add_entry(&item);
        print!("Idx: {}     \r", idx);
    }
    println!();
}

pub fn find_entry_test(path: &path::PathBuf) {
    println!("\n[FIND ENTRY TEST]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);

    let idx: usize = 1000;
    let my_struct: ExampleStruct = ExampleStruct {
        id: idx as u128,
        start_t: [idx, idx],
        end_t: [idx, idx],
        week: [true; 7],
    };

    let time: Instant = Instant::now();
    println!("Contains: {}", db.contains(&my_struct));
    let taken: Duration = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
}

pub fn get_entry_test(path: &path::PathBuf) {
    println!("\n[GET ENTRY TEST]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);
    let time: Instant = Instant::now();
    let entry: Result<DBEntry<ExampleStruct>, DBError> = db.get_by_uid(49_000);
    let taken: Duration = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
    println!("Entry: {:?}", entry);
}

pub fn database_benchmark(path: &path::PathBuf) {
    println!("\n[DATABASE BENCHMARK]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);
    let db_iterator: DBIterator<'_, BTreeSet<ExampleStruct>> = db.iterator();

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

pub fn database_integrity_test(path: &path::PathBuf) {
    println!("\n[DATABASE INTEGRITY TEST]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);
    let db_iterator: DBIterator<'_, BTreeSet<ExampleStruct>> = db.iterator();
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
    let _ = db.remove_by_uid(uid);
    let taken: Duration = instant.elapsed();
    println!("Taken: {}ms", taken.as_millis());
}

pub fn query_test(path: &path::PathBuf) {
    println!("\n[QUERY TEST]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);

    let result: Result<DBEntry<ExampleStruct>, DBError> =
        db.query(|s: &ExampleStruct| &s.start_t, [327, 327]);
    println!("Result: {:?}", result);
}

pub fn print_database(path: &path::PathBuf) {
    println!("\n[PRINT DATABASE]");
    let db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, false);
    let db_iterator: DBIterator<'_, BTreeSet<ExampleStruct>> = db.iterator();

    for entry in db_iterator.into_iter() {
        if let Ok(entry) = entry {
            println!("Entry: {:?}", entry);
        }
    }
}

pub fn refresh_database(path: &path::PathBuf) {
    println!("\n[REFRESH DATABASE]");
    let _ = std::fs::remove_file(path);
    write_entries_at_once(path);
}
