use std::io::{self, Write};
use crate::settings::Options;
use prettytable::{Table, Row, Cell, row, Attr, color};
use crate::testing::{self, TestResult};

fn clear_console() {
    print!("{}[2J", 27 as char);
}

fn print_title() {
    println!("     __   __    __  ___  ____  ___  ___  ____ 
    / _) /  \\  /  \\(   \\(_  _)(  _)/ __)(_  _)
   ( (/\\( () )( () )) ) ) )(   ) _)\\__ \\  )(  
    \\__/ \\__/  \\__/(___/ (__) (___)(___/ (__)");
}

fn print_menu() {
    print_title();
    println!("            1. Rozpocznij testy");
    println!("            2. Wyświetl ustawienia");
    println!("            3. Wyjdź");
    print!("\n\n\t Wybierz opcję i potwierdź ENTERem: ");
    let _ = io::stdout().flush();
}

pub fn invoke(settings: &Options) {
    clear_console();
    print_menu();
    let mut choice: u8;
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Błąd wczytywania. Spróbuj ponownie.");
            print!("Wybierz opcję i potwierdź ENTERem: ");
            let _ = io::stdout().flush();
        } else if let Ok(parsed) = input.trim().parse() {
            choice = parsed;

            if (1..=3).contains(&choice) {
                break;
            } else {
                println!("Błąd wczytywania. Podaj prawidłową cyfrę.");
                print!("Wybierz opcję i potwierdź ENTERem: ");
                let _ = io::stdout().flush();
            }
        } else {
            println!("Błąd wczytywania. Podaj prawidłową liczbę.");
            print!("Wybierz opcję i potwierdź ENTERem: ");
            let _ = io::stdout().flush();
        }
        
        input.clear();
    }

    match choice {
        1 => {
            let results = testing::run_testing(settings);
            print_results(&results);
        }
        _ => panic!("NIE")
    }
}

fn truncate(text: &str) -> String {
    let mut result = String::new();
    let mut char_number = 0;
    for character in text.chars() {
        if character == '\n' {
            char_number = 1;
        } else {
            char_number += 1;
        }

        if char_number%200 == 0 {
            result.push('\n');
            result.push(character);
        } else {
            result.push(character);
        }
    }

    result
}

fn print_results(results: &Vec<TestResult>) {
    let mut show_result = Table::new();

    show_result.add_row(row!["ID", "NAME", "TIME", "PASSED", "PROBLEM"]);
    for (index, result) in results.iter().enumerate() {
        let id = index.to_string();
        let name = result.get_name();
        if result.passed() {
            let mut time = result.get_time().to_string();
            time.push_str(" s");
            show_result.add_row(Row::new(vec![
                Cell::new(&id),
                Cell::new(&name),
                Cell::new(&time),
                Cell::new("TRUE")
                    .with_style(Attr::ForegroundColor(color::GREEN)),
                Cell::new("-")
            ]));
        } else {
            let mut description = result.get_problem_description();
            description.push_str(": ");
            let problem = result.get_problem();
            description.push_str(&truncate(&problem));
            show_result.add_row(Row::new(vec![
                Cell::new(&id),
                Cell::new(&name),
                Cell::new("-"),
                Cell::new("FALSE")
                    .with_style(Attr::ForegroundColor(color::RED)),
                Cell::new(&description)
            ]));
        }
    }

    clear_console();
    show_result.printstd();
}
