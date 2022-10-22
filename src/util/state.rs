use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct State {
    pub current_commit_hash: String,
    pub current_branch: String,
}

impl State {
    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }

    pub fn deserialize(object: Vec<u8>) -> Self {
        serde_json::from_slice(object.as_slice()).unwrap()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_commit_hash: "".to_owned(),
            current_branch: "main".to_owned(),
        }
    }
}

impl State {
    pub fn new(current_commit_hash: String) -> Self {
        Self {
            current_commit_hash,
            current_branch: "main".to_owned(),
        }
    }
}