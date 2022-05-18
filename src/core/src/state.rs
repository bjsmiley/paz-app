use serde::{Deserialize, Serialize};
use ts_rs::TS;
use std::fs;
use std::io::{BufReader, Write, Error};
use std::result::Result;

pub static CLIENT_STATE_CONFIG_NAME: &str = "client_state.json";

#[derive(Debug, Serialize, Deserialize, Clone, Default, TS)]
#[ts(export)]
pub struct ClientState {
    pub first_name: String,
    pub data_path: String,
}

impl ClientState {
    
    pub fn new(data_path: &str) -> ClientState {
        let config = ClientState {
            first_name: "lol".to_string(),
            data_path: data_path.to_string()
        };
        return config;
    }

    pub fn save(&self) {
        let content = &self.clone();
        // only write to disk if config path is set
        if !content.data_path.is_empty() {
          let config_path = format!("{}/{}", content.data_path, CLIENT_STATE_CONFIG_NAME);
          //println!("trying to write to '{:?}'", config_path);
          let mut file = fs::File::create(config_path).unwrap();
          let json = serde_json::to_string(content).unwrap();
          file.write_all(json.as_bytes()).unwrap();
        }
      }
    
      pub fn read_disk(&mut self) -> Result<(),Error> {
        let config_path = format!("{}/{}", &self.data_path, CLIENT_STATE_CONFIG_NAME);
        //println!("trying to read from '{:?}'", config_path);
        // open the file and parse json
        let file = fs::File::open(config_path)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        // assign to self
        *self = data;
        Ok(())
      }
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub enum View {
  Dashboard,
  Settings
}