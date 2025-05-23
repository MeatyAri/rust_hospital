use std::io;

use crate::auth::Auth;
use crate::menus_logic::{
    add_drug,
    add_drug_to_gp,
    assign_patients,
    cancel_appointment,
    create_drug_gp,
    dispense_medications,
    display_all_drugs,
    make_appointment,
    remove_drug,
    remove_drug_gp,
    search_drugs,
    visit_patients_wrapper,
    display_all_drug_gps,
    show_search_complexity,
    add_location,
    remove_location,
    print_map,
    add_ambulance,
    remove_ambulance,
    move_ambulance,
    send_ambulance_to_patient,
    list_ambulances,
    print_logs,
};


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
    clear_terminal();

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
    clear_terminal();

    match selected.as_str() {
        "Visit Patients" => visit_patients_wrapper(auth),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn pharmacist_menu(auth: &mut Auth) {
    let options = [
        "Dispense patient medications",
        "Add Drug",
        "Remove Drug",
        "Search Drugs",
        "Show Search Complexity",
        "Display All Drugs",
        "Display Drug Groups",
        "Drug Groups Management",
        "My Account",
        "Logout"
    ];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
    let selected = menu.run();
    clear_terminal();

    match selected.as_str() {
        "Dispense patient medications" => dispense_medications(auth),
        "Add Drug" => add_drug(auth),
        "Remove Drug" => remove_drug(auth),
        "Search Drugs" => search_drugs(auth),
        "Show Search Complexity" => show_search_complexity(auth),
        "Display All Drugs" => display_all_drugs(auth),
        "Display Drug Groups" => display_all_drug_gps(auth),
        "Drug Groups Management" => drug_groups_menu(auth),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn drug_groups_menu(auth: &mut Auth) {
    let options = ["Create Drug Group", "Add Drug to Group", "Remove Drug from Group", "back"];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
    let selected = menu.run();
    clear_terminal();

    match selected.as_str() {
        "Create Drug Group" => create_drug_gp(auth),
        "Add Drug to Group" => add_drug_to_gp(auth),
        "Remove Drug from Group" => remove_drug_gp(auth),
        "back" => pharmacist_menu(auth),
        _ => println!("Invalid option"),
    }
}

pub fn triage_supervisor_menu(auth: &mut Auth) {
    let options = ["Assign patients to doctors", "My Account", "Logout"];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
    let selected = menu.run();
    clear_terminal();

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
    clear_terminal();

    match selected.as_str() {
        "Visit Triage patients" => visit_patients_wrapper(auth),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

pub fn admin_menu(auth: &mut Auth) {
    let options = ["Register a new user", "Delete a user", "Search for a user", "View all users", "Map & Ambulances", "My Account", "Logout"];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
    let selected = menu.run();
    clear_terminal();

    match selected.as_str() {
        "Register a new user" => println!("Register a new user"),
        "Delete a user" => println!("Delete a user"),
        "Search for a user" => println!("Search for a user"),
        "View all users" => println!("View all users"),
        "Map & Ambulances" => map_ambulances_menu(auth),
        "My Account" => println!("My Account"),
        "Logout" => auth.logout(),
        _ => println!("Invalid option"),
    }
}

fn map_ambulances_menu(auth: &mut Auth) {
    let options = [
        "Add Location",
        "Remove Location",
        "Print Map",
        "Add Ambulance",
        "Remove Ambulance",
        "Move Ambulance",
        "List Ambulances",
        "Send Ambulance to Patient",
        "History",
        "back"
    ];
    let menu = MenuHandler::new("What would you like to do?".to_string(), options.into_iter());
    let selected = menu.run();
    clear_terminal();

    match selected.as_str() {
        "Add Location" => add_location(auth),
        "Remove Location" => remove_location(auth),
        "Print Map" => print_map(auth),
        "Add Ambulance" => add_ambulance(auth),
        "Remove Ambulance" => remove_ambulance(auth),
        "Move Ambulance" => move_ambulance(auth),
        "List Ambulances" => list_ambulances(auth),
        "Send Ambulance to Patient" => send_ambulance_to_patient(auth),
        "History" => print_logs(auth),
        "back" => admin_menu(auth),
        _ => println!("Invalid option"),
    }
}
