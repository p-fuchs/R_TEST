use std::io::{self, Write};

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

pub fn invoke() {
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
}

