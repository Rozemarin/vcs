use glob::glob;
use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;

use std::collections::BTreeMap;
use crate::util::{hasher::hasher, branches::Branches, state::State, diff::Diff, commit::Commit};

fn find_vcs_folder(current_dir: PathBuf) -> Option<PathBuf> {
    let ancestors = current_dir.ancestors();
    for ancestor in ancestors.into_iter() {
        let check = ancestor.join(".vcs");
        if check.exists() {
            return Some(check);
        }
    }
    None
}

fn find_repo_folder(vcs_folder: PathBuf) -> Option<PathBuf> {
    let parent = vcs_folder.parent();
    if parent.is_some() {
        return Some(parent.unwrap().to_path_buf());
    }
    None
}


pub struct VcsStateManager {
    vcs_folder: PathBuf,
    repo_folder: PathBuf,
    files_folder: PathBuf,
    commits_folder: PathBuf,
    branches: Branches,
    state: State,
}


impl VcsStateManager {
    pub fn restore(current_dir: PathBuf) -> Option<Self> {
        let vcs_folder = find_vcs_folder(current_dir);
        if vcs_folder.is_none() {
            return None;
        }
        let vcs_folder = vcs_folder.unwrap();
        let repo_folder = find_repo_folder(vcs_folder.clone());
        if repo_folder.is_none() {
            return None;
        }
        let repo_folder = repo_folder.unwrap();
        let branches = Branches::deserialize(fs::read(vcs_folder.join("branches")).ok().unwrap());
        let state = State::deserialize(fs::read(vcs_folder.join("state")).ok().unwrap());
        Some(
            Self {
                vcs_folder: vcs_folder.clone(),
                repo_folder,
                files_folder: vcs_folder.join("files"),
                commits_folder: vcs_folder.join("commits"),
                branches,
                state,
            }
        )
    }

    pub fn save_vcs_state(&self) {
        fs::write(self.vcs_folder.join("branches"), self.branches.serialize());
        fs::write(self.vcs_folder.join("state"), self.state.serialize());
    }

    pub fn initialize(inicial_dir: PathBuf) {
        let vcs_dir = inicial_dir.join(Path::new(".vcs"));
        fs::create_dir(&vcs_dir);
        fs::create_dir(vcs_dir.join("commits"));
        fs::create_dir(vcs_dir.join("files"));
        fs::write(vcs_dir.join(Path::new("branches")), Branches::default().serialize());
        fs::write(vcs_dir.join(Path::new("state")), State::default().serialize());

        let mut vcs_state_manager = VcsStateManager::restore(inicial_dir).unwrap();
    
        let mut initial_commit = vcs_state_manager.create_commit("Initial commit".to_owned());
        initial_commit.parent = "".to_owned(); // deleting fake parent hash
        let initial_commit_hash = vcs_state_manager.save_commit(initial_commit);
        vcs_state_manager.branches.branch_list.get_mut("main").unwrap().0 = initial_commit_hash;
        vcs_state_manager.save_vcs_state();
    }
    
    pub fn get_commit_by_hash(&self, commit_hash: &String) -> Commit {
        if commit_hash.is_empty() {
            return Commit::default();
        }
        let commit_file = fs::read(self.commits_folder.join(commit_hash)).ok().unwrap();
        return Commit::deserialize(commit_file);
    }
    
    pub fn save_commit(&mut self, commit: Commit) -> String { // returns hash
        let commit_hash = commit.get_hash();
        let commit_data = commit.serialize();
        fs::write(self.commits_folder.join(&commit_hash), commit_data);
        self.state.current_commit_hash = commit_hash.to_string();
        self.branches.branch_list.get_mut(&commit.branch).unwrap().1 = commit_hash.to_string();
        commit_hash
    }

    pub fn create_commit(&self, message: String) -> Commit {
        let filenames_to_hashes = self.get_all_files_in_repo();
        let (branch, parent_hash) = (self.state.current_branch.to_string(), self.state.current_commit_hash.to_string());
        let diff = self.get_diff_between_repo_and_commit(&parent_hash);
        self.save_files(&diff.modified);
        self.save_files(&diff.added);
        Commit::new(message, branch, parent_hash, filenames_to_hashes)
    }
    
    pub fn save_files(&self, file_paths: &Vec<PathBuf>) {
        for file_path in file_paths {
            let content = fs::read(file_path).unwrap();;
            let file_hash = hasher(&content);
            let new_path_in_files_folder = self.files_folder.join(file_hash);
            if !new_path_in_files_folder.exists() {
                fs::write(new_path_in_files_folder, content);
            }
        }
    }
    
    pub fn get_all_files_in_repo(&self) -> BTreeMap<PathBuf, String> {
        let mut file_names = BTreeMap::new();
        for entry in WalkDir::new(&self.repo_folder)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| !e.file_type().is_dir())
                 {
            let f_path = entry.path();
            let mut flag = false;
            for component in f_path.components() {
                if String::from(component.as_os_str().to_str().unwrap()) == ".vcs".to_string() {
                    flag = true;
                    break;
                }
            }
            if flag {
                continue;
            }
            flag = true;
            let f_content = fs::read(f_path); // add error
            file_names.insert(f_path.to_path_buf(), hasher(&f_content.unwrap()));
        }
        file_names
    }

    // fn get_diff_between_two_commits(&self, first_commit_hash: String, last_commit_hash: String) -> Diff {
    //     let first_commit = self.get_commit_by_hash(&first_commit_hash);
    //     let last_commit = self.get_commit_by_hash(&last_commit_hash);
    //     Diff::get_diff(first_commit.filepaths_to_hashes, last_commit.filepaths_to_hashes)
    // }
    
    pub fn get_diff_between_repo_and_commit(&self, commit_hash: &String) -> Diff {
        let all_files = self.get_all_files_in_repo();
        let last_commit_files = self.get_commit_by_hash(commit_hash).filepaths_to_hashes;
        Diff::get_diff(&last_commit_files, &all_files)
        // let mut ans = Diff::default();
        // for (file_name, hash) in all_files.iter() {
        //     if last_commit_files.contains_key(file_name) {
        //         if *last_commit_files.get(file_name).unwrap() != *hash {
        //             ans.modified.push(file_name.to_path_buf());
        //         }
        //     } else {
        //         ans.added.push(file_name.to_path_buf());
        //     }
        // }
        // for (file_name, hash) in last_commit_files {
        //     if !all_files.contains_key(&file_name) {
        //         ans.deleted.push(file_name.to_path_buf());
        //     }
        // }
        // ans
    }

    pub fn status(&self) -> Diff {
        self.get_diff_between_repo_and_commit(&self.state.current_commit_hash)
    }

    pub fn jump_to_commit(&mut self, commit_hash: String) -> Result<bool, String> {
        // считаем разницу между текущим состоянием репозитория и 
        let diff = self.status();
        if !diff.is_empty() {
            return Err("Unstaged changes".to_owned());
        }
        // deleting all files in repo (later can be upgrated to partial delition)
        let current_files = self.get_all_files_in_repo();
        for (filepath, hash) in current_files {
            fs::remove_file(filepath);
        }
        // restoring files from storage
        let commit = self.get_commit_by_hash(&commit_hash);
        for (filepath, hash) in commit.filepaths_to_hashes {
            let contents = fs::read(self.files_folder.join(hash)).unwrap();
            fs::write(filepath, contents);
        }
        self.state.current_branch = commit.branch;
        self.state.current_commit_hash = commit_hash;
        return Ok(true);
    }

    pub fn jump_to_branch(&mut self, branch: String) -> Result<bool, String> {
        if !self.branches.branch_list.contains_key(&branch) {
            return Err("No such branch".to_owned());
        }
        // get last commit in branch and then jump_to_commit
        let commit_hash = self.branches.branch_list.get_mut(&branch).unwrap().1.to_string();
        self.jump_to_commit(commit_hash)
    }

    pub fn new_branch(&mut self, name: String) -> Result<bool, String> {
        if self.state.current_branch != "main".to_owned() {
            return Err("not on main".to_owned());
        }
        self.state.current_branch = name;
        self.branches.branch_list.insert(self.state.current_branch.to_string(), (self.state.current_commit_hash.to_string(), self.state.current_commit_hash.to_string()));
        Ok(true)
    }

    pub fn merde(&mut self, branch: String) -> Result<bool, String> {
        if self.state.current_branch != "main".to_owned() {
            return Err("not on main".to_owned());
        }
        if self.branches.branch_list["main"].1 != self.state.current_commit_hash {
            return Err("not last commit on main".to_owned());
        }
        let diff = self.status();
        if !diff.is_empty() {
            return Err("Unstaged changes".to_owned());
        }
        // here we are on final commit on main
        let fork_commit = self.get_commit_by_hash(&self.branches.branch_list[&branch].0);
        // diff on feature branch
        let last_commit_in_feature = self.get_commit_by_hash(&self.branches.branch_list[&branch].1.to_string());
        let diff_featue = Diff::get_diff(&fork_commit.filepaths_to_hashes, &last_commit_in_feature.filepaths_to_hashes);

        // diff on main branch
        let last_commit_in_main = self.get_commit_by_hash(&self.branches.branch_list["main"].1);
        let diff_main = Diff::get_diff(&fork_commit.filepaths_to_hashes, &last_commit_in_main.filepaths_to_hashes);
        Ok(true)
    }
}
