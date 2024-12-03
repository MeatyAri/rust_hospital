use std::io;

use crate::data_structures::priority_queue::PriorityQueue;
use crate::db::db_handler::Database;
use crate::cli_handler::{clear_terminal, get_input_string, MenuHandler};
use crate::db::entities::{DoctorsList, Role, User};


pub struct Auth<'a> {
    pub db: &'a mut Database,
    pub user: Option<User>,
}

impl<'a> Auth<'a> {
    pub fn new(db: &'a mut Database) -> Self {
        Auth {
            db,
            user: None,
        }
    }

    pub fn login(&mut self, username: String, password: String) -> bool {
        match self.db.get_user(username.clone()) {
            Some(user) => {
                if user.verify_password(password) {
                    self.user = Some(user.clone());
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub fn logout(&mut self) {
        self.user = None;
        clear_terminal();
    }

    pub fn register(&mut self, username: String, password: String, full_name: String, ssn: String, age: u32, role: Role) -> io::Result<User> {
        let user = User::new(username, password, full_name, ssn, age, role);
        self.db.insert_user(user.clone())?;

        if user.role == Role::Doctor || user.role == Role::EmergencyDoctor{
            self.db.insert_doctors_list(DoctorsList { doctor: user.username.clone(), patients: PriorityQueue::new() })?;
        }

        self.db.commit().unwrap();
        Ok(user)
    }

    pub fn signup(&mut self, username: String, password: String, full_name: String, ssn: String, age: u32, role: Role) -> io::Result<()> {
        let user = self.register(username, password, full_name, ssn, age, role)?;
        self.user = Some(user);
        Ok(())
    }

    pub fn authenticate(&mut self, method: String) {
        match method.as_str() {
            "Login" => {
                println!("Login");
                let username = get_input_string("Enter your username".to_string());
                let password = get_input_string("Enter your password".to_string());
                if self.login(username.clone(), password) {
                    clear_terminal();
                    println!("Logged in as: {:?}", username);
                } else {
                    println!("Login failed");
                }
            }
            "Sign Up" => {
                println!("Sign Up");
                loop {
                    let username = get_input_string("Enter a username".to_string());
                    let password = get_input_string("Enter a password".to_string());
                    let full_name = get_input_string("Enter your full name".to_string());
                    let ssn = get_input_string("Enter your ssn".to_string());
                    
                    let age = get_input_string("Enter your age".to_string());
                    let age: u32 = age.parse().unwrap();

                    let options = ["Patient", "Doctor", "Pharmacist", "TriageSupervisor", "EmergencyDoctor", "Admin"];
                    let role_menu = MenuHandler::new("Select your role:".to_string(), options.into_iter());
                    let role = role_menu.run();
                    let role = match role.as_str() {
                        "Patient" => Role::Patient,
                        "Doctor" => Role::Doctor,
                        "Pharmacist" => Role::Pharmacist,
                        "TriageSupervisor" => Role::TriageSupervisor,
                        "EmergencyDoctor" => Role::EmergencyDoctor,
                        "Admin" => Role::Admin,
                        _ => panic!("Invalid role"),
                    };
                    
                    match self.signup(username, password, full_name, ssn, age, role) {
                        Ok(_) => {
                            println!("Sign up successful");
                            break;
                        }
                        Err(e) => println!("Sign up failed: {}", e),
                    }
                }
            }
            "Exit" => {
                println!("Exiting");
                std::process::exit(0);
            }
            _ => panic!("Invalid option"),
        }
    }
}


// fn main() -> io::Result<()> {
//     let mut db = Database::new();
//     let mut auth = Auth::new(&mut db);
//     auth.register("user1".to_string(), "password1".to_string(), "John Doe".to_string(), "123-45-6789".to_string(), 30, Role::Patient);
//     auth.signup("user2".to_string(), "password2".to_string(), "Jane Smith".to_string(), "987-65-4321".to_string(), 25, Role::Patient);
//     auth.register("user3".to_string(), "password3".to_string(), "Bob Johnson".to_string(), "456-78-9012".to_string(), 40, Role::Patient);
//     println!("Logged in as: {:?}", auth.user);
//     auth.logout();
//     println!("Logged out");

//     // // Insert a few users
//     // db.insert(User::new("user1".to_string(), "password1".to_string(), "John Doe".to_string(), "123-45-6789".to_string(), 30, Role::Patient));
//     // db.insert(User::new("user2".to_string(), "password2".to_string(), "Jane Smith".to_string(), "987-65-4321".to_string(), 25, Role::Patient));
//     // db.insert(User::new("user3".to_string(), "password3".to_string(), "Bob Johnson".to_string(), "456-78-9012".to_string(), 40, Role::Patient));

//     // // Save to file
//     // db.commit()?;

//     // Load from file
//     let loaded_db = Database::load_from_file("database.db")?;
//     if let Some(ref data) = loaded_db.data {
//         println!("{:?}", data);
//     }
//     if let Some(user) = loaded_db.get("user2".to_string()) {
//         println!("Loaded User: {:?}", user);
//     }

//     Ok(())
// }
