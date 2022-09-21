use std::path::PathBuf;

use crate::state::*;

#[test]
fn state_initializes() {
    let state = AppState::init(
        crate::config::Config {
            version: String::from("1"),
            workspace_dir: String::from("./workspace"),
            after_code_download: vec![],
            before_code_upload: vec![],
            actions: vec![],
        },
        PathBuf::from("./src/tests/fixtures/minimum-config-app"),
    );

    state.expect("Failed to init state from config");
}
