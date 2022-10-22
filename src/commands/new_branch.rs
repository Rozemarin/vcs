use crate::vcs_state_manager::VcsStateManager;

pub fn new_branch(name: String) {
    let current_dir = std::env::current_dir().ok().unwrap();
    let mut vcs_state_manager = VcsStateManager::restore(current_dir);
    if vcs_state_manager.is_none() {
        panic!(); // add error later
    }
    let mut vcs_state_manager = vcs_state_manager.unwrap();
    vcs_state_manager.new_branch(name);
    vcs_state_manager.save_vcs_state();
}