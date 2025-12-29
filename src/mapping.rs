use std::{collections::HashMap, error::Error, sync::Mutex};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

use crate::message::Note;

pub static MAPPING: Lazy<Mutex<HashMap<Note, rdev::Key>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    mapping: HashMap<rdev::Key, String>,
    device: HashMap<String, String>,
}

impl Config {
    pub async fn init() -> Result<Self, Box<dyn Error>> {
        let mut input = File::open("./Mapping.toml").await?;
        let mut s = String::new();
        input.read_to_string(&mut s).await.unwrap();

        let toml: Config = toml::from_str(&s).expect("failed parsing the config");
        //let m = MAPPING.lock().await;
        let m = toml.mapping.clone();
        
        for (_, (ac, m)) in m.iter().enumerate() {
            println!("MAPPING: {:?} = {:?}", ac, m);
            //let key: Actions = ac.as_str().into();
            //let key: rdev::Key = 
            let midi: Note = m.as_str().into();
            
            {
                let mut mapping = MAPPING.lock().unwrap();
                mapping.insert(midi, *ac);
            }
        }

        Ok(toml)
    }

    pub fn get_input_name(&self) -> Option<String> {
        self.device.get("input").cloned()
    }

    pub fn get_output_name(&self) -> Option<String> {
        self.device.get("output").cloned()
    }
}