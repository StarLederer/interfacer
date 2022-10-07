mod config_latest;

pub use self::config_latest::*;

#[derive(Default)]
pub struct HookConfig {
    pub before_each_action: Vec<String>,
    pub after_code_download: Vec<String>,
    pub before_code_upload: Vec<String>,
}

#[derive(Clone)]
pub struct ActionConfig {
    pub command: String,
    pub user_terminated: bool,
    pub idle_name: String,
    pub active_name: String,
}

pub struct Config {
    pub workspace_dir: String,
    pub hooks: HookConfig,
    pub actions: Vec<ActionConfig>,
}

impl From<config_latest::ActionConfigLatest> for ActionConfig {
    fn from(config_v1: ActionConfigLatest) -> Self {
        ActionConfig {
            command: config_v1.command,
            user_terminated: config_v1.user_terminated,
            idle_name: config_v1.idle_name,
            active_name: config_v1.active_name,
        }
    }
}

pub fn parse_config(config_src: &String, defaults: Config) -> Result<Config, serde_yaml::Error> {
    let config = serde_yaml::from_str::<ConfigLatest>(&config_src)?;

    Ok(Config {
        workspace_dir: config.workspace_dir.unwrap_or(defaults.workspace_dir),
        hooks: {
            if let Some(hooks) = config.hooks {
                HookConfig {
                    before_each_action: hooks
                        .before_each_action
                        .unwrap_or(defaults.hooks.before_each_action),
                    after_code_download: hooks
                        .after_code_download
                        .unwrap_or(defaults.hooks.after_code_download),
                    before_code_upload: hooks
                        .before_code_upload
                        .unwrap_or(defaults.hooks.before_code_upload),
                }
            } else {
                defaults.hooks
            }
        },
        actions: {
            if let Some(v1_actions) = config.actions {
                let mut actions: Vec<ActionConfig> = vec![];

                for v1_action in v1_actions.iter() {
                    actions.push(ActionConfig::from(v1_action.to_owned()));
                }

                actions
            } else {
                defaults.actions
            }
        },
    })
}
