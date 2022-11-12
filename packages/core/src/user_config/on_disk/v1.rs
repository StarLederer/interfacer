use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigV1 {
    pub version: Option<String>,
    pub git: Option<GitConfig>,
}

impl From<crate::user_config::Config> for ConfigV1 {
    fn from(memory_config: crate::user_config::Config) -> ConfigV1 {
        ConfigV1 {
            version: Some(String::from("1")),
            git: Some({
                let git_config = memory_config.git;

                GitConfig {
                    username: git_config.username,
                    password: git_config.password,
                }
            }),
        }
    }
}
