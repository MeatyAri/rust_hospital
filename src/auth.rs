use std::io;
use serde::{Serialize, Deserialize};
use std::cmp::{Ord, Ordering};

use crate::data_structures::bst::UniqueAttribute;
use crate::db_handler::Database;
use crate::cli_handler::MenuHandler;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Patient,
    Doctor,
    Pharmacist,
    TriageSupervisor,
    EmergencyDoctor,
    Admin,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    password: String,
    full_name: String,
    ssn: String,
    age: u32,
    role: Role,
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.username.cmp(&other.username)
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}

impl Eq for User {}

impl UniqueAttribute for User {
    fn uattr(&self) -> String {
        self.username.clone()
    }
}

impl User {
    pub fn new(username: String, password: String, full_name: String, ssn: String, age: u32, role: Role) -> Self {
        User {
            username,
            password,
            full_name,
            ssn,
            age,
            role,
        }
    }
}

pub struct Auth<'a> {
    db: &'a mut Database,
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
        match self.db.get(username.clone()) {
            Some(user) => {
                if user.password == password {
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
    }

    pub fn register(&mut self, username: String, password: String, full_name: String, ssn: String, age: u32, role: Role) -> io::Result<User> {
        let user = User::new(username, password, full_name, ssn, age, role);
        self.db.insert(user.clone())?;
        self.db.save_to_file("users.db").unwrap();
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
                let username = MenuHandler::get_input_string("Enter your username".to_string());
                let password = MenuHandler::get_input_string("Enter your password".to_string());
                if self.login(username, password) {
                    println!("Login successful");
                } else {
                    println!("Login failed");
                }
            }
            "Sign Up" => {
                println!("Sign Up");
                loop {
                    let username = MenuHandler::get_input_string("Enter a username".to_string());
                    let password = MenuHandler::get_input_string("Enter a password".to_string());
                    let full_name = MenuHandler::get_input_string("Enter your full name".to_string());
                    let ssn = MenuHandler::get_input_string("Enter your ssn".to_string());
                    
                    let age = MenuHandler::get_input_string("Enter your age".to_string());
                    let age: u32 = age.parse().unwrap();

                    let options = [
                        "Patient".to_string(),
                        "Doctor".to_string(),
                        "Pharmacist".to_string(),
                        "TriageSupervisor".to_string(),
                        "EmergencyDoctor".to_string(),
                        "Admin".to_string(),
                    ];
                    let role_menu = MenuHandler::new("Select your role:".to_string(), &options);
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
//     // db.save_to_file("users.db")?;

//     // Load from file
//     let loaded_db = Database::load_from_file("users.db")?;
//     if let Some(ref data) = loaded_db.data {
//         println!("{:?}", data);
//     }
//     if let Some(user) = loaded_db.get("user2".to_string()) {
//         println!("Loaded User: {:?}", user);
//     }

//     Ok(())
// }
