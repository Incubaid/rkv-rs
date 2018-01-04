// Not: apparently it doesn't see usage in test mods as usage
// TODO: check how to add test_utils

pub const TEST_DIR: &str = "/tmp/test-rkv";
use std::fs;
use std::panic;

fn setup() {
    fs::create_dir_all(TEST_DIR).unwrap();
}

fn teardown() {
    fs::remove_dir_all(TEST_DIR).unwrap();
}

#[allow(dead_code)]
pub fn run_test<T>(test: T) -> ()
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    setup();
    let result = panic::catch_unwind(test);
    teardown();
    assert!(result.is_ok())
}
