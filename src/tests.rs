extern crate alloc;
use alloc::collections::BTreeSet;

use std::time::Duration;
use std::time::Instant;

use crate::db::Database;
use crate::error::DBError;
use crate::impls::OpenFile;
use crate::serializer::DBSerializer;
use crate::stream::DBFileStream;
use crate::structures::DBEntry;
use crate::structures::DBIterator;
use crate::traits::CPathTrait;
use crate::traits::OpenFileBox;
use crate::traits::OpenFileTrait;

use bincode::Decode;
use bincode::Encode;

#[derive(Encode, Decode, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct ExampleStruct {
    id: u128,
    start_t: [usize; 2],
    end_t: [usize; 2],
    week: [bool; 7],
}

pub fn write_entries_at_once(path: &dyn CPathTrait) {
    println!("\n[WRITE ENTRIES AT ONCE]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, open, false);

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

    let _ = db.add_entries(items);
    println!();
}

pub fn write_per_entry(path: &dyn CPathTrait) {
    println!("\n[WRITE PER ENTRY]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, open, false);

    for idx in 0..100_000 {
        let item: ExampleStruct = ExampleStruct {
            id: idx as u128,
            start_t: [idx, idx],
            end_t: [idx, idx],
            week: [true; 7],
        };
        let _ = db.add_entry(&item);
        print!("Idx: {}     \r", idx);
    }
    println!();
}

pub fn find_entry_test(path: &dyn CPathTrait) {
    println!("\n[FIND ENTRY TEST]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, open, false);

    let idx: usize = 1000;
    let my_struct: ExampleStruct = ExampleStruct {
        id: idx as u128,
        start_t: [idx, idx],
        end_t: [idx, idx],
        week: [true; 7],
    };

    let time: Instant = Instant::now();
    println!("Contains: {:?}", db.contains(&my_struct));
    let taken: Duration = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
}

pub fn get_entry_test(path: &dyn CPathTrait) {
    println!("\n[GET ENTRY TEST]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, open, false);
    let time: Instant = Instant::now();
    let entry: Result<DBEntry<ExampleStruct>, DBError> = db.get_by_uid(49_000);
    let taken: Duration = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
    println!("Entry: {:?}", entry);
}

pub fn database_benchmark(path: &dyn CPathTrait) {
    println!("\n[DATABASE BENCHMARK]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, open, false);

    let mut file = db.get_file();
    let db_stream = DBFileStream::new(&mut file);
    let db_serializer: DBSerializer<'_, BTreeSet<ExampleStruct>> = DBSerializer::new();
    let db_iterator: DBIterator<'_, BTreeSet<ExampleStruct>> =
        DBIterator::new(db_stream, db_serializer);

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

pub fn database_integrity_test(path: &dyn CPathTrait) {
    println!("\n[DATABASE INTEGRITY TEST]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, open, false);

    let mut file = db.get_file();
    let db_stream = DBFileStream::new(&mut file);
    let db_serializer: DBSerializer<'_, BTreeSet<ExampleStruct>> = DBSerializer::new();
    let db_iterator: DBIterator<'_, BTreeSet<ExampleStruct>> =
        DBIterator::new(db_stream, db_serializer);

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

pub fn remove_test(path: &dyn CPathTrait) {
    println!("\n[REMOVE TEST]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, open, false);

    let uid: u32 = 2;
    let instant: Instant = Instant::now();
    let _ = db.remove_by_uid(uid);
    let taken: Duration = instant.elapsed();
    println!("Taken: {}ms", taken.as_millis());
}

pub fn remove_loop_test(path: &dyn CPathTrait) {
    println!("\n[REMOVE TEST]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, open, false);

    for uid in 0..500 {
        let instant: Instant = Instant::now();
        let _ = db.remove_by_uid(uid);
        let taken: Duration = instant.elapsed();
        println!("Taken: {}ms", taken.as_millis());
    }
}

pub fn query_test(path: &dyn CPathTrait) {
    println!("\n[QUERY TEST]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, open, false);

    let result: Result<DBEntry<ExampleStruct>, DBError> =
        db.query(|s: &ExampleStruct| &s.start_t, [327, 327]);
    println!("Result: {:?}", result);
}

pub fn print_database(path: &dyn CPathTrait) {
    println!("\n[PRINT DATABASE]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<ExampleStruct>> = Database::new(path, open, false);

    let mut file = db.get_file();
    let db_stream = DBFileStream::new(&mut file);
    let db_serializer: DBSerializer<'_, BTreeSet<ExampleStruct>> = DBSerializer::new();
    let db_iterator: DBIterator<'_, BTreeSet<ExampleStruct>> =
        DBIterator::new(db_stream, db_serializer);

    for entry in db_iterator.into_iter() {
        if let Ok(entry) = entry {
            println!("Entry: {:?}", entry);
        }
    }
}

pub fn refresh_database(path: &dyn CPathTrait) {
    println!("\n[REFRESH DATABASE]");
    let _ = std::fs::remove_file(path.as_str());
    write_entries_at_once(path);
}
