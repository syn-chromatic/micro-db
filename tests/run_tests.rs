mod fixed_size_tests;
mod variable_size_tests;

extern crate micro_db;
use micro_db::impls::CPath;

#[test]
fn run_all() {
    fixed_size_test();
    variable_size_test();
}

#[test]
fn fixed_size_test() {
    println!("[FIXED SIZE TEST]");
    let path: CPath = CPath::new("./database.mdb");
    fixed_size_tests::refresh_database(&path);

    // fixed_size_tests::write_entries_from_file(&path);
    // fixed_size_tests::write_entries_at_once(&path);
    // fixed_size_tests::write_entry(&path);

    fixed_size_tests::find_entry_test(&path);
    fixed_size_tests::get_entry_test(&path);

    fixed_size_tests::query_test(&path);

    // fixed_size_tests::print_chunk_lens(&path);

    // fixed_size_tests::database_benchmark(&path);
    // fixed_size_tests::remove_test(&path);
    fixed_size_tests::remove_loop_test(&path);
    fixed_size_tests::database_benchmark(&path);
    // fixed_size_tests::print_database(&path);
    fixed_size_tests::database_integrity_test(&path);
}

#[test]
fn variable_size_test() {
    println!("[VARIABLE SIZE TEST]");
    let path: CPath = CPath::new("./database.mdb");
    variable_size_tests::refresh_database(&path);

    // variable_size_tests::write_entries_from_file(&path);
    // variable_size_tests::write_entries_at_once(&path);
    // variable_size_tests::write_entry(&path);

    variable_size_tests::find_entry_test(&path);
    variable_size_tests::get_entry_test(&path);

    // variable_size_tests::query_test(&path);

    // variable_size_tests::print_chunk_lens(&path);

    // variable_size_tests::database_benchmark(&path);
    // variable_size_tests::remove_test(&path);
    variable_size_tests::database_benchmark(&path);
    variable_size_tests::remove_loop_test(&path);
    variable_size_tests::database_benchmark(&path);
    // variable_size_tests::print_database(&path);
    variable_size_tests::database_integrity_test(&path);
}
