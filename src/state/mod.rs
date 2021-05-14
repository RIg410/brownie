use anyhow::Result;
use std::fs;
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {

}

impl State {
    pub fn load(config: String) -> Result<State> {
        Ok(serde_json::from_str(&fs::read_to_string(config)?)?)
    }

    pub fn store(&self, config: String) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::remove_file(&config)?;
        fs::write(&config, json)?;
        Ok(())
    }
}

