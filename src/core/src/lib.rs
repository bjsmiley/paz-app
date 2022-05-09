use serde::{Deserialize, Serialize};
use state::ClientState;
use ts_rs::TS;
use std::{path::PathBuf, fs};
use thiserror::Error;

pub mod state;


pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}


pub struct Core {
    state: ClientState
}

impl Core {

    // create new instance of core, run startup tasks
    pub fn new(mut data_dir: PathBuf) -> Core {
        data_dir = data_dir.join("paz");
        let data_dir = data_dir.to_str().unwrap();

        // create data directory if it doesn't exist
        fs::create_dir_all(data_dir).unwrap();

        // prepare client state
        let mut state = ClientState::new(data_dir);

        // load from disk
        state.read_disk().unwrap();
        state.save();

        let core = Core {
            state: state,
        };

        return core;
    }

    pub fn initialize(&self) {
        println!("Info: Core: initializing...");
    }

    // handle queries
    pub fn exec_query(&self, query: ClientQuery) -> Result<CoreResponse, CoreError> {
        println!("Info: Query: {:?}", query);
        Ok(match query {
            ClientQuery::ClientGetState => CoreResponse::ClientGetState(self.state.clone()),
            _ => todo!()
        })
    }

    pub fn exec_command(&self, command: ClientCommand) -> Result<CoreResponse, CoreError> {
        println!("Info: Command: {:?}", command);
        Ok(match command {
            ClientCommand::AddOne { value } => CoreResponse::Sum(value + 1),
            ClientCommand::Add { x, y } => {
                CoreResponse::Sum(x + y)
            },
            // _ => todo!()
        })
    }

}


#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(tag = "key", content = "params")]
#[ts(export)]
pub enum ClientQuery {
    ClientGetState,
    JobGetRunning
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(tag = "key", content = "params")]
#[ts(export)]
pub enum ClientCommand {
    AddOne { value: i32 },
    Add { x: i32, y: i32},
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(tag = "key", content = "data")]
#[ts(export)]
pub enum CoreResponse {
    Success(()),
    ClientGetState(ClientState),
    Sum(i32)
}

#[derive(Error, Debug)]
pub enum CoreError {
  #[error("Query error")]
  QueryError,
//   #[error("System error")]
//   SysError(#[from] sys::SysError),
//   #[error("File error")]
//   FileError(#[from] file::FileError),
//   #[error("Job error")]
//   JobError(#[from] job::JobError),
//   #[error("Database error")]
//   DatabaseError(#[from] prisma::QueryError),
//   #[error("Database error")]
//   LibraryError(#[from] library::LibraryError),
}