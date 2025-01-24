use std::fs::File;
use std::io::{self, Write, Read, Error, ErrorKind};
use bincode;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;

use crate::data_structures::map::Graph;
use crate::db::entities::User;
use crate::data_structures::bst::TreeNode;
use crate::data_structures::linked_list::LinkedList;

use super::entities::{Clinic, DoctorsList, Prescription, Drug, DrugGP};


#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub users_data: Option<TreeNode<User>>,
    pub clinics_data: Option<LinkedList<Clinic>>,
    pub doctors_data: Option<LinkedList<DoctorsList>>,
    pub prescriptions_data: Option<LinkedList<Prescription>>,
    pub drugs_data: Option<Box<TreeNode<Drug>>>,
    pub drug_gps: Option<LinkedList<DrugGP>>,
    pub map: Graph,
}

impl Database {
    pub fn new() -> Self {
        Database {
            users_data: None,
            clinics_data: None,
            doctors_data: None,
            prescriptions_data: None,
            drugs_data: None,
            drug_gps: None,
            map: Graph::new(),
        }
    }

    pub fn insert_user(&mut self, user: User) -> io::Result<()> {
        match self.users_data {
            Some(ref mut data) => {
                if data.get_by_uniq_attr(user.username.clone()).is_some() {
                    return Err(Error::new(ErrorKind::AlreadyExists, "Username already exists"));
                }
                data.insert(user);
                Ok(())
            },
            None => {
                self.users_data = Some(TreeNode::new(user));
                Ok(())
            },
        }
    }

    pub fn insert_clinic(&mut self, clinic: Clinic) -> io::Result<()> {
        match self.clinics_data {
            Some(ref mut data) => {
                if data.get_by_uniq_attr(clinic.name.clone()).is_some() {
                    return Err(Error::new(ErrorKind::AlreadyExists, "Clinic name already exists"));
                }
                data.insert(clinic);
                Ok(())
            },
            None => {
                self.clinics_data = Some(LinkedList::new());
                self.clinics_data.as_mut().unwrap().insert(clinic);
                Ok(())
            },
        }
    }

    pub fn insert_doctors_list(&mut self, doctors_list: DoctorsList) -> io::Result<()> {
        match self.doctors_data {
            Some(ref mut data) => {
                if data.get_by_uniq_attr(doctors_list.doctor.clone()).is_some() {
                    return Err(Error::new(ErrorKind::AlreadyExists, "Doctors list already exists"));
                }
                data.insert(doctors_list);
                Ok(())
            },
            None => {
                self.doctors_data = Some(LinkedList::new());
                self.doctors_data.as_mut().unwrap().insert(doctors_list);
                Ok(())
            },
        }
    }

    pub fn insert_prescription(&mut self, prescription: Prescription) -> io::Result<()> {
        match self.prescriptions_data {
            Some(ref mut data) => {
                data.insert(prescription);
                Ok(())
            },
            None => {
                self.prescriptions_data = Some(LinkedList::new());
                self.prescriptions_data.as_mut().unwrap().insert(prescription);
                Ok(())
            },
        }
    }

    pub fn insert_drug(&mut self, drug: Drug) -> io::Result<()> {
        match self.drugs_data {
            Some(ref mut data) => {
                if data.get_drug_by_id(drug.id).is_some() || data.get_drug_by_name(drug.name.clone()).is_some() {
                    return Err(Error::new(ErrorKind::AlreadyExists, "Drug with the same id or name already exists"));
                }
                data.insert(drug);
                data.balance();
                Ok(())
            },
            None => {
                self.drugs_data = Some(Box::new(TreeNode::new(drug)));
                Ok(())
            },
        }
    }

    pub fn insert_drug_gp(&mut self, drug_gp: DrugGP) -> io::Result<()> {
        match self.drug_gps {
            Some(ref mut data) => {
                if data.get_by_uniq_attr(drug_gp.name.clone()).is_some() {
                    return Err(Error::new(ErrorKind::AlreadyExists, "Drug group already exists"));
                }
                data.insert(drug_gp);
                Ok(())
            },
            None => {
                self.drug_gps = Some(LinkedList::new());
                self.drug_gps.as_mut().unwrap().insert(drug_gp);
                Ok(())
            },
        }
    }

    pub fn get_user(&self, uniq_attr: String) -> Option<&User> {
        match self.users_data {
            Some(ref data) => data.get_by_uniq_attr(uniq_attr),
            None => None,
        }
    }

    pub fn get_doctors_list(&mut self, uniq_attr: String) -> Option<&mut DoctorsList> {
        match self.doctors_data {
            Some(ref mut data) => data.get_by_uniq_attr(uniq_attr),
            None => None,
        }
    }

    pub fn get_clinic(&mut self, uniq_attr: String) -> Option<&mut Clinic> {
        match self.clinics_data {
            Some(ref mut data) => data.get_by_uniq_attr(uniq_attr),
            None => None,
        }
    }

    pub fn get_prescription(&mut self, uniq_attr: String) -> Option<&mut Prescription> {
        match self.prescriptions_data {
            Some(ref mut data) => data.get_by_uniq_attr(uniq_attr),
            None => None,
        }
    }

    pub fn get_drug_by_id(&mut self, id: u32) -> Option<&mut Drug> {
        match self.drugs_data {
            Some(ref mut data) => data.get_drug_by_id_mut(id),
            None => None,
        }
    }

    pub fn get_drug_by_name(&mut self, name: String) -> Option<&mut Drug> {
        match self.drugs_data {
            Some(ref mut data) => data.get_drug_by_name_mut(name),
            None => None,
        }
    }

    pub fn get_drug_gp(&mut self, uniq_attr: String) -> Option<&mut DrugGP> {
        match self.drug_gps {
            Some(ref mut data) => data.get_by_uniq_attr(uniq_attr),
            None => None,
        }
    }

    pub fn remove_prescription(&mut self, uniq_attr: String) -> bool {
        match self.prescriptions_data {
            Some(ref mut data) => data.remove_by_uniq_attr(uniq_attr),
            None => false,
        }
    }

    pub fn remove_drug(&mut self, id: u32) {
        if let Some(ref mut _data) = self.drugs_data {
            self.drugs_data = TreeNode::remove_drug_by_id(self.drugs_data.take(), id);
        }
    }

    pub fn remove_drug_gp(&mut self, uniq_attr: String) -> bool {
        match self.drug_gps {
            Some(ref mut data) => data.remove_by_uniq_attr(uniq_attr),
            None => false,
        }
    }

    pub fn commit(&mut self) -> io::Result<()> {
        self.save_to_file("database.bin")
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let encoded = bincode::serialize(self).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let database: Database = bincode::deserialize(&buffer).unwrap();
        Ok(database)
    }
}
