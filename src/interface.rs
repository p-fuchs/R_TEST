use std::io::{self, Write};
use std::fs::{File};
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

fn print_options(settings: &Options) {
    println!("                    _ (`-.  .-') _                            .-') _   .-')    
               ( (OO  )(  OO) )                          ( OO ) ) ( OO ).  
 .-'),-----.  _.`     \\/     '._ ,-.-')  .-'),-----. ,--./ ,--,' (_)---\\_) 
( OO'  .-.  '(__...--''|'--...__)|  |OO)( OO'  .-.  '|   \\ |  |\\ /    _ |  
/   |  | |  | |  /  | |'--.  .--'|  |  \\/   |  | |  ||    \\|  | )\\  :` `.  
\\_) |  |\\|  | |  |_.' |   |  |   |  |(_/\\_) |  |\\|  ||  .     |/  '..`''.) 
  \\ |  | |  | |  .___.'   |  |  ,|  |_.'  \\ |  | |  ||  |\\    |  .-._)   \\ 
   `'  '-'  ' |  |        |  | (_|  |      `'  '-'  '|  | \\   |  \\       / 
     `-----'  `--'        `--'   `--'        `-----' `--'  `--'   `-----'");
    
    println!("        1)    Test directory: {}", settings.get_test_path());
    println!("        2)      Program path: {}", settings.get_program_path());
    println!("        3) Valgrind activity: {}", settings.get_valgrind_activity());
    println!("        4)   Stderr activity: {}", settings.get_stderr_option());
    println!("        WARNING: Please use ABSOLUTE PATHS!\n");
    print!("        Wybierz opcję (5 powoduje powrót): ");
    let _ = io::stdout().flush();
}

fn print_menu() {
    print_title();
    println!("            1. Rozpocznij testy");
    println!("            2. Wyświetl ustawienia");
    println!("            3. Wyjdź");
    print!("\n\n\t Wybierz opcję i potwierdź ENTERem: ");
    let _ = io::stdout().flush();
}

/**
Drive options menu
*/
fn manage_options(settings: &mut Options) {
    clear_console();
    print_options(settings);
    let choice = read_input(4);

    match choice {
        1 => {
            manage_test_dir(settings);
            manage_options(settings);
        },
        2 => {
            manage_program_path(settings);
            manage_options(settings);
        },
        3 => {
            manage_valgrind_activity(settings);
            manage_options(settings);
        },
        4 => {
            manage_stderr_activity(settings);
            manage_options(settings);
        }
        _ => {}
    }
}

fn manage_test_dir(settings: &mut Options) {
    loop {
        print!("Podaj scieżkę do folderu z testami: ");
        let _ = io::stdout().flush();
        let mut path = String::new();

        io::stdin()
            .read_line(&mut path)
            .expect("IO ERROR");
        
        let path_trimmed = path.trim();
        if settings.set_test_path(path_trimmed){
            break;
        } else {
            println!("Podano nieprawidłową ścieżkę!");
        }
    }
}

fn manage_program_path(settings: &mut Options) {
    loop {
        print!("Podaj scieżkę do skompilowanego programu: ");
        let _ = io::stdout().flush();
        let mut path = String::new();

        io::stdin()
            .read_line(&mut path)
            .expect("IO ERROR");
        
        let path_trimmed = path.trim();
        if settings.set_program_path(path_trimmed){
            break;
        } else {
            println!("Podano nieprawidłową ścieżkę!");
        }
    }
}

fn read_bool_stdin() -> Option<bool> {
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("IO ERROR");
        
    let option_trimmed = option.trim().to_ascii_lowercase();
    let option_cleaned = option_trimmed.as_str();

    match option_cleaned {
        "true" => {
            Some(true)
        },
        "false" => {
            Some(false)
        },
        _ => {
            None
        }
    }
}

fn manage_valgrind_activity(settings: &mut Options) {
    loop {
        print!("Użycie valgrinda: (true/false) ");
        let _ = io::stdout().flush();

        match read_bool_stdin() {
            Some(option) => {
                settings.set_valgrind_activity(option);
                break;
            }
            None => {
                println!("Podano nieprawidłową wartość!");
            }
        }
    }
}

fn manage_stderr_activity(settings: &mut Options) {
    loop {
        print!("Użycie testów stderr: (true/false) ");
        let _ = io::stdout().flush();

        match read_bool_stdin() {
            Some(option) => {
                settings.set_stderr_usage(option);
                break;
            }
            None => {
                println!("Podano nieprawidłową wartość!");
            }
        }
    }
}

/// Reads a positive number from stdandard input, then checks wheter the number is
/// less or equal to maximum_number and returns it.
fn read_input(maximum_number: u8) -> u8{
    let mut choice: u8;
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Błąd wczytywania. Spróbuj ponownie.");
            print!("Wybierz opcję i potwierdź ENTERem: ");
            let _ = io::stdout().flush();
        } else if let Ok(parsed) = input.trim().parse() {
            choice = parsed;

            if (1..=maximum_number).contains(&choice) {
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
    choice
}

/// Starts a whole program interface
pub fn start_program(settings: &mut Options) {
    clear_console();
    print_menu();
    let choice = read_input(3);

    match choice {
        1 => {
            let results = testing::run_testing(settings);
            print_results(&results);
        }
        3 => {
            clear_console();
            println!("Program kończy swoje działanie.");
        }
        _ => {
            manage_options(settings);
            start_program(settings);
        }
    }
}

/**
Add a endline char every 200 characters without newline.
*/
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

/**
Prints a table with results of tests and saves it to a history.log file in
main directory
*/
fn print_table(results: &[TestResult]){
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
            description.push_str(": \n");
            let problem = result.get_problem();
            description.push_str(&truncate(problem));
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
    show_result.printstd();
    let content = show_result.to_string();
    let mut output = File::create("history.log").unwrap();
    let _ = write!(output, "{}", content);
}

/**
Prints out summary of conducted tests
*/
fn print_summary(results: &[TestResult]) {
    let mut passed = 0;
    let mut valgrind_failed = 0;
    let mut diff_failed = 0;
    let mut other_failed = 0;
    
    for result in results {
        if result.passed() {
            passed += 1;
        } else if result.diff_error() {
            diff_failed += 1;
        } else if result.valgrind_error() {
            valgrind_failed += 1;
        } else {
            other_failed += 1;
        }
        
    }

    let mut summary = Table::new();
    summary.add_row(Row::new(vec![
        Cell::new("TOTAL")
            .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
        Cell::new(&(passed + valgrind_failed + other_failed + diff_failed).to_string())
    ]));

    summary.add_row(Row::new(vec![
        Cell::new("PASSED")
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new(&passed.to_string())
    ]));

    summary.add_row(Row::new(vec![
        Cell::new("FAILED")
            .with_style(Attr::ForegroundColor(color::RED)),
        Cell::new(&(valgrind_failed + other_failed + diff_failed).to_string())
    ]));


    summary.add_row(Row::new(vec![
        Cell::new("VALGRIND FAILED")
            .with_style(Attr::ForegroundColor(color::RED)),
        Cell::new(&valgrind_failed.to_string())
    ]));

    summary.add_row(Row::new(vec![
        Cell::new("DIFF FAILED")
            .with_style(Attr::ForegroundColor(color::RED)),
        Cell::new(&diff_failed.to_string())
    ]));

    summary.add_row(Row::new(vec![
        Cell::new("OTHER FAIL")
            .with_style(Attr::ForegroundColor(color::RED)),
        Cell::new(&other_failed.to_string())
    ]));

    summary.printstd();
}

/**
Prints summary and table of conducted tests.
*/
fn print_results(results: &[TestResult]) {
    clear_console();
    print_table(results);
    println!();
    print_summary(results);
}
