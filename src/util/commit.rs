#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use crate::util::hasher;
use crate::util::hasher::hasher;
use std::path::PathBuf;

// format:
// { "message": "Hi",
//   "branch": "main",
//   "parent": "1a2b3",
//     "files":
//         {
//             "file.txt":"12345",
//             "data.txt":"54321"
//         }
// }

#[derive(Debug, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Commit {
    pub message: String,
    pub branch: String,
    pub parent: String,
    pub filepaths_to_hashes: BTreeMap<PathBuf, String>,
}

impl Commit {
    pub fn new(message: String, branch: String, parent: String, files: BTreeMap<PathBuf, String>) -> Self {
        Self {
            message,
            branch,
            parent,
            filepaths_to_hashes: files,
        }
    }
}

impl Default for Commit {
    fn default() -> Self {
        Self {
            message: "".to_owned(),
            branch: "main".to_owned(),
            parent: "".to_owned(), 
            filepaths_to_hashes: BTreeMap::new(),
        }
    }
}

impl Commit {
    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }

    pub fn deserialize(object: Vec<u8>) -> Self {
        serde_json::from_slice(object.as_slice()).unwrap()
    }
    
    pub fn get_hash(&self) -> String {
        hasher(&Commit::serialize(&self))
    }
}

// TESTS //

// #[test]
// fn test_commit_serialize_work() {
//     let message = "Hi".to_owned();
//     let mut files = BTreeMap::new();
//     files.insert("file.txt".to_owned(), "12345".to_owned());
//     files.insert("data.txt".to_owned(), "54321".to_owned());
//     let parent = "1a2b3".to_owned();
//     let commit = Commit{ message, parent, filepaths_to_hashes: files };
//     let serialized = serde_json::to_string(&commit).unwrap();
//     let expected = r#"{"message":"Hi","parent":"1a2b3",files":{"data.txt":"54321","file.txt":"12345"}}"#.to_owned();
//     assert_eq!(expected, serialized)
// }

// #[test]
// fn test_commit_deserialize_work() {
//     let message = "Hi".to_owned();
//     let mut files = BTreeMap::new();
//     files.insert("file.txt".to_owned(), "12345".to_owned());
//     files.insert("data.txt".to_owned(), "54321".to_owned());
//     let parent = "1a2b3".to_owned();
//     let exp = Commit{ message, parent, filepaths_to_hashes: files };

//     let val = r#"{"message":"Hi","parent":"1a2b3",files":{"data.txt":"54321","file.txt":"12345"}}"#.to_owned();
//     let res = Commit::deserialize(val);

//     assert_eq!(exp, res)
// }

// #[test]
// fn test_commit_get_hash_work() {
//     let val = r#"{"message":"Hi","files":{"data.txt":"54321","file.txt":"12345"}}"#.to_owned();
//     let message = "Hi".to_owned();
//     let mut files = BTreeMap::new();
//     files.insert("file.txt".to_owned(), "12345".to_owned());
//     files.insert("data.txt".to_owned(), "54321".to_owned());
//     let commit = Commit{ message, files };

//     let mut hasher = Sha1::new();
//     hasher.update(Commit::serialize(&commit));
//     let exp = hasher.finalize().to_vec().to_string;

//     let res = commit.get_hash();

//     assert_eq!(exp, res)
// }
