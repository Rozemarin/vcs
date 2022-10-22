use crate::vcs_state_manager::VcsStateManager;

pub fn jump(branch: Option<String>, commit: Option<String>) {
    if branch.is_some() && commit.is_some() {
        panic!();
    }

    let current_dir = std::env::current_dir().ok().unwrap();
    let mut vcs_state_manager = VcsStateManager::restore(current_dir);
    if vcs_state_manager.is_none() {
        panic!(); // add error later
    }
    // check if we have this commit later;
    let mut vcs_state_manager = vcs_state_manager.unwrap();
    
    if commit.is_some() {
        let commit_hash = commit.unwrap();
        let res = vcs_state_manager.jump_to_commit(commit_hash);
        if res.is_err() {
            println!("{:?}", res.unwrap());
            panic!(); // add error later
        }
    }

    if branch.is_some() {
        let branch_name = branch.unwrap();
        let res = vcs_state_manager.jump_to_branch(branch_name);
        if res.is_err() {
            println!("{:?}", res.unwrap());
            panic!(); // add error later
        }
    }
    
    vcs_state_manager.save_vcs_state();
}