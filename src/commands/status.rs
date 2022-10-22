use crate::vcs_state_manager::VcsStateManager;
use crate::util::diff::Diff;

fn status() -> Diff {
    let current_dir = std::env::current_dir().ok().unwrap();
    let mut vcs_state_manager = VcsStateManager::restore(current_dir);
    if vcs_state_manager.is_none() {
        panic!(); // add error later
    }
    let vcs_state_manager = vcs_state_manager.unwrap();
    vcs_state_manager.status()
}