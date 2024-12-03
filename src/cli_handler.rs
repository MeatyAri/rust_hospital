use std::io;

use crate::auth::Auth;
use crate::menus_logic::{assign_patients, cancel_appointment, dispense_medications, make_appointment, visit_patients, visit_patients_wrapper};


pub struct MenuHandler<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    pub query: String,
    pub options: I,
}

impl<'a, I> MenuHandler<'a, I>
where
    I: Iterator<Item = &'a str> + Clone,
{
    pub fn new(query: String, options: I) -> MenuHandler<'a, I> {
        MenuHandler { query, options }
    }

    fn get_selected_option(&self) -> String {
        loop {
            println!("{}", self.query);
            for (i, option) in self.options.clone().enumerate() {
                println!("{}: {}", i + 1, option);
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().parse::<usize>();
            match input {
                Ok(n) if n > 0 && n <= self.options.clone().count() => {
                    return self
                        .options
                        .clone()
                        .nth(n - 1)
                        .expect("option not found")
                        .to_string();
                }
                _ => println!("Invalid input"),
            }
        }
    }

    pub fn run(&self) -> String {
        self.get_selected_option()
    }
}

pub fn get_input_string(query: String) -> String {
    println!("{}", query);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

// ### menus ###

pub fn main_menu() -> String {
    let options = ["Login", "Sign Up", "Exit"];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
    let selected = menu.run();
    selected
}

pub fn patient_menu(auth: &mut Auth) {
    let options = ["Make an appointment", "Cancel an appointment", "My Account", "Logout"];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
    let selected = menu.run();

    match selected.as_str() {
        "Make an appointment" => make_appointment(auth),
        "Cancel an appointment" => cancel_appointment(auth),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn doctor_menu(auth: &mut Auth) {
    let options = ["Visit Patients", "My Account", "Logout"];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
    let selected = menu.run();

    match selected.as_str() {
        "Visit Patients" => visit_patients_wrapper(auth),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn pharmacist_menu(auth: &mut Auth) {
    let options = ["Dispense patient medications", "My Account", "Logout"];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
    let selected = menu.run();

    match selected.as_str() {
        "Dispense patient medications" => dispense_medications(auth),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn triage_supervisor_menu(auth: &mut Auth) {
    let options = ["Assign patients to doctors", "My Account", "Logout"];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
    let selected = menu.run();

    match selected.as_str() {
        "Assign patients to doctors" => assign_patients(auth),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn emergency_doctor_menu(auth: &mut Auth) {
    let options = ["Visit Triage patients", "My Account", "Logout"];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
    let selected = menu.run();

    match selected.as_str() {
        "Visit Triage patients" => visit_patients_wrapper(auth),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn admin_menu(auth: &mut Auth) {
    let options = ["Register a new user", "Delete a user", "Search for a user", "View all users", "My Account", "Logout"];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
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
