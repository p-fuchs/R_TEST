use std::fs::{self, DirEntry, File};
use std::io::{self, Write};
use rayon::prelude::*;

use crate::settings::Options;
use std::process::{Command, Stdio};

#[derive(Debug)]
enum DiffResult {
    Ok,
    Difference(String),
    Trouble(String),
    InnerProblem(String)
}
#[derive(Debug)]
enum TestFail {
    Valgrind(String),
    Diff(DiffResult),
    //LackOfStdout,
    //LackOfStderr,
    InnerProblem(String)
}

impl TestFail {
    fn get_problem(&self) -> &str {
        match self {
            TestFail::Valgrind(err) => &err,
            TestFail::InnerProblem(err) => &err,
            TestFail::Diff(diff_error) => {
                match diff_error {
                    DiffResult::Difference(err) => &err,
                    DiffResult::InnerProblem(err) => &err,
                    DiffResult::Trouble(err) => &err,
                    _ => "UNDEFINED BEHAVIOUR OF GET_PROBLEM FUNCTION"
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct TestResult {
    name: String,
    passed: bool,
    time: f32,
    failed_cause: TestFail
}

impl TestResult {
    fn new(path: &str) -> TestResult {
        TestResult {
            name: path.to_string(),
            passed: false,
            time: 0.0,
            failed_cause: TestFail::InnerProblem("".to_string())
        }
    }

    fn load(path: &str) -> Vec<TestResult> {
        let source = fs::read_dir(path)
            .expect("ERROR: Opening directory with tests FAILED.");

        let mut result = Vec::new();
        for file in source {
            let entry = file.expect("ERROR: Reading tests FAILED.");
            if is_infile(&entry) {
                result.push(TestResult::new(entry.path().to_str().unwrap()));
            }
        }
        result
    }

    fn get_core(&self) -> String {
        let mut result = String::new();
        let mut iterator = self.name.split('.');
        result.push_str(iterator.next().unwrap());
        result
    }

    fn get_stdout_file(&self) -> String {
        let mut result = self.get_core();
        result.push_str(".out");
        result
    }

    pub fn get_name(&self) -> String {
        let mut iterator = self.name.split('/');
        let name = iterator.last().unwrap();
        name.to_string()
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }

    fn get_stderr_file(&self) -> String {
        let mut result = self.get_core();
        result.push_str(".err");
        result
    }

    pub fn passed(&self) -> bool {
        self.passed
    }

    fn run_valgrind(&mut self, index: usize, program: &str) -> bool {
        let divert_output = format!("rtest_stdout{}", index);
        let divert_error = format!("rtest_stderr{}", index);
        let mut input_file = File::open(&self.name).expect("Failed to open input");
        let mut output_file = File::create(divert_output).expect("Failed to open outputFile");
        let mut error_file = File::create(divert_error).expect("Failed to create errorFile");

        let mut process = Command::new("valgrind")
            .arg("--leak-check=full")
            .arg("--error-exitcode=1")
            .arg("-q")
            .arg(program)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("ERROR: Spawning child process of Valgrind FAILED.");
        
        io::copy(&mut input_file, process.stdin.as_mut().unwrap()).unwrap();

        let process = process.wait_with_output();

        match process {
            Err(e) => {
                self.failed_cause = TestFail::InnerProblem(e.to_string());
                false
            }
            Ok(output) => {
                let status = output.status.code().expect("ERROR: Reading exitcode of valgrind FAILED.");
                println!("STATUS VAL {}", status);
                match status {
                    0 => {
                        let stdout_result = String::from_utf8_lossy(&output.stdout).to_string();
                        let stderr_result = String::from_utf8_lossy(&output.stderr).to_string();
                        write!(&mut output_file, "{}", stdout_result).unwrap();
                        write!(&mut error_file, "{}", stderr_result).unwrap();
                        true
                    }
                    _ => {
                        let failed_result = String::from_utf8_lossy(&output.stderr).to_string();
                        println!("VAL FAIL {}", &failed_result);
                        self.failed_cause = TestFail::Valgrind(failed_result);
                        false
                    }
                }
            }
        }
    }

    fn diff_files(input_diff: &str, output_diff: &str) -> DiffResult {
        let process = Command::new("diff")
            .arg("-c")
            .arg(input_diff)
            .arg(output_diff)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();
        
        match process {
            Err(e) => {
                DiffResult::InnerProblem(e.to_string())
            }
            Ok(output) => {
                let status = output.status.code().expect("ERROR: Reading exitcode of diff FAILED.");
                match status {
                    0 => {
                        DiffResult::Ok
                    }
                    1 => {
                        let diff_result = String::from_utf8_lossy(&output.stdout).to_string();
                        DiffResult::Difference(diff_result)
                    }
                    _ => {
                        let diff_error = String::from_utf8_lossy(&output.stderr).to_string();
                        DiffResult::Trouble(diff_error)
                    }
                }
            }
        }
    }

    fn test_with_valgrind(&mut self, program_path: &str, index: usize) {
        use std::time::SystemTime;
        let beggining = SystemTime::now();
        if self.run_valgrind(index, program_path) && self.run_diff(index){
            self.passed = true;
        }

        let stdout = format!("rtest_stdout{}", index);
        let stderr = format!("rtest_stderr{}", index);
        let _ = fs::remove_file(stdout);
        let _ = fs::remove_file(stderr);

        self.time = SystemTime::now()
            .duration_since(beggining)
            .unwrap_or_else(|_| panic!("ERROR: Time_calculation of {:?} FAILED.", self.name))
            .as_secs_f32();
    }

    fn run_diff(&mut self, index: usize) -> bool {
        let stdout_input = format!("rtest_stdout{}", index);
        let stderr_input = format!("rtest_stderr{}", index);
        let stdout_output = self.get_stdout_file();
        let stderr_output = self.get_stderr_file();
        
        let stdout_result = TestResult::diff_files(&stdout_input, &stdout_output);

        match stdout_result {
            DiffResult::Ok => {},
            other => {
                self.failed_cause = TestFail::Diff(other);
                return false;
            }
        }

        let stderr_result = TestResult::diff_files(&stderr_input, &stderr_output);

        match stderr_result {
            DiffResult::Ok => true,
            other => {
                self.failed_cause = TestFail::Diff(other);
                false
            }
        }
    }

    pub fn get_problem_description(&self) -> String {
        match &self.failed_cause {
            TestFail::Valgrind(_) => {
                "Valgrind ERROR".to_string()
            },
            TestFail::Diff(diff_error) => {
                match diff_error {
                    DiffResult::Difference(_) => {
                        "Diff ERROR: Difference".to_string()
                    },
                    DiffResult::InnerProblem(_) => {
                        "Diff ERROR: InnerProblem".to_string()
                    }
                    DiffResult::Trouble(_) => {
                        "Diff ERROR: Trouble".to_string()
                    }
                    _ => {
                        "PROGRAM UNDEFINED DIFF ERROR".to_string()
                    }
                }
            },
            TestFail::InnerProblem(_) => {
                "PROGRAM INNER PROBLEM".to_string()
            }
        }
    }

    pub fn get_problem(&self) -> &str {
        self.failed_cause.get_problem()
    }
}

fn is_infile(to_test: &DirEntry) -> bool {
    let file_name = to_test.path();
    let extension = file_name.extension()
        .unwrap_or_else(|| panic!("ERROR: Reading extension of {:?} FAILED.", &file_name));

    extension == "in"
}

pub fn run_testing(settings: &Options) -> Vec<TestResult> {
    let mut list = TestResult::load(settings.get_test_path());
    list.par_iter_mut()
        .enumerate()
        .for_each(|(index, frame)| {
            frame.test_with_valgrind(settings.get_program_path(), index)
        });
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_stdout_test() {
        let ts = TestResult {
            name: "/usr/bin/a/b/c/def.in".to_string(),
            passed: false,
            time: 0.0,
            failed_cause: TestFail::InnerProblem("".to_string())
        };
        
        assert!(ts.get_stdout_file() == "/usr/bin/a/b/c/def.out");
    }

    #[test]
    fn get_stderr_test() {
        let ts = TestResult {
            name: "/usr/bin/a/b/c/def.in".to_string(),
            passed: false,
            time: 0.0,
            failed_cause: TestFail::InnerProblem("".to_string())
        };
        
        assert!(ts.get_stderr_file() == "/usr/bin/a/b/c/def.err");
    }

    #[test]
    fn get_name_test() {
        let ts = TestResult {
            name: "/usr/bin/a/b/c/def.in".to_string(),
            passed: false,
            time: 0.0,
            failed_cause: TestFail::InnerProblem("".to_string())
        };

        assert!(ts.get_name() == "def.in");
    }
}