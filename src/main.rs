mod cli_handler;
mod auth;
mod db;
mod data_structures;
mod menus_logic;

use cli_handler::{admin_menu, doctor_menu, emergency_doctor_menu, patient_menu, pharmacist_menu, triage_supervisor_menu, MenuHandler};
use data_structures::{linked_list::LinkedList, priority_queue::PriorityQueue};
use db::{db_handler::Database, entities::{Clinic, DoctorsList, User}};
use auth::Auth;
use db::entities::Role;


fn test_data(auth: &mut Auth) {
    // Insert two patients for testing
    auth.signup("patient1".to_string(), "password1".to_string(), "John Doe".to_string(), "123-45-6789".to_string(), 30, Role::Patient).unwrap();
    auth.logout();
    // auth.register("patient2".to_string(), "password2".to_string(), "Jane Smith".to_string(), "987-65-4321".to_string(), 25, Role::Patient).unwrap();

    // Insert two doctors for testing
    auth.signup("doc1".to_string(), "password1".to_string(), "Dr. John Doe".to_string(), "123-45-6789".to_string(), 30, Role::Doctor).unwrap();
    auth.logout();
    // auth.register("doc2".to_string(), "password2".to_string(), "Dr. Jane Smith".to_string(), "987-65-4321".to_string(), 25, Role::Doctor).unwrap();

    // insert a pharmacist for testing
    auth.signup("pharmacist1".to_string(), "password1".to_string(), "Dr. John Doe".to_string(), "123-45-6789".to_string(), 30, Role::Pharmacist).unwrap();
    auth.logout();

    // Insert two clinics for testing
    let mut doctors1 = LinkedList::new();
    doctors1.insert("doc1".to_string());
    // let mut doctors2 = LinkedList::new();
    // doctors2.insert("doc2".to_string());
    auth.db.insert_clinic(Clinic { name: "Clinic A".to_string(), doctors: doctors1}).unwrap();
    // auth.db.insert_clinic(Clinic { name: "Clinic B".to_string(), doctors: doctors2 }).unwrap();

    auth.db.commit().unwrap();
}

fn main() {
    let mut db = Database::load_from_file("database.bin").unwrap_or(Database::new());
    let mut auth = Auth::new(&mut db);

    // test_data(&mut auth);
    println!("{:?}", auth.db); // for debugging

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
        }
    }
}
