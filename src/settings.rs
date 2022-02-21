use std::path;
use std::fs;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Options {
    test_root_directory: String,
    program_executable_path: String,
    is_valgrind_active: bool
}

impl Options {
    fn default() -> Options {
        Options {
            test_root_directory: ".".to_string(),
            program_executable_path: ".".to_string(),
            is_valgrind_active: true
        }
    }

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

    pub fn save(self) {
        let content = serde_json::to_string_pretty(&self)
            .expect("ERROR: Creating JSON description of options FAILED.");
        let _ = fs::write("config.json", &content);
    }
}