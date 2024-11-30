use std::fs::File;
use std::io::{self, Write, Read, Error, ErrorKind};
use bincode;

use crate::auth::User;
use crate::data_structures::bst::TreeNode;


pub struct Database {
    pub users_data: Option<TreeNode<User>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            users_data: None,
        }
    }

    pub fn insert(&mut self, user: User) -> io::Result<()> {
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

    pub fn get(&self, uniq_attr: String) -> Option<&User> {
        match self.users_data {
            Some(ref data) => data.get_by_uniq_attr(uniq_attr),
            None => None,
        }
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let encoded: Vec<u8> = bincode::serialize(&self.users_data).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let data: Option<TreeNode<User>> = bincode::deserialize(&buffer).unwrap();
        Ok(Database { users_data: data })
    }
}
