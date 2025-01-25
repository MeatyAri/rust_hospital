use serde::{Serialize, Deserialize};
use std::cmp::{Ord, Ordering};
use std::fmt::Debug;

use crate::data_structures::linked_list::LinkedList;
use crate::data_structures::priority_queue::PriorityQueue;
use crate::data_structures::stack::Stack;
use crate::sha_hasher::Sha256;
use hex;


pub trait UniqueAttribute {
    fn uattr(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    pub full_name: String,
    pub ssn: String,
    pub age: u32,
    pub role: Role,
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

    pub fn verify_password(&self, password: String) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let password = hex::encode(hasher.finalize());
        
        self.password == password
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Clinic {
    pub name: String,
    pub doctors: LinkedList<String>,
}

impl UniqueAttribute for Clinic {
    fn uattr(&self) -> String {
        self.name.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoctorsList {
    pub doctor: String,
    pub patients: PriorityQueue<Patient>,
}

impl UniqueAttribute for DoctorsList {
    fn uattr(&self) -> String {
        self.doctor.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Patient {
    pub name: String,
    pub priority: u32,
}

impl UniqueAttribute for Patient {
    fn uattr(&self) -> String {
        self.name.clone()
    }
}

impl Ord for Patient {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Patient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Patient {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Patient {}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prescription {
    pub patient_name: String,
    pub medications: Stack<String>,
}

impl UniqueAttribute for Prescription {
    fn uattr(&self) -> String {
        self.patient_name.clone()
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Drug {
    pub id: u32,
    pub name: String,
    pub price: f32,
    pub quantity: u32,
}

impl Drug {
    pub fn new(id: u32, name: String, price: f32, quantity: u32) -> Self {
        Drug {
            id,
            name,
            price,
            quantity,
        }
    }
}

impl Ord for Drug {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Drug {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Drug {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Drug {}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DrugGP {
    pub name: String,
    pub drugs: LinkedList<u32>,
}

impl UniqueAttribute for DrugGP {
    fn uattr(&self) -> String {
        self.name.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ambulance {
    pub name: String,
    pub hospital: String,
    pub location: String,
}

impl Ambulance {
    pub fn new(name: String, hospital: String, location: String) -> Self {
        Ambulance {
            name,
            hospital,
            location,
        }
    }
}

impl UniqueAttribute for Ambulance {
    fn uattr(&self) -> String {
        self.name.clone()
    }
}
