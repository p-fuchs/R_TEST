pub trait LiteralGenerator {
    fn get_literal<'input>(&'input self, identificator: &str) -> &'input str;
    fn language_format_good(&self, language_format: &str) -> bool;
}
/// Metoda która przyjmuje Konfigurację i zwraca odpowiednią Instancję
pub mod language_dictionary;
