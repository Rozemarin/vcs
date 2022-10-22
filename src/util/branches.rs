use std::collections::HashMap;

use serde::{Serialize, Deserialize};
// format:
// "branch list" {
//     "main": {
//          commit1, commitN,
//      } 
// }   S

#[derive(Debug, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Branches {
    pub branch_list: HashMap<String, (String, String)>,
}

impl Branches {
    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }

    pub fn deserialize(object: Vec<u8>) -> Self {
        serde_json::from_slice(object.as_slice()).unwrap()
    }
}

impl Default for Branches {
    fn default() -> Self {
        let mut branch_list = HashMap::new();
        branch_list.insert("main".to_owned(), ("".to_owned(), "".to_owned()));
        Self {
            branch_list,
        }
    }
}

impl Branches {
    pub fn new(commit_hash: String) -> Self {
        let mut branch_list = HashMap::new();
        branch_list.insert("main".to_owned(), (commit_hash.clone(), commit_hash.clone()));
        Self {
            branch_list,
        }
    }

    // pub fn add_new_branch(branch_name: Branches,
}