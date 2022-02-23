//! Program is used to convey IO tests of C programs, it can manage stdout and stderr.
//! It needs a compiled version of program to test and absolute path to it and folder with tests.
//! Testfolder should be a directory with .in, .out and .err files with matching names.

#[macro_use] extern crate prettytable;
mod settings;
mod interface;
mod testing;

fn main() {
    let mut configuration = settings::Options::new();
    interface::start_program(&mut configuration);
    configuration.save();
}
