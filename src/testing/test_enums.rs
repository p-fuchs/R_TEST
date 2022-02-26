/// Results which can occur while using diff
#[derive(Debug)]
pub enum DiffResult {
    Ok,
    DifferenceNotSpecified(String),
    DifferenceStderr(String),
    DifferenceStdout(String),
    Trouble(String),
    InnerProblem(String)
}
/// Possible causes of failing tests
#[derive(Debug)]
pub enum TestFail {
    Valgrind(String),
    Diff(DiffResult),
    InnerProblem(String),
    ProgramExitCode(String)
}

impl TestFail {

    /**
    Gets a problem description
    */
    pub fn get_problem(&self) -> &str {
        match self {
            TestFail::Valgrind(err) => err,
            TestFail::InnerProblem(err) => err,
            TestFail::ProgramExitCode(err) => err,
            TestFail::Diff(diff_error) => {
                match diff_error {
                    DiffResult::DifferenceStderr(err) => err,
                    DiffResult::DifferenceStdout(err) => err,
                    DiffResult::InnerProblem(err) => err,
                    DiffResult::Trouble(err) => err,
                    _ => "UNDEFINED BEHAVIOUR OF GET_PROBLEM FUNCTION"
                }
            }
        }
    }
}