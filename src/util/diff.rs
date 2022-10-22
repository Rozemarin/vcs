use std::path::PathBuf;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Diff {
    pub modified: Vec<PathBuf>,
    pub deleted: Vec<PathBuf>,
    pub added: Vec<PathBuf>,
}

impl Diff {
    pub fn get_diff(first_collection: &BTreeMap<PathBuf, String>, last_collection: &BTreeMap<PathBuf, String>) -> Self {
        let mut ans = Diff::default();
        for (file_name, hash) in last_collection.iter() {
            if first_collection.contains_key(file_name) {
                if *first_collection.get(file_name).unwrap() != *hash {
                    ans.modified.push(file_name.to_path_buf());
                }
            } else {
                ans.added.push(file_name.to_path_buf());
            }
        }
        for (file_name, hash) in first_collection {
            if !first_collection.contains_key(file_name) {
                ans.deleted.push(file_name.to_path_buf());
            }
        }
        ans
    }

    pub fn is_empty(&self) -> bool {
        if self.modified.is_empty() && self.deleted.is_empty() && self.added.is_empty() {
            return true;
        }
        false
    }
}
