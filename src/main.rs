#![cfg_attr(feature = "embedded", no_std)]
#![cfg_attr(feature = "embedded", no_main)]

#[cfg(feature = "std")]
use micro_db::impls::CPath;
#[cfg(feature = "std")]
use micro_db::tests::*;

#[cfg(feature = "std")]
fn main() {
    let path: CPath = CPath::new("./database.mdb");
    // refresh_database(&path);
    remove_database(&path);
    write_entries_from_file(&path);

    remove_database(&path);
    write_entries_at_once(&path);

    remove_database(&path);
    write_entry(&path);

    // find_entry_test(&path);
    // get_entry_test(&path);

    // query_test(&path);

    // database_benchmark(&path);
    // remove_test(&path);
    // remove_loop_test(&path);
    // // print_database(&path);
    // database_integrity_test(&path);
}
