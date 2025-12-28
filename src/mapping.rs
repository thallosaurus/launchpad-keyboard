use std::{collections::HashMap, error::Error};

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};



#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    mapping: HashMap<String, String>,
    device: HashMap<String, String>,
}

impl Config {
    pub async fn init() -> Result<Self, Box<dyn Error>> {
        let mut input = File::open("./Mapping.toml").await?;
        let mut s = String::new();
        input.read_to_string(&mut s).await.unwrap();

        Ok(toml::from_str(&s).expect("failed parsing the config"))
    }

    pub fn get_input_name(&self) -> Option<String> {
        self.device.get("input").cloned()
    }

    pub fn get_output_name(&self) -> Option<String> {
        self.device.get("output").cloned()
    }
}