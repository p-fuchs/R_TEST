pub mod test_result;
pub mod test_enums;

pub use test_result::TestResult;
use std::fs::DirEntry;
use rayon::prelude::*;

use crate::settings::Options;

/**
Checks wheter a given file is a file containing input
*/
fn is_infile(to_test: &DirEntry) -> bool {
    let file_test_path = to_test.path();
    let extension = file_test_path.extension()
        .unwrap_or_else(|| panic!("ERROR: Reading extension of {:?} FAILED.", &file_test_path));

    extension == "in"
}

/// Main function to run tests. Produces a vector of results.
pub fn run_testing(settings: &Options) -> Vec<TestResult> {
    let mut list = TestResult::load(settings.get_test_path());
    
    if settings.get_valgrind_activity() {
        list.par_iter_mut()
        .enumerate()
        .for_each(|(index, frame)| {
            frame.test_with_valgrind(settings.get_program_path(), index, settings.get_stderr_option())
        });
    } else {
        list.par_iter_mut()
        .enumerate()
        .for_each(|(index, frame)| {
            frame.test_no_valgrind(settings.get_program_path(), index, settings.get_stderr_option())
        });
    }

    list
}