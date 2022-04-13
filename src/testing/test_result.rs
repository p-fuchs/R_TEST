use std::fs::{self, File};
use std::io::{self, Write};
use std::process::{Command, Stdio};

use super::test_enums::{DiffResult, TestFail};
use super::{is_cfile, is_infile};
/// Structure to manage testing
#[derive(Debug)]
pub struct TestResult {
    test_path: String,
    passed: bool,
    time: f32,
    failed_cause: TestFail,
    return_code: i32,
    compilation_warnings: Option<String>,
}

impl PartialEq for TestResult {
    fn eq(&self, other: &Self) -> bool {
        self.test_path == other.test_path
    }
}

impl Eq for TestResult {}

impl PartialOrd for TestResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.test_path.partial_cmp(&other.test_path)
    }
}

impl Ord for TestResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.test_path.cmp(&other.test_path)
    }
}

impl TestResult {
    /**
    Creates new structure with given absolute path of test
    */
    fn new(path: &str) -> TestResult {
        TestResult {
            test_path: path.to_string(),
            passed: false,
            time: 0.0,
            failed_cause: TestFail::InnerProblem("".to_string()),
            return_code: 5,
            compilation_warnings: None,
        }
    }

    pub fn get_exit_code(&self) -> i32 {
        self.return_code
    }

    /// Returns wheter there occured valgrind error while testing (true - occured).
    /// WARNING: It should be used only on struct, which was tested in the past
    pub fn valgrind_error(&self) -> bool {
        matches!(self.failed_cause, TestFail::Valgrind(_))
    }

    /// Returns wheter there occured diff error while testing (true - occured)
    /// WARNING: It should be used only on struct, which was tested in the past.
    pub fn diff_error(&self) -> bool {
        matches!(self.failed_cause, TestFail::Diff(_))
    }

    /**
    Creates a vector of TestResults from every single file with .in extension
    in given absolue path.
    */
    pub(super) fn load(path: &str) -> Vec<TestResult> {
        let source = fs::read_dir(path).expect("ERROR: Opening directory with tests FAILED.");

        let mut result = Vec::new();
        for file in source {
            let entry = file.expect("ERROR: Reading tests FAILED.");
            if is_infile(&entry) {
                result.push(TestResult::new(entry.path().to_str().unwrap()));
            }
        }
        result
    }

    /**
    Creates a vector of TestResults from every single file with .c extension
    in given absolue path.
    */
    pub(super) fn load_c(path: &str) -> Vec<TestResult> {
        let source = fs::read_dir(path).expect("ERROR: Opening directory with tests FAILED.");

        let mut result = Vec::new();
        for file in source {
            let entry = file.expect("ERROR: Reading tests FAILED.");
            if is_cfile(&entry) {
                result.push(TestResult::new(entry.path().to_str().unwrap()));
            }
        }
        result
    }

    /**
    Returns a whole path without extension.
    EXAMPLE: test_path = /usr/bin/abc.de -> /usr/bin/abc
    */
    fn get_core(&self) -> String {
        let mut result = String::new();
        let mut iterator = self.test_path.split('.');
        result.push_str(iterator.next().unwrap());
        result
    }

    /**
    Returns a path of .out file.
    EXAMPLE -> test_path = /usr/bin/abc.de -> /usr/bin/abc.out
    */
    fn get_stdout_file(&self) -> String {
        let mut result = self.get_core();
        result.push_str(".out");
        result
    }

    /// Returns a name of testfile.
    /// EXAMPLE -> test_path = /usr/bin/abc.de -> abc.in
    pub fn get_name(&self) -> String {
        let iterator = self.test_path.split('/');
        let test_path = iterator.last().unwrap();
        test_path.to_string()
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }

    /**
    Returns a path of .err file.
    EXAMPLE -> test_path = /usr/bin/abc.de -> /usr/bin/abc.err
    */
    fn get_stderr_file(&self) -> String {
        let mut result = self.get_core();
        result.push_str(".err");
        result
    }

    /// Returns true when test was succesfully done.
    /// WARNING: It should be used only on struct, which was tested in the past.
    pub fn passed(&self) -> bool {
        self.passed
    }

    /**
    Runs a program without valgrind and saves its stdout and stderr to a file in main directory,
    which is based of index. Index should be an unique number to all of the conducted tests.
    */
    fn run_program(&mut self, index: usize, program: &str) -> bool {
        let divert_output = format!("rtest_stdout{}", index);
        let divert_error = format!("rtest_stderr{}", index);
        let mut input_file = File::open(&self.test_path).expect("Failed to open input");
        let mut output_file = File::create(divert_output).expect("Failed to open outputFile");
        let mut error_file = File::create(divert_error).expect("Failed to create errorFile");

        let mut process = Command::new(program)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("ERROR: Spawning child process of Program FAILED.");

        io::copy(&mut input_file, process.stdin.as_mut().unwrap()).unwrap();

        let process = process.wait_with_output();

        match process {
            Err(e) => {
                self.failed_cause = TestFail::InnerProblem(e.to_string());
                false
            }
            Ok(output) => {
                let status = output.status.code();

                if let Some(exit_code) = status {
                    let stdout_result = String::from_utf8_lossy(&output.stdout).to_string();
                    let stderr_result = String::from_utf8_lossy(&output.stderr).to_string();
                    write!(&mut output_file, "{}", stdout_result).unwrap();
                    write!(&mut error_file, "{}", stderr_result).unwrap();
                    self.return_code = exit_code;
                    true
                } else {
                    self.failed_cause = TestFail::ProgramExitCode();
                    false
                }
            }
        }
    }

    /**
    Compiles a program to be tested. Program_path should be precompiled .o library.
    */
    fn compile_program(&mut self, program_path: &str, index: usize) -> bool {
        let compiled_program = format!("rtest_compilation{}", index);

        let process = Command::new("gcc")
            .arg("-O2")
            .arg("-Wall")
            .arg("-Wextra")
            .arg("-Wno-implicit-fallthrough")
            .arg(&self.test_path)
            .arg(program_path)
            .arg("-o")
            .arg(compiled_program)
            .stderr(Stdio::piped())
            .spawn()
            .expect("ERROR: Spawning child process of GCC FAILED.");

        let process = process.wait_with_output();

        match process {
            Err(e) => {
                self.failed_cause = TestFail::InnerProblem(e.to_string());
                false
            }
            Ok(output) => {
                let status = output.status.code();

                if let Some(exit_code) = status {
                    match exit_code {
                        1 => {
                            let failed_result = String::from_utf8_lossy(&output.stderr).to_string();
                            self.failed_cause = TestFail::Compilation(failed_result);
                            false
                        }
                        _ => {
                            let compilation_warning =
                                String::from_utf8_lossy(&output.stderr).to_string();
                            self.compilation_warnings = Some(compilation_warning);
                            true
                        }
                    }
                } else {
                    self.failed_cause = TestFail::CompilationExitCode;
                    false
                }
            }
        }
    }

    /**
    Runs a program with valgrind and saves its stdout and stderr to a file in main directory,
    which is based of index. Index should be an unique number to all of the conducted tests.
    */
    fn run_valgrind(&mut self, index: usize, program: &str) -> bool {
        let divert_output = format!("rtest_stdout{}", index);
        let divert_error = format!("rtest_stderr{}", index);
        let mut input_file = File::open(&self.test_path).expect("Failed to open input");
        let mut output_file = File::create(divert_output).expect("Failed to open outputFile");
        let mut error_file = File::create(divert_error).expect("Failed to create errorFile");

        let mut process = Command::new("valgrind")
            .arg("--leak-check=full")
            .arg("--error-exitcode=-573")
            .arg("--show-leak-kinds=all")
            .arg("--errors-for-leak-kinds=all")
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
                let status = output.status.code();

                if let Some(exit_code) = status {
                    match exit_code {
                        -573 => {
                            let failed_result = String::from_utf8_lossy(&output.stderr).to_string();
                            self.failed_cause = TestFail::Valgrind(failed_result);
                            false
                        }

                        return_code => {
                            let stdout_result = String::from_utf8_lossy(&output.stdout).to_string();
                            let stderr_result = String::from_utf8_lossy(&output.stderr).to_string();
                            write!(&mut output_file, "{}", stdout_result).unwrap();
                            write!(&mut error_file, "{}", stderr_result).unwrap();
                            self.return_code = return_code;
                            println!("OUTPUT FILE {}", stdout_result);
                            true
                        }
                    }
                } else {
                    self.failed_cause = TestFail::ValgrindExitCode();
                    false
                }
            }
        }
    }

    /**
    Compares two files using diff program. input_diff indicates path to program generated file
    output_diff indicates path to template file
    */
    fn diff_files(input_diff: &str, output_diff: &str) -> DiffResult {
        let process = Command::new("diff")
            .arg("-c")
            .arg(input_diff)
            .arg(output_diff)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match process {
            Err(e) => DiffResult::InnerProblem(e.to_string()),
            Ok(output) => {
                let status = output
                    .status
                    .code()
                    .expect("ERROR: Reading exitcode of diff FAILED.");
                match status {
                    0 => DiffResult::Ok,
                    1 => {
                        let diff_result = String::from_utf8_lossy(&output.stdout).to_string();
                        DiffResult::DifferenceNotSpecified(diff_result)
                    }
                    _ => {
                        let diff_error = String::from_utf8_lossy(&output.stderr).to_string();
                        DiffResult::Trouble(diff_error)
                    }
                }
            }
        }
    }

    /**
    Conducts a test process with compilation and valgrind usage.
    */
    pub(super) fn test_compiled_with_valgrind(
        &mut self,
        program_path: &str,
        index: usize,
        use_stderr: bool,
    ) {
        use std::time::SystemTime;
        let beggining = SystemTime::now();

        if !self.compile_program(program_path, index) {
            self.passed = false;
            return;
        }

        let compiled_path = format!("./rtest_compilation{}", index);
        if self.run_valgrind(index, &compiled_path) && self.run_diff(index, use_stderr) {
            self.passed = true;
        }

        let stdout = format!("rtest_stdout{}", index);
        let stderr = format!("rtest_stderr{}", index);
        let compiled = format!("rtest_compilation{}", index);
        let _ = fs::remove_file(stdout);
        let _ = fs::remove_file(stderr);
        let _ = fs::remove_file(compiled);

        self.time = SystemTime::now()
            .duration_since(beggining)
            .unwrap_or_else(|_| panic!("ERROR: Time_calculation of {:?} FAILED.", self.test_path))
            .as_secs_f32();
    }

    /**
    Conducts a test process with compilation and without valgrind usage.
    */
    pub(super) fn test_compiled_no_valgrind(
        &mut self,
        program_path: &str,
        index: usize,
        use_stderr: bool,
    ) {
        use std::time::SystemTime;
        let beggining = SystemTime::now();

        if !self.compile_program(program_path, index) {
            self.passed = false;
            return;
        }

        let compiled_path = format!("./rtest_compiled{}", index);
        if self.run_program(index, &compiled_path) && self.run_diff(index, use_stderr) {
            self.passed = true;
        }

        let stdout = format!("rtest_stdout{}", index);
        let stderr = format!("rtest_stderr{}", index);
        let compiled = format!("rtest_compilation{}", index);
        let _ = fs::remove_file(stdout);
        let _ = fs::remove_file(stderr);
        let _ = fs::remove_file(compiled);

        self.time = SystemTime::now()
            .duration_since(beggining)
            .unwrap_or_else(|_| panic!("ERROR: Time_calculation of {:?} FAILED.", self.test_path))
            .as_secs_f32();
    }

    /**
    Conducts a test process with using valgrind
    */
    pub(super) fn test_with_valgrind(
        &mut self,
        program_path: &str,
        index: usize,
        use_stderr: bool,
    ) {
        use std::time::SystemTime;
        let beggining = SystemTime::now();
        //println!("THREAD {} RUN", self.get_name());
        if self.run_valgrind(index, program_path) && self.run_diff(index, use_stderr) {
            self.passed = true;
        }

        let stdout = format!("rtest_stdout{}", index);
        let stderr = format!("rtest_stderr{}", index);
        let _ = fs::remove_file(stdout);
        let _ = fs::remove_file(stderr);

        self.time = SystemTime::now()
            .duration_since(beggining)
            .unwrap_or_else(|_| panic!("ERROR: Time_calculation of {:?} FAILED.", self.test_path))
            .as_secs_f32();
    }

    /**
    Conducts a test process without using valgrind
    */
    pub(super) fn test_no_valgrind(&mut self, program_path: &str, index: usize, use_stderr: bool) {
        use std::time::SystemTime;
        let beggining = SystemTime::now();
        //println!("THREAD {} RUN", self.get_name());

        if self.run_program(index, program_path) && self.run_diff(index, use_stderr) {
            self.passed = true;
        }

        let stdout = format!("rtest_stdout{}", index);
        let stderr = format!("rtest_stderr{}", index);
        let _ = fs::remove_file(stdout);
        let _ = fs::remove_file(stderr);

        self.time = SystemTime::now()
            .duration_since(beggining)
            .unwrap_or_else(|_| panic!("ERROR: Time calculation of {:?} FAILED.", self.test_path))
            .as_secs_f32();
    }

    /**
    Runs diff on program output. Index indicate test index
    */
    fn run_diff(&mut self, index: usize, use_stderr: bool) -> bool {
        let stdout_input = format!("rtest_stdout{}", index);
        let stderr_input = format!("rtest_stderr{}", index);
        let stdout_output = self.get_stdout_file();
        let stderr_output = self.get_stderr_file();

        let stdout_result = TestResult::diff_files(&stdout_input, &stdout_output);

        match stdout_result {
            DiffResult::Ok => {}
            DiffResult::DifferenceNotSpecified(error) => {
                self.failed_cause = TestFail::Diff(DiffResult::DifferenceStdout(error));
                return false;
            }
            other => {
                self.failed_cause = TestFail::Diff(other);
                return false;
            }
        }

        if use_stderr {
            let stderr_result = TestResult::diff_files(&stderr_input, &stderr_output);

            match stderr_result {
                DiffResult::Ok => true,
                DiffResult::DifferenceNotSpecified(error) => {
                    self.failed_cause = TestFail::Diff(DiffResult::DifferenceStderr(error));
                    false
                }
                _ => {
                    panic!("UNABLE ARM OF RUN DIFF REACHED!! (STDERR)");
                }
            }
        } else {
            true
        }
    }

    /// Returns 'title' of problem which has occured while testing
    pub fn get_problem_description(&self) -> String {
        match &self.failed_cause {
            TestFail::ProgramExitCode() => "SYSTEM: Program EXITCODE read failed!".to_string(),
            TestFail::ValgrindExitCode() => "SYSTEM: Valgrind EXITCODE read failed!".to_string(),
            TestFail::CompilationExitCode => "SYSTEM: Gcc EXITCODE read failed!".to_string(),
            TestFail::Valgrind(_) => "Valgrind ERROR".to_string(),
            TestFail::Compilation(_) => "Compilation ERROR".to_string(),
            TestFail::Diff(diff_error) => match diff_error {
                DiffResult::DifferenceNotSpecified(_) => {
                    "Diff ERROR: Difference (not specified)".to_string()
                }
                DiffResult::DifferenceStderr(_) => "Diff ERROR: Difference (stderr)".to_string(),
                DiffResult::DifferenceStdout(_) => "Diff ERROR: Difference (stdout)".to_string(),
                DiffResult::InnerProblem(_) => "Diff ERROR: InnerProblem".to_string(),
                DiffResult::Trouble(_) => "Diff ERROR: Trouble".to_string(),
                _ => "PROGRAM UNDEFINED DIFF ERROR".to_string(),
            },
            TestFail::InnerProblem(_) => "PROGRAM INNER PROBLEM".to_string(),
        }
    }

    /// Returns a problem description of failed test
    pub fn get_problem(&self) -> &str {
        self.failed_cause.get_problem()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_stdout_test() {
        let ts = TestResult {
            test_path: "/usr/bin/a/b/c/def.in".to_string(),
            passed: false,
            time: 0.0,
            failed_cause: TestFail::InnerProblem("".to_string()),
            return_code: 1,
            compilation_warnings: None,
        };

        assert!(ts.get_stdout_file() == "/usr/bin/a/b/c/def.out");
    }

    #[test]
    fn get_stderr_test() {
        let ts = TestResult {
            test_path: "/usr/bin/a/b/c/def.in".to_string(),
            passed: false,
            time: 0.0,
            failed_cause: TestFail::InnerProblem("".to_string()),
            return_code: 1,
            compilation_warnings: None,
        };

        assert!(ts.get_stderr_file() == "/usr/bin/a/b/c/def.err");
    }

    #[test]
    fn get_test_path_test() {
        let ts = TestResult {
            test_path: "/usr/bin/a/b/c/def.in".to_string(),
            passed: false,
            time: 0.0,
            failed_cause: TestFail::InnerProblem("".to_string()),
            return_code: 1,
            compilation_warnings: None,
        };

        assert!(ts.get_name() == "def.in");
    }
}
