use serde_json;
use std::collections::HashMap;
use std::fs;

const LANG_DIR: &str = "./lang/";

const ENG_LANG: &str = r#"{
    "MAIN_TITLE": "         __   __    __  ___  ____  ___  ___  ____ \n        / _) /  \\  /  \\(   \\(_  _)(  _)/ __)(_  _)\n       ( (/\\( () )( () )) ) ) )(   ) _)\\__ \\  )(  \n        \\__/ \\__/  \\__/(___/ (__) (___)(___/ (__)",
    "OPTIONS_TITLE": "                    _ (`-.  .-') _                            .-') _   .-')    \n               ( (OO  )(  OO) )                          ( OO ) ) ( OO ).  \n .-'),-----.  _.`     \\/     '._ ,-.-')  .-'),-----. ,--./ ,--,' (_)---\\_) \n( OO'  .-.  '(__...--''|'--...__)|  |OO)( OO'  .-.  '|   \\ |  |\\ /    _ |  \n/   |  | |  | |  /  | |'--.  .--'|  |  \\/   |  | |  ||    \\|  | )\\  :` `.  \n\\_) |  |\\|  | |  |_.' |   |  |   |  |(_/\\_) |  |\\|  ||  .     |/  '..`''.) \n  \\ |  | |  | |  .___.'   |  |  ,|  |_.'  \\ |  | |  ||  |\\    |  .-._)   \\ \n   `'  '-'  ' |  |        |  | (_|  |      `'  '-'  '|  | \\   |  \\       / \n     `-----'  `--'        `--'   `--'        `-----' `--'  `--'   `-----'",
    "TEST_FOLDER_PATH":       "1)    Test directory:",
    "PROGRAM_PATH":           "2)      Program path:",
    "VALGRIND_ACTIVITY":      "3) Valgrind activity:",
    "STDERR_ACTIVITY":        "4)   Stderr activity:",
    "LANGUAGE_OPTION":        "5)          Language:",
    "WARNING_ABSOLUTE_PATH":  "Warning 1: Please use paths of absolute formats!",
    "WARNING_LANGUAGE":       "Warning 2: Language will change after program restart.",
    "CHOOSE_OPTION_RETURN6":  "Choose option (6 exits options menu):",
    "START_TESTS":            "Start testing",
    "SHOW_SETTINGS":          "Program settings",
    "EXIT_PROGRAM":           "Exit program",
    "CHOOSE_OPTION_ENTER":    "Choose an option and press ENTER:",
    "GET_LANGUAGE":           "Enter a language:",
    "GET_TEST_PATH":          "Enter a path to folder with tests:",
    "INCORRECT_PATH":         "Entered path was incorrect!",
    "GET_PROGRAM_PATH":       "Enter a path to compiled program:",
    "VALGRIND_USAGE":         "Valgrind usage: (true / false)",
    "INCORRECT_VALUE":        "Entered value was incorrect!",
    "STDERR_USAGE":           "Testing of stderr on: (true / false)",
    "READ_ERROR":             "Reading input error. Try again.",
    "READ_ERROR_DIGIT":       "Reading input error. Entered digit was incorrect.",
    "READ_ERROR_NUMBER":      "Reading input error. Entered number was incorrect.",
    "PROGRAM_END":            "Program is terminating.",
    "RESULT_ID":              "ID",
    "RESULT_NAME":            "NAME",
    "RESULT_TIME":            "TIME",
    "RESULT_PASSED":          "PASSED",
    "RESULT_OUTCOME":         "PROBLEM / EXITCODE",
    "RESULT_TRUE_OUT":        "TRUE",
    "RESULT_FALSE_OUT":       "FALSE",
    "RESULT_EXITCODE":        "Program's returned exitcode",
    "TEST_TOTAL":             "TOTAL",
    "TEST_PASSED":            "PASSED",
    "TEST_FAILED":            "FAILED",
    "TEST_VALGRIND_FAILED":   "VALGRIND FAILED",
    "TEST_DIFF_FAILED":       "DIFF FAILED",
    "TEST_OTHER_FAILED":      "OTHER FAIL"
  }"#;

pub struct LangDict {
    database: HashMap<String, String>,
}

impl super::LiteralGenerator for LangDict {
    fn get_literal<'input>(&'input self, identificator: &str) -> &'input str {
        match self.database.get(identificator) {
            None => "NO TRANSLATION YET.",
            Some(string) => string,
        }
    }

    fn language_format_good(&self, language_format: &str) -> bool {
        matches!(language_format, "EN_en" | "PL_pl")
    }
}

impl LangDict {
    pub fn new(language_id: &str) -> Self {
        let mut path = String::new();
        path.push_str(LANG_DIR);
        path.push_str(language_id);
        path.push_str(".lang");

        let content = match fs::read_to_string(&path) {
            Err(_) => {
                eprintln!("ERROR: Translation file not found. ID: {}", language_id);
                return LangDict::handle_wrong(language_id);
            }
            Ok(s) => s,
        };

        let map: HashMap<String, String> = match serde_json::from_str(&content) {
            Err(_) => {
                eprintln!("ERROR: Translation file is wrong. ID: {}", language_id);
                return LangDict::handle_wrong(language_id);
            }
            Ok(map) => map,
        };

        LangDict { database: map }
    }

    fn handle_wrong(language_id: &str) -> Self {
        let res = LangDict::default();
        res.serialize(language_id);

        res
    }

    pub fn serialize(&self, language_id: &str) {
        let path = format!("{}{}.lang", LANG_DIR, language_id);
        let content = serde_json::to_string_pretty(&self.database).unwrap();

        println!("PATH OF SERIALIZATION {}", path);
        let _ = fs::write(path, content);
    }
}

impl Default for LangDict {
    fn default() -> Self {
        let map = serde_json::from_str(ENG_LANG).unwrap();
        LangDict { database: map }
    }
}
