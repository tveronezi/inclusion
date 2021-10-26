use serde::Deserialize;

const ERR_MSG: &str = "Unable to set default";

lazy_static::lazy_static! {
    static ref DEFAULT_CONFIG: upstream_config::Config = {
        let mut config = upstream_config::Config::default();
        config.set_default("inclusion.ini", "inclusion.ini").expect(ERR_MSG);
        // The defaults favour development.
        // We need to override these for the proper production environment.
        config.set_default("bind", "127.0.0.1:4000").expect(ERR_MSG);
        config.set_default("articles.db.url", "postgres://rusty:rusty@localhost:5432/articlesdb").expect(ERR_MSG);
        config
    };

    static ref ENV_CONFIG: upstream_config::Environment = {
        let env_config = upstream_config::Environment::default();
        env_config.separator("_")
    };
}

#[derive(Debug)]
pub struct Config {
    config: upstream_config::Config,
}

pub enum ConfigKey {
    ArticlesDbUrl,
    Bind,
}

impl Config {
    pub fn get<'de, T: Deserialize<'de>>(&self, key: ConfigKey) -> T {
        match key {
            ConfigKey::ArticlesDbUrl => self
                .config
                .get("articles.db.url")
                .expect("Default value missing."),
            ConfigKey::Bind => self.config.get("bind").expect("Default value missing."),
        }
    }
}

pub fn get_settings() -> Config {
    let config = upstream_config::Config::default()
        .with_merged(DEFAULT_CONFIG.clone())
        .expect("Unable to load config default values.");
    // environment variables values
    let config = config
        .with_merged(ENV_CONFIG.clone())
        .expect("Unable to load environment variables.");
    // ini file values values
    let ini_file = &config.get_str("inclusion.ini").unwrap();
    let config = config
        .with_merged({
            let ini_config = upstream_config::File::with_name(ini_file.as_str());
            ini_config.required(false)
        })
        .expect("Unable to load the 'inclusion.ini' file");
    Config { config }
}
