use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConfigV1 {
    pub version: Option<String>,
    pub git: GitConfig,
}

pub struct Config {
    pub git: GitConfig,
}

pub fn parse_config(config_src: &String) -> Result<Config, serde_yaml::Error> {
    let config_v1 = serde_yaml::from_str::<ConfigV1>(&config_src)?;

    Ok(Config {
        git: config_v1.git,
    })
}
