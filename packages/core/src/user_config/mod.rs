mod on_disk;

pub use on_disk::v1::ConfigV1;

pub struct GitConfig {
    pub username: String,
    pub password: String,
}

pub struct Config {
    pub git: GitConfig,
}

impl From<crate::state::UserState> for Config {
    fn from(state: crate::state::UserState) -> Config {
        Config {
            git: {
                GitConfig {
                    username: state.git_username,
                    password: state.git_password,
                }
            },
        }
    }
}

impl From<ConfigV1> for Config {
    fn from(disk_config: ConfigV1) -> Config {
        Config {
            git: {
                let git_config = disk_config.git.unwrap_or_default();

                GitConfig {
                    username: git_config.username,
                    password: git_config.password,
                }
            },
        }
    }
}

pub fn parse_config(config_src: &String) -> Result<Config, serde_yaml::Error> {
    let config_v1 = serde_yaml::from_str::<ConfigV1>(&config_src)?;

    Ok(Config::from(config_v1))
}
