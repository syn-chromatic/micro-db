use micro_db::impls::PathBuf;
use micro_db::tests::*;
use micro_db::traits::PathBufBox;

fn main() {
    let path: PathBufBox = PathBuf::new("./database.mdb");
    refresh_database(&path);
    // write_entries_at_once(&path);
    // write_per_entry(&path);
    // find_entry_test(&path);
    // get_entry_test(&path);

    // query_test(&path);

    database_benchmark(&path);
    remove_test(&path);
    // print_database(&path);
    database_integrity_test(&path);
}
