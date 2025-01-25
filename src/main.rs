mod cli_handler;
mod auth;
mod db;
mod data_structures;
mod menus_logic;

use cli_handler::{admin_menu, doctor_menu, emergency_doctor_menu, patient_menu, pharmacist_menu, triage_supervisor_menu};
use data_structures::{linked_list::LinkedList, map::{LocationType, Object}};
use db::{db_handler::Database, entities::{Ambulance, Clinic, Drug, DrugGP, Role}};
use auth::Auth;


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

    // insert a triage supervisor for testing
    auth.signup("tir1".to_string(), "password1".to_string(), "Dr. John Doe".to_string(), "123-45-6789".to_string(), 30, Role::TriageSupervisor).unwrap();
    auth.logout();

    // insert an emergency doctor for testing
    auth.signup("emdoc1".to_string(), "password1".to_string(), "Dr. John Doe".to_string(), "123-45-6789".to_string(), 30, Role::EmergencyDoctor).unwrap();
    auth.logout();

    // insert an admin for testing
    auth.signup("admin1".to_string(), "password1".to_string(), "Dr. John Doe".to_string(), "123-45-6789".to_string(), 30, Role::Admin).unwrap();
    auth.logout();

    // Insert two clinics for testing
    let mut doctors1 = LinkedList::new();
    doctors1.insert("doc1".to_string());
    doctors1.insert("emdoc1".to_string());
    // let mut doctors2 = LinkedList::new();
    // doctors2.insert("doc2".to_string());
    auth.db.insert_clinic(Clinic { name: "Clinic A".to_string(), doctors: doctors1}).unwrap();
    // auth.db.insert_clinic(Clinic { name: "Clinic B".to_string(), doctors: doctors2 }).unwrap();

    // Insert some drugs for testing
    auth.db.insert_drug(Drug::new(0, "Aspirin".to_string(), 32.99, 50)).unwrap();
    auth.db.insert_drug(Drug::new(1, "Ibuprofen".to_string(), 12.99, 100)).unwrap();
    auth.db.insert_drug(Drug::new(2, "Paracetamol".to_string(), 9.99, 200)).unwrap();
    auth.db.insert_drug(Drug::new(3, "Amoxicillin".to_string(), 19.99, 30)).unwrap();
    auth.db.insert_drug(Drug::new(4, "Azithromycin".to_string(), 29.99, 20)).unwrap();
    auth.db.insert_drug(Drug::new(5, "Ciprofloxacin".to_string(), 39.99, 10)).unwrap();

    // Insert some drug groups for testing
    let mut drugs1 = LinkedList::new();
    drugs1.insert(0);
    drugs1.insert(1);
    drugs1.insert(2);
    auth.db.insert_drug_gp(DrugGP { name: "Painkiller".to_string(), drugs: drugs1 }).unwrap();

    let mut drugs2 = LinkedList::new();
    drugs2.insert(3);
    drugs2.insert(4);
    auth.db.insert_drug_gp(DrugGP { name: "Antibiotics".to_string(), drugs: drugs2 }).unwrap();

    // insert some locations for testing
    auth.db.map.add_node("Hospital A".to_string(), LocationType::Hospital);
    auth.db.map.add_node("Hospital B".to_string(), LocationType::Hospital);
    auth.db.map.add_node("Home A".to_string(), LocationType::Home);
    auth.db.map.add_node("Home B".to_string(), LocationType::Home);
    auth.db.map.add_node("Other A".to_string(), LocationType::Other);
    auth.db.map.add_node("Other B".to_string(), LocationType::Other);

    // insert some edges for testing
    auth.db.map.add_edge("Hospital A".to_string(), "Hospital B".to_string());
    auth.db.map.add_edge("Hospital A".to_string(), "Home A".to_string());
    auth.db.map.add_edge("Hospital A".to_string(), "Other A".to_string());
    auth.db.map.add_edge("Hospital B".to_string(), "Home B".to_string());
    auth.db.map.add_edge("Hospital B".to_string(), "Other B".to_string());
    auth.db.map.add_edge("Home A".to_string(), "Home B".to_string());
    auth.db.map.add_edge("Home A".to_string(), "Other A".to_string());
    auth.db.map.add_edge("Home B".to_string(), "Other B".to_string());
    auth.db.map.add_edge("Other B".to_string(), "Home B".to_string());

    // insert some ambulances for testing
    auth.db.insert_ambulance(Ambulance { name: "Ambulance A".to_string(), hospital: "Hospital A".to_string(), location: "Hospital A".to_string() }).unwrap();
    auth.db.map.add_object_to_node("Hospital A", Object { name: "Ambulance A".to_string() });
    auth.db.insert_ambulance(Ambulance { name: "Ambulance B".to_string(), hospital: "Hospital B".to_string(), location: "Hospital B".to_string() }).unwrap();
    auth.db.map.add_object_to_node("Hospital B", Object { name: "Ambulance B".to_string() });
    auth.db.insert_ambulance(Ambulance { name: "Ambulance C".to_string(), hospital: "Hospital A".to_string(), location: "Other B".to_string() }).unwrap();
    auth.db.map.add_object_to_node("Other B", Object { name: "Ambulance C".to_string() });
    
    auth.db.commit().unwrap();
}

fn main() {
    let mut db = Database::load_from_file("database.bin").unwrap_or(Database::new());
    let mut auth = Auth::new(&mut db);

    // test_data(&mut auth); // for testing
    // println!("{:?}", auth.db); // for debugging

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
