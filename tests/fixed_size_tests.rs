extern crate alloc;
extern crate micro_db;

use alloc::collections::BTreeSet;

use std::time::Duration;
use std::time::Instant;

use micro_db::db::Database;
use micro_db::error::DBError;
use micro_db::impls::OpenFile;
use micro_db::structures::DBChunkIterator;
use micro_db::structures::DBEntry;
use micro_db::structures::DBIterator;
use micro_db::traits::CPathTrait;
use micro_db::traits::OpenFileBox;
use micro_db::traits::OpenFileTrait;

use bincode::Decode;
use bincode::Encode;

const WRITE_ENTRIES_AT_ONCE: usize = 100_000 + 1;
const WRITE_ENTRIES: usize = 1000 + 1;
const FIND_ENTRY: usize = 1000;
const REMOVE_TEST: u32 = 0;
const REMOVE_LOOP_TEST: u32 = 100;

#[derive(Encode, Decode, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct FixedSizeStruct {
    id: u128,
    start_t: [usize; 2],
    end_t: [usize; 2],
    week: [bool; 7],
}

pub fn create_fixed_struct(idx: usize) -> FixedSizeStruct {
    let fixed_struct = FixedSizeStruct {
        id: idx as u128,
        start_t: [idx, idx],
        end_t: [idx, idx],
        week: [true; 7],
    };
    fixed_struct
}

pub fn write_entries_at_once(path: &dyn CPathTrait) {
    println!("\n[WRITE ENTRIES AT ONCE]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);

    let time: Instant = Instant::now();
    let mut items: BTreeSet<FixedSizeStruct> = BTreeSet::new();
    for idx in 0..WRITE_ENTRIES_AT_ONCE {
        let item: FixedSizeStruct = create_fixed_struct(idx);
        items.insert(item);
        print!("Idx: {}    \r", idx);
    }

    let _ = db.add_entries(items);
    let taken: Duration = time.elapsed();
    println!("\nTaken: {}ms", taken.as_millis());
}

pub fn write_entries(path: &dyn CPathTrait) {
    println!("\n[WRITE ENTRIES]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);

    let time: Instant = Instant::now();

    for idx in 0..WRITE_ENTRIES {
        let item: FixedSizeStruct = create_fixed_struct(idx);
        let _ = db.add_entry(&item);
    }

    let taken: Duration = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
}

pub fn write_entry(path: &dyn CPathTrait) {
    println!("\n[WRITE ENTRY]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);

    let time: Instant = Instant::now();

    let idx: usize = 0;
    let item: FixedSizeStruct = create_fixed_struct(idx);
    let _ = db.add_entry(&item);

    let taken: Duration = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
}

pub fn find_entry_test(path: &dyn CPathTrait) {
    println!("\n[FIND ENTRY TEST]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);

    let idx: usize = 1000;
    let item: FixedSizeStruct = create_fixed_struct(idx);

    let time: Instant = Instant::now();
    println!("Contains: {:?}", db.contains(&item));
    let taken: Duration = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
}

pub fn get_entry_test(path: &dyn CPathTrait) {
    println!("\n[GET ENTRY TEST]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);
    let time: Instant = Instant::now();
    let entry: Result<DBEntry<FixedSizeStruct>, DBError> = db.get_by_uid(490);
    let taken: Duration = time.elapsed();
    println!("Taken: {}ms", taken.as_millis());
    println!("Entry: {:?}", entry);
}

pub fn database_benchmark(path: &dyn CPathTrait) {
    println!("\n[DATABASE BENCHMARK]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);

    let db_iterator: DBIterator<'_, BTreeSet<FixedSizeStruct>> = db.get_iterator().unwrap();

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
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);

    let db_iterator: DBIterator<'_, BTreeSet<FixedSizeStruct>> = db.get_iterator().unwrap();

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
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);

    let uid: u32 = REMOVE_TEST;
    let instant: Instant = Instant::now();
    let _ = db.remove_by_uid(uid);
    let taken: Duration = instant.elapsed();
    println!("Taken: {}ms", taken.as_millis());
}

pub fn remove_loop_test(path: &dyn CPathTrait) {
    println!("\n[REMOVE TEST]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);

    for uid in (0..REMOVE_LOOP_TEST).rev() {
        let instant: Instant = Instant::now();
        let _ = db.remove_by_uid(uid);
        let taken: Duration = instant.elapsed();
        println!("Taken: {}ms", taken.as_millis());
    }
}

pub fn query_test(path: &dyn CPathTrait) {
    println!("\n[QUERY TEST]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);

    let result: Result<DBEntry<FixedSizeStruct>, DBError> =
        db.query(|s: &FixedSizeStruct| &s.start_t, [327, 327]);
    println!("Result: {:?}", result);
}

pub fn print_database(path: &dyn CPathTrait) {
    println!("\n[PRINT DATABASE]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);

    let db_iterator: DBIterator<'_, BTreeSet<FixedSizeStruct>> = db.get_iterator().unwrap();

    for entry in db_iterator.into_iter() {
        if let Ok(entry) = entry {
            println!("Entry: {:?}", entry);
        }
    }
}

pub fn print_chunk_lens(path: &dyn CPathTrait) {
    println!("\n[PRINT CHUNK LENS]");
    let open: OpenFileBox = OpenFile::new();
    let mut db: Database<'_, BTreeSet<FixedSizeStruct>> = Database::new(path, open);

    let chunk_iterator: DBChunkIterator<'_> = db.get_chunk_iterator().unwrap();

    for chunk in chunk_iterator.into_iter() {
        if let Ok(chunk) = chunk {
            println!("Chunk Len: {}", chunk.len());
        }
    }
}

pub fn refresh_database(path: &dyn CPathTrait) {
    println!("\n[REFRESH DATABASE]");
    remove_database(path);
    write_entries_at_once(path);
    // write_entries(path);
}

pub fn remove_database(path: &dyn CPathTrait) {
    println!("\n[REMOVE DATABASE]");
    let _ = std::fs::remove_file(path.as_str());
}
