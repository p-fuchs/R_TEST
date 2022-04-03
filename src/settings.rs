use std::path;
use std::fs;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
/// Structure to manage program configuration
pub struct Options {
    test_root_directory: String,
    program_executable_path: String,
    is_valgrind_active: bool,
    use_stderr_tests: bool,
    language: String
}

impl Default for Options {
    /**
    Default structure of settings.
    */
    fn default() -> Options {
        Options {
            test_root_directory: "testy/".to_string(),
            program_executable_path: "PLEASE, SET!".to_string(),
            is_valgrind_active: true,
            use_stderr_tests: false,
            language: "EN_en".to_string()
        }
    }
}

impl Options {
    /// Return absolute path to folder with tests
    pub fn get_test_path(&self) -> &str {
        &self.test_root_directory
    }

    /// Return actual language
    pub fn get_language(&self) -> &str {
        &self.language
    }

    /// Returns wheater to use valgrind or not (true - use)
    pub fn get_valgrind_activity(&self) -> bool {
        self.is_valgrind_active
    }

    /// Returns wheater to use stderr tests or not (true - use)
    pub fn get_stderr_option(&self) -> bool {
        self.use_stderr_tests
    }

    /// Function checkes wheter path points to a directory, if so, it sets
    /// path of test folder accordingly and return true, otherwise it return false
    pub fn set_test_path(&mut self, path: &str) -> bool {
        use std::path::Path;
        if Path::new(path).is_dir() {
            self.test_root_directory = path.to_string();
            true
        } else {
            false
        }
    }

    /// Function checkes wheter path points to a file, if so, it sets
    /// path of program main file accordingly and return true, otherwise it return false
    pub fn set_program_path(&mut self, path: &str) -> bool {
        use std::path::Path;
        if Path::new(path).is_file() {
            self.program_executable_path = path.to_string();
            true
        } else {
            false
        }
    }

    /// Sets wheter to use valgrind in testing process (true - use)
    pub fn set_valgrind_activity(&mut self, option: bool) {
        self.is_valgrind_active = option;
    }

    /// Sets wheter to use stderr tests in testing process (true - use)
    pub fn set_stderr_usage(&mut self, option: bool) {
        self.use_stderr_tests = option;
    }

    pub fn set_language(&mut self, lang: &str) {
        self.language = lang.to_string();
    }

    /// Returns &str which representes program executable absolute path
    pub fn get_program_path(&self) -> &str {
        &self.program_executable_path
    }

    /// If correct config.json exists in main folder, it generates an Options structure
    /// which is saved in that file. Otherwise it generate default structure.
    pub fn new() -> Options {
        if path::Path::new("config.json").exists() {
            let content = fs::read_to_string("config.json")
                .expect("ERROR: Opening folder with settings FAILED.");
            
            let preferences = serde_json::from_str(&content);
            if preferences.is_err() {
                return Options::default();
            }

            preferences.unwrap()
        } else {
            Options::default()
        }
    }

    /// Function saves config structure to config.json file
    pub fn save(self) {
        let content = serde_json::to_string_pretty(&self)
            .expect("ERROR: Creating JSON description of options FAILED.");
        let _ = fs::write("config.json", &content);
    }
}