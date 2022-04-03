use std::collections::HashMap;
use std::fs;
use serde_json;

const LANG_DIR: &str = "./lang/";

pub struct LangDict {
    database: HashMap<String, String>
}

impl super::LiteralGenerator for LangDict {
    fn get_literal<'input>(&'input self, identificator: &str) -> &'input str {
        match self.database.get(identificator) {
            None => "NO TRANSLATION YET.",
            Some(string) => string
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
            Ok(s) => s
        };
        
        let map: HashMap<String, String> = match serde_json::from_str(&content) {
            Err(_) => {
                eprintln!("ERROR: Translation file is wrong. ID: {}", language_id);
                return LangDict::handle_wrong(language_id);
            }
            Ok(map) => map
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
        let content = serde_json::to_string_pretty(&self.database)
            .unwrap();
        
        println!("PATH OF SERIALIZATION {}", path);
        let _ = fs::write(path, content);
    }
}

impl Default for LangDict {
    fn default() -> Self {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("WARNING_ABSOLUTE_PATH".to_string(), "Warning! Please use paths of absolute formats!".to_string());
        map.insert("TEST_FOLDER_PATH".to_string(), "tak".to_string());
        map.insert("MAIN_TITLE".to_string(), "          __   __    __  ___  ____  ___  ___  ____ 
        / _) /  \\  /  \\(   \\(_  _)(  _)/ __)(_  _)
       ( (/\\( () )( () )) ) ) )(   ) _)\\__ \\  )(  
        \\__/ \\__/  \\__/(___/ (__) (___)(___/ (__)".to_string());
        LangDict { database: map }
    }
}