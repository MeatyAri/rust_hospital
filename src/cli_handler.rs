use std::io;

use crate::auth::Auth;

pub struct MenuHandler<'a> {
    pub query: String,
    pub options: &'a [String],
}

impl<'a> MenuHandler<'a> {
    pub fn new(query: String, options: &'a [String]) -> MenuHandler<'a> {
        MenuHandler { query, options }
    }

    pub fn clear_terminal() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn get_selected_option(&self) -> String {
        loop {
            println!("{}", self.query);
            for (i, option) in self.options.iter().enumerate() {
                println!("{}: {}", i + 1, option);
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().parse::<usize>();
            match input {
                Ok(n) if n > 0 && n <= self.options.len() => {
                    return self.options[n - 1].clone();
                }
                _ => println!("Invalid input"),
            }
        }
    }

    pub fn get_input_string(query: String) -> String {
        println!("{}", query);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn run(&self) -> String {
        if self.options.is_empty() {
            Self::get_input_string(self.query.clone())
        } else {
            self.get_selected_option()
        }
    }
}

pub fn main_menu() -> String {
    let options = ["Login".to_string(), "Sign Up".to_string(), "Exit".to_string()];
    let menu = MenuHandler::new("What would you like to do?".to_string(), &options);
    let selected = menu.run();
    selected
}

pub fn patient_menu(auth: &mut Auth) {
    let options = ["Make an appointment".to_string(), "Cancel an appointment".to_string(), "My Account".to_string(), "Logout".to_string()];
    let menu = MenuHandler::new("What would you like to do?".to_string(), &options);
    let selected = menu.run();

    match selected.as_str() {
        "Make an appointment" => println!("Make an appointment"),
        "Cancel an appointment" => println!("Cancel an appointment"),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn doctor_menu(auth: &mut Auth) {
    let options = ["Visit Patient".to_string(), "My Account".to_string(), "Logout".to_string()];
    let menu = MenuHandler::new("What would you like to do?".to_string(), &options);
    let selected = menu.run();

    match selected.as_str() {
        "Visit Patient" => println!("Visit Patient"),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn pharmacist_menu(auth: &mut Auth) {
    let options = ["Dispense patient medications".to_string(), "My Account".to_string(), "Logout".to_string()];
    let menu = MenuHandler::new("What would you like to do?".to_string(), &options);
    let selected = menu.run();

    match selected.as_str() {
        "Dispense patient medications" => println!("Dispense patient medications"),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn triage_supervisor_menu(auth: &mut Auth) {
    let options = ["Assign patients to doctors".to_string(), "My Account".to_string(), "Logout".to_string()];
    let menu = MenuHandler::new("What would you like to do?".to_string(), &options);
    let selected = menu.run();

    match selected.as_str() {
        "Assign patients to doctors" => println!("Assign patients to doctors"),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn emergency_doctor_menu(auth: &mut Auth) {
    let options = ["Visit Triage patients".to_string(), "My Account".to_string(), "Logout".to_string()];
    let menu = MenuHandler::new("What would you like to do?".to_string(), &options);
    let selected = menu.run();

    match selected.as_str() {
        "Visit Triage patients" => println!("Visit Triage patients"),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn admin_menu(auth: &mut Auth) {
    let options = [
        "Register a new user".to_string(),
        "Delete a user".to_string(),
        "Search for a user".to_string(),
        "View all users".to_string(),
        "My Account".to_string(),
        "Logout".to_string(),
    ];
    let menu = MenuHandler::new("What would you like to do?".to_string(), &options);
    let selected = menu.run();

    match selected.as_str() {
        "Register a new user" => println!("Register a new user"),
        "Delete a user" => println!("Delete a user"),
        "Search for a user" => println!("Search for a user"),
        "View all users" => println!("View all users"),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}
