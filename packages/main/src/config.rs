use serde::{Deserialize, Serialize};

macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!(WrappActionCommand {
    lang: String,
    command: String,
});

pub_struct! (WrappAction {
    name: String,
    commands: Vec<WrappActionCommand>,
});

pub_struct!(WrappConfig {
    version: String,
    workspace_dir: Option<String>,
    after_code_download: Vec<String>,
    before_code_upload: Vec<String>,
    actions: Vec<WrappAction>,
});
