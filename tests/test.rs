use std::path::PathBuf;

use compiler::db::RootDatabase;
use compiler::diagnostics::check_and_eprint_diagnostics;
use compiler::project::setup_project;
use filesystem::ids::CrateId;
use num_bigint::BigInt;
use runner::{RunResultValue, SierraCasmRunner};
use sierra_generator::db::SierraGenGroup;
use sierra_generator::replace_ids::replace_sierra_ids_in_program;
use test_case::test_case;

fn setup(name: &str) -> (RootDatabase, Vec<CrateId>) {
    let dir = env!("CARGO_MANIFEST_DIR");
    // Pop the "/tests" suffix.
    let mut path = PathBuf::from(dir).parent().unwrap().to_owned();
    path.push("src");
    path.push(format!("{name}.cairo"));

    let mut db = RootDatabase::default();
    let main_crate_ids = setup_project(&mut db, path.as_path()).expect("Project setup failed.");
    assert!(!check_and_eprint_diagnostics(&mut db));
    (db, main_crate_ids)
}

/// Compiles the Cairo code for `name` to a Sierra program.
fn checked_compile_to_sierra(name: &str) -> sierra::program::Program {
    let (db, main_crate_ids) = setup(name);
    let sierra_program = db.get_sierra_program(main_crate_ids).unwrap();
    replace_sierra_ids_in_program(&db, &sierra_program)
}

#[test_case(
    "add_one",
    &[41].map(BigInt::from) =>
    RunResultValue::Success(vec![BigInt::from(42)]);
    "add_one"
)]
#[test_case(
    "add_one",
    &[0].map(BigInt::from) =>
    RunResultValue::Success(vec![BigInt::from(1)]);
    "add_one_to_zero"
)]
fn run_function_test(name: &str, params: &[BigInt]) -> RunResultValue {
    let runner = SierraCasmRunner::new(checked_compile_to_sierra(name), false)
        .expect("Failed setting up runner.");
    let result = runner
        .run_function(/* find first */ "", params, &None)
        .expect("Failed running the function.");
    result.value
}
