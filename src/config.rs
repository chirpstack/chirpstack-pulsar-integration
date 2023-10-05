use std::{env, fs};

use anyhow::Result;
use serde::Deserialize;

use crate::integration;

#[derive(Default, Deserialize, Clone)]
#[serde(default)]
pub struct Configuration {
    #[serde(flatten)]
    pub integration: integration::Configuration,
    pub pulsar: Pulsar,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct Pulsar {
    pub server: String,
    pub event_topic: String,
    pub auth_token: String,
    pub json: bool,
}

impl Default for Pulsar {
    fn default() -> Self {
        Pulsar {
            server: "pulsar://127.0.0.1:6650".into(),
            event_topic: "application.{{application_id}}.device.{{dev_eui}}.event.{{event}}".into(),
            auth_token: "".to_string(),
            json: false,
        }
    }
}

impl Configuration {
    pub fn load(config_file: &str) -> Result<Self> {
        let mut config_toml = fs::read_to_string(config_file)?;

        // substitute environment variables in config file
        for (k, v) in env::vars() {
            config_toml = config_toml.replace(&format!("${}", k), &v);
        }

        Ok(toml::from_str(&config_toml)?)
    }
}
