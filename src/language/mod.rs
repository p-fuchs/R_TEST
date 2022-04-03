pub trait LiteralGenerator {
    fn get_literal<'input>(&'input self, identificator: &str) -> &'input str;
    fn language_format_good(&self, language_format: &str) -> bool;
}

pub mod language_dictionary;