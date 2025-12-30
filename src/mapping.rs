use std::{collections::HashMap, error::Error};

use log::debug;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

use crate::midi::note::{MAPPING, MidiNote};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceConfig {
    input: Option<String>,
    output: Option<String>,
    pub lights: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    mapping: HashMap<MidiNote, rdev::Key>,
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
        //let m = MAPPING.lock().await;
        let m = toml.mapping.clone();

        for (_, (m, ac)) in m.iter().enumerate() {
            debug!("MAPPING: {:?} = {:?}", ac, m);

            {
                let mut mapping = MAPPING.lock().unwrap();
                mapping.insert(*m, *ac);
            }
        }

        Ok(toml)
    }

    pub fn get_input_name(&self) -> Option<String> {
        self.device.input.clone()
    }

    pub fn get_output_name(&self) -> Option<String> {
        self.device.output.clone()
    }
}
