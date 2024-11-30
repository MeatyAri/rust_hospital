mod cli_handler;
mod auth;
mod db_handler;
mod data_structures;

use cli_handler::{admin_menu, doctor_menu, emergency_doctor_menu, patient_menu, pharmacist_menu, triage_supervisor_menu, MenuHandler};
use db_handler::Database;
use auth::{Auth, Role};

fn main() {
    let mut db = Database::load_from_file("users.db").unwrap_or(Database::new());
    let mut auth = Auth::new(&mut db);

    loop {
        if let Some(ref user) = auth.user {
            match user.role {
                Role::Patient => patient_menu(&mut auth),
                Role::Doctor => doctor_menu(&mut auth),
                Role::Pharmacist => pharmacist_menu(&mut auth),
                Role::TriageSupervisor => triage_supervisor_menu(&mut auth),
                Role::EmergencyDoctor => emergency_doctor_menu(&mut auth),
                Role::Admin => admin_menu(&mut auth),
            }
        } else {
            let selected = cli_handler::main_menu();
            auth.authenticate(selected);
        
            MenuHandler::clear_terminal();
            println!("Logged in as: {:?}", auth.user);
        }
    }
}
