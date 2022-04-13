use crate::language::LiteralGenerator;
use crate::settings::Options;
use crate::testing::{self, TestResult};
use prettytable::{color, row, Attr, Cell, Row, Table};
use std::fs::File;
use std::io::{self, Write};

fn clear_console() {
    print!("{}[2J", 27 as char);
}

fn print_title<T: LiteralGenerator>(lang: &T) {
    println!("{}", lang.get_literal("MAIN_TITLE"));
    /*println!("     __   __    __  ___  ____  ___  ___  ____
     / _) /  \\  /  \\(   \\(_  _)(  _)/ __)(_  _)
    ( (/\\( () )( () )) ) ) )(   ) _)\\__ \\  )(
     \\__/ \\__/  \\__/(___/ (__) (___)(___/ (__)");*/
}

fn print_options<T: LiteralGenerator>(settings: &Options, lang: &T) {
    /*println!("                    _ (`-.  .-') _                            .-') _   .-')
                   ( (OO  )(  OO) )                          ( OO ) ) ( OO ).
     .-'),-----.  _.`     \\/     '._ ,-.-')  .-'),-----. ,--./ ,--,' (_)---\\_)
    ( OO'  .-.  '(__...--''|'--...__)|  |OO)( OO'  .-.  '|   \\ |  |\\ /    _ |
    /   |  | |  | |  /  | |'--.  .--'|  |  \\/   |  | |  ||    \\|  | )\\  :` `.
    \\_) |  |\\|  | |  |_.' |   |  |   |  |(_/\\_) |  |\\|  ||  .     |/  '..`''.)
      \\ |  | |  | |  .___.'   |  |  ,|  |_.'  \\ |  | |  ||  |\\    |  .-._)   \\
       `'  '-'  ' |  |        |  | (_|  |      `'  '-'  '|  | \\   |  \\       /
         `-----'  `--'        `--'   `--'        `-----' `--'  `--'   `-----'");*/
    println!("{}", lang.get_literal("OPTIONS_TITLE"));

    println!(
        "        {} {}",
        lang.get_literal("TEST_FOLDER_PATH"),
        settings.get_test_path()
    );
    println!(
        "        {} {}",
        lang.get_literal("PROGRAM_PATH"),
        settings.get_program_path()
    );
    println!(
        "        {} {}",
        lang.get_literal("VALGRIND_ACTIVITY"),
        settings.get_valgrind_activity()
    );
    println!(
        "        {} {}",
        lang.get_literal("STDERR_ACTIVITY"),
        settings.get_stderr_option()
    );
    println!(
        "        {} {}",
        lang.get_literal("LANGUAGE_OPTION"),
        settings.get_language()
    );
    println!(
        "        {} {}",
        lang.get_literal("PROGRAM_MODE"),
        settings.get_program_mode()
    );
    println!("        {}", lang.get_literal("WARNING_ABSOLUTE_PATH"));
    println!("        {}", lang.get_literal("WARNING_LANGUAGE"));
    println!("        {}", lang.get_literal("WARNING_COMPILATION"));
    print!("        {} ", lang.get_literal("CHOOSE_OPTION_RETURN7"));
    let _ = io::stdout().flush();
}

fn print_menu<T: LiteralGenerator>(lang: &T) {
    print_title(lang);
    /*println!("            1. Rozpocznij testy");
    println!("            2. Wyświetl ustawienia");
    println!("            3. Wyjdź");*/
    println!("            1. {}", lang.get_literal("START_TESTS"));
    println!("            2. {}", lang.get_literal("SHOW_SETTINGS"));
    println!("            3. {}", lang.get_literal("EXIT_PROGRAM"));
    print!("\n\n\t {} ", lang.get_literal("CHOOSE_OPTION_ENTER"));
    let _ = io::stdout().flush();
}

/**
Drive options menu
*/
fn manage_options<T: LiteralGenerator>(settings: &mut Options, lang: &T) {
    clear_console();
    print_options(settings, lang);
    let choice = read_input(7, lang);

    match choice {
        1 => {
            manage_test_dir(settings, lang);
            manage_options(settings, lang);
        }
        2 => {
            manage_program_path(settings, lang);
            manage_options(settings, lang);
        }
        3 => {
            manage_valgrind_activity(settings, lang);
            manage_options(settings, lang);
        }
        4 => {
            manage_stderr_activity(settings, lang);
            manage_options(settings, lang);
        }
        5 => {
            manage_language(settings, lang);
            manage_options(settings, lang);
        }
        6 => {
            manage_compilation_mode(settings, lang);
            manage_options(settings, lang);
        }
        _ => {}
    }
}

fn manage_compilation_mode<T: LiteralGenerator>(settings: &mut Options, lang: &T) {
    loop {
        print!("{} ", lang.get_literal("COMPILATION_USAGE"));
        let _ = io::stdout().flush();

        match read_bool_stdin() {
            Some(option) => {
                settings.set_compilation_mode(option);
                break;
            }
            None => {
                println!("{}", lang.get_literal("INCORRECT_VALUE"));
            }
        }
    }
}

fn manage_language<T: LiteralGenerator>(settings: &mut Options, lang: &T) {
    print!("{} ", lang.get_literal("GET_LANGUAGE"));
    let _ = io::stdout().flush();
    let mut language = String::new();

    io::stdin().read_line(&mut language).expect("IO ERROR");

    if lang.language_format_good(language.trim()) {
        settings.set_language(language.trim());
    } else {
        eprintln!("Language not avilable yet!");
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
fn manage_test_dir<T: LiteralGenerator>(settings: &mut Options, lang: &T) {
    loop {
        print!("{} ", lang.get_literal("GET_TEST_PATH"));
        let _ = io::stdout().flush();
        let mut path = String::new();

        io::stdin().read_line(&mut path).expect("IO ERROR");

        let path_trimmed = path.trim();
        if settings.set_test_path(path_trimmed) {
            break;
        } else {
            println!("{}", lang.get_literal("INCORRECT_PATH"));
        }
    }
}

fn manage_program_path<T: LiteralGenerator>(settings: &mut Options, lang: &T) {
    loop {
        print!("{} ", lang.get_literal("GET_PROGRAM_PATH"));
        let _ = io::stdout().flush();
        let mut path = String::new();

        io::stdin().read_line(&mut path).expect("IO ERROR");

        let path_trimmed = path.trim();
        if settings.set_program_path(path_trimmed) {
            break;
        } else {
            println!("{}", lang.get_literal("INCORRECT_PATH"));
        }
    }
}

fn read_bool_stdin() -> Option<bool> {
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("IO ERROR");

    let option_trimmed = option.trim().to_ascii_lowercase();
    let option_cleaned = option_trimmed.as_str();

    match option_cleaned {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn manage_valgrind_activity<T: LiteralGenerator>(settings: &mut Options, lang: &T) {
    loop {
        print!("{} ", lang.get_literal("VALGRIND_USAGE"));
        let _ = io::stdout().flush();

        match read_bool_stdin() {
            Some(option) => {
                settings.set_valgrind_activity(option);
                break;
            }
            None => {
                println!("{}", lang.get_literal("INCORRECT_VALUE"));
            }
        }
    }
}

fn manage_stderr_activity<T: LiteralGenerator>(settings: &mut Options, lang: &T) {
    loop {
        print!("{} ", lang.get_literal("STDERR_USAGE"));
        let _ = io::stdout().flush();

        match read_bool_stdin() {
            Some(option) => {
                settings.set_stderr_usage(option);
                break;
            }
            None => {
                println!("{}", lang.get_literal("INCORRECT_VALUE"));
            }
        }
    }
}

/// Reads a positive number from stdandard input, then checks wheter the number is
/// less or equal to maximum_number and returns it.
fn read_input<T: LiteralGenerator>(maximum_number: u8, lang: &T) -> u8 {
    let mut choice: u8;
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", lang.get_literal("READ_ERROR"));
            print!("{} ", lang.get_literal("CHOOSE_OPTION_ENTER"));
            let _ = io::stdout().flush();
        } else if let Ok(parsed) = input.trim().parse() {
            choice = parsed;

            if (1..=maximum_number).contains(&choice) {
                break;
            } else {
                println!("{}", lang.get_literal("READ_ERROR_DIGIT"));
                print!("{} ", lang.get_literal("CHOOSE_OPTION_ENTER"));
                let _ = io::stdout().flush();
            }
        } else {
            println!("{}", lang.get_literal("READ_ERROR_NUMBER"));
            print!("{} ", lang.get_literal("CHOOSE_OPTION_ENTER"));
            let _ = io::stdout().flush();
        }

        input.clear();
    }
    choice
}

/// Starts a whole program interface
pub fn start_program<T: LiteralGenerator>(settings: &mut Options, dict: &T) {
    clear_console();
    print_menu(dict);
    let choice = read_input(3, dict);

    match choice {
        1 => {
            let results = testing::run_testing(settings);
            print_results(&results, dict);
        }
        3 => {
            clear_console();
            println!("{}", dict.get_literal("PROGRAM_END"));
        }
        _ => {
            manage_options(settings, dict);
            start_program(settings, dict);
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

        if char_number % 200 == 0 {
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
fn print_table<T: LiteralGenerator>(results: &[TestResult], lang: &T) {
    let mut show_result = Table::new();
    let id = lang.get_literal("RESULT_ID");
    let name = lang.get_literal("RESULT_NAME");
    let time = lang.get_literal("RESULT_TIME");
    let passed = lang.get_literal("RESULT_PASSED");
    let overall = lang.get_literal("RESULT_OUTCOME");

    let test_true = lang.get_literal("RESULT_TRUE_OUT");
    let test_false = lang.get_literal("RESULT_FALSE_OUT");

    show_result.add_row(row![id, name, time, passed, overall]);
    for (index, result) in results.iter().enumerate() {
        let id = (index + 1).to_string();
        let name = result.get_name();
        let code = format!(
            "{}: | {} |",
            lang.get_literal("RESULT_EXITCODE"),
            result.get_exit_code()
        );

        if result.passed() {
            let mut time = result.get_time().to_string();
            time.push_str(" s");
            show_result.add_row(Row::new(vec![
                Cell::new(&id),
                Cell::new(&name).with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
                Cell::new(&time).with_style(Attr::ForegroundColor(color::YELLOW)),
                Cell::new(test_true).with_style(Attr::ForegroundColor(color::GREEN)),
                Cell::new(&code),
            ]));
        } else {
            let mut description = result.get_problem_description();
            description.push_str(": \n");
            let problem = result.get_problem();
            description.push_str(&truncate(problem));
            show_result.add_row(Row::new(vec![
                Cell::new(&id),
                Cell::new(&name).with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
                Cell::new("-"),
                Cell::new(test_false).with_style(Attr::ForegroundColor(color::RED)),
                Cell::new(&description),
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
fn print_summary<T: LiteralGenerator>(results: &[TestResult], lang: &T) {
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
        Cell::new(lang.get_literal("TEST_TOTAL"))
            .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
        Cell::new(&(passed + valgrind_failed + other_failed + diff_failed).to_string()),
    ]));

    summary.add_row(Row::new(vec![
        Cell::new(lang.get_literal("TEST_PASSED")).with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new(&passed.to_string()),
    ]));

    summary.add_row(Row::new(vec![
        Cell::new(lang.get_literal("TEST_FAILED")).with_style(Attr::ForegroundColor(color::RED)),
        Cell::new(&(valgrind_failed + other_failed + diff_failed).to_string()),
    ]));

    summary.add_row(Row::new(vec![
        Cell::new(lang.get_literal("TEST_VALGRIND_FAILED"))
            .with_style(Attr::ForegroundColor(color::RED)),
        Cell::new(&valgrind_failed.to_string()),
    ]));

    summary.add_row(Row::new(vec![
        Cell::new(lang.get_literal("TEST_DIFF_FAILED"))
            .with_style(Attr::ForegroundColor(color::RED)),
        Cell::new(&diff_failed.to_string()),
    ]));

    summary.add_row(Row::new(vec![
        Cell::new(lang.get_literal("TEST_OTHER_FAILED"))
            .with_style(Attr::ForegroundColor(color::RED)),
        Cell::new(&other_failed.to_string()),
    ]));

    summary.printstd();
}

/**
Prints summary and table of conducted tests.
*/
fn print_results<T: LiteralGenerator>(results: &[TestResult], lang: &T) {
    clear_console();
    print_table(results, lang);
    println!();
    print_summary(results, lang);
}
