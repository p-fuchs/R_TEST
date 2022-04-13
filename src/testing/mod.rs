pub mod test_enums;
pub mod test_result;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::fs::DirEntry;
pub use test_result::TestResult;

use crate::settings::Options;

/**
Checks wheter a given file is a file with .in extension
*/
fn is_infile(to_test: &DirEntry) -> bool {
    let file_test_path = to_test.path();
    let extension = file_test_path.extension();

    match extension {
        Some(extension) => extension == "in",
        None => false,
    }
}

/**
Checks wheter a given file is a file with .c extension
*/
fn is_cfile(to_test: &DirEntry) -> bool {
    let file_test_path = to_test.path();
    let extension = file_test_path.extension();

    match extension {
        Some(extension) => extension == "c",
        None => false,
    }
}

/// Main function to run tests. Produces a vector of results.
pub fn run_testing(settings: &Options) -> Vec<TestResult> {
    if settings.get_program_mode() {
        run_compiled_version(settings)
    } else {
        run_feeded_version(settings)
    }
}

fn run_compiled_version(settings: &Options) -> Vec<TestResult> {
    let mut list = TestResult::load_c(settings.get_test_path());
    list.sort();
    let length = list.len();

    if settings.get_valgrind_activity() {
        list.par_iter_mut()
            .progress_count(length as u64)
            .enumerate()
            .for_each(|(index, frame)| {
                frame.test_compiled_with_valgrind(
                    settings.get_program_path(),
                    index,
                    settings.get_stderr_option(),
                )
            });
    } else {
        list.par_iter_mut()
            .progress_count(length as u64)
            .enumerate()
            .for_each(|(index, frame)| {
                frame.test_compiled_no_valgrind(
                    settings.get_program_path(),
                    index,
                    settings.get_stderr_option(),
                )
            });
    }

    list
}

fn run_feeded_version(settings: &Options) -> Vec<TestResult> {
    let mut list = TestResult::load(settings.get_test_path());
    list.sort();
    let length = list.len();

    if settings.get_valgrind_activity() {
        list.par_iter_mut()
            .progress_count(length as u64)
            .enumerate()
            .for_each(|(index, frame)| {
                frame.test_with_valgrind(
                    settings.get_program_path(),
                    index,
                    settings.get_stderr_option(),
                )
            });
    } else {
        list.par_iter_mut()
            .progress_count(length as u64)
            .enumerate()
            .for_each(|(index, frame)| {
                frame.test_no_valgrind(
                    settings.get_program_path(),
                    index,
                    settings.get_stderr_option(),
                )
            });
    }

    list
}
