use std::path::PathBuf;

use crate::state::*;

#[test]
fn projects_initialize() {
    let state = ProjectState::init(
        crate::project_config::Config {
            workspace_dir: String::from("./workspace"),
            hooks: crate::project_config::HookConfig::default(),
            actions: vec![],
        },
        PathBuf::from("./src/tests/fixtures/minimum-config-app"),
    );

    state.expect("Failed to init state from config");
}
