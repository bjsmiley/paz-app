use serde::{Deserialize, Serialize};
use ts_rs::TS;
use std::fs;
use std::io::{BufReader, Write, Error};
use std::result::Result;
use uuid::Uuid;

pub static CLIENT_STATE_CONFIG_NAME: &str = "client_state.json";

#[derive(Debug, Serialize, Deserialize, Clone, Default, TS)]
#[ts(export)]
pub struct ClientState {
    pub client_uuid: String,
    pub data_path: String,
    pub reminders: Vec<ReminderState>
}

impl ClientState {
    
    pub fn new(data_path: &str) -> ClientState {
        let mut config = ClientState {
            client_uuid: Uuid::new_v4().to_string(),
            data_path: data_path.to_string(),
            reminders: Vec::new(),
        };

        config.reminders.push(ReminderState::new("Stretch".to_string(), 10 * 1000, 15 * 1000));
        config.reminders.push(ReminderState::new("Drink Water".to_string(), 20 * 1000, 30 * 1000));
        return config;
    }

    pub fn save(&self) {
        let content = &self.clone();
        // only write to disk if config path is set
        if !content.data_path.is_empty() {
          let config_path = format!("{}/{}", content.data_path, CLIENT_STATE_CONFIG_NAME);
          let mut file = fs::File::create(config_path).unwrap();
          let json = serde_json::to_string(content).unwrap();
          file.write_all(json.as_bytes()).unwrap();
        }
      }
    
      pub fn read_disk(&mut self) -> Result<(),Error> {
        let config_path = format!("{}/{}", &self.data_path, CLIENT_STATE_CONFIG_NAME);
        // open the file and parse json
        let file = fs::File::open(config_path)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        // assign to self
        *self = data;
        Ok(())
      }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, TS)]
#[ts(export)]
pub struct ReminderState {
  pub id: String,
  pub name: String,
  pub is_active: bool,
  pub wait_ms: i32,
  pub duration_ms: i32
}

impl ReminderState {
    pub fn new(name: String, wait: i32, duration: i32) -> ReminderState {
      ReminderState {
        id: Uuid::new_v4().to_string(),
        name: name,
        is_active: false,
        wait_ms: wait,
        duration_ms: duration
      }
    }
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub enum View {
  Dashboard,
  Settings
}