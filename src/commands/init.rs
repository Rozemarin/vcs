#![forbid(unsafe_code)]

use crate::vcs_state_manager::VcsStateManager;
use std::path::PathBuf;

pub fn init(path: PathBuf) {
    if VcsStateManager::restore(path.clone()).is_some() {
        panic!(); // add error later
    }
    VcsStateManager::initialize(path);
}