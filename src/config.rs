use std::{collections::HashMap, error::Error};

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{DeviceNameRetrieve, midi::{note::{MidiNote, set_mapping}, output::OutputDeviceNameRetrieve}};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Key(rdev::Key),
    Shell {
        press: Option<String>,
        release: Option<String>
    },

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceConfig {
    input: Option<String>,
    output: Option<String>,
    pub lights: bool,
}

impl DeviceNameRetrieve for DeviceConfig {
    fn get_input_name(&self) -> Option<String> {
        self.input.clone()
    }

    fn get_output_name(&self) -> Option<String> {
        self.output.clone()
    }
}

impl OutputDeviceNameRetrieve for DeviceConfig {
    fn get_light_status(&self) -> bool {
        self.lights
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Integration {
    input: String,
    output: String,
}

impl DeviceNameRetrieve for Integration {
    fn get_input_name(&self) -> Option<String> {
        Some(self.input.clone())
    }

    fn get_output_name(&self) -> Option<String> {
        Some(self.output.clone())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    mapping: HashMap<MidiNote, Action>,
    pub integration: Integration,
    //device: HashMap<String, String>,
    pub device: DeviceConfig,
}

impl Config {
    pub async fn init(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut input = File::open(path).await?;
        let mut s = String::new();
        input
            .read_to_string(&mut s)
            .await
            .expect("error reading mapping file");
        let toml: Config = toml::from_str(&s).expect("failed parsing the config");

        set_mapping(toml.mapping.clone());

        Ok(toml)
    }
}
