use serde::{Deserialize, Serialize};
use state::{ClientState, ReminderState};
use tokio::sync::{oneshot, mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel}};
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
    state: ClientState,
    // background job
    // internal channel
    // event sender
    query_channel: (UnboundedSender<ReturnableMessage<ClientQuery>>, UnboundedReceiver<ReturnableMessage<ClientQuery>>),
    command_channel: (UnboundedSender<ReturnableMessage<ClientCommand>>, UnboundedReceiver<ReturnableMessage<ClientCommand>>),
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
        state.read_disk().unwrap_or_default();
        state.save();

        let core = Core {
            state: state,
            query_channel: unbounded_channel(),
            command_channel: unbounded_channel()
        };

        return core;
    }

    pub fn initialize(&self) {
        println!("Info: Core: initializing...");
    }

    pub fn get_controller(&self) -> CoreController {
        CoreController {
          query_tx: self.query_channel.0.clone(),
          command_tx: self.command_channel.0.clone(),
        }
    }

    pub async fn start(&mut self) {
        loop {
            tokio::select! {
                Some(q) = self.query_channel.1.recv() => {
                    let res = self.exec_query(q.data).await;
                    q.tx_return.send(res).unwrap_or(());
                }
                Some(c) = self.command_channel.1.recv() => {
                    let res = self.exec_command(c.data).await;
                    c.tx_return.send(res).unwrap_or(());
                }
            }
        }
    }

    // handle queries
    pub async fn exec_query(&self, query: ClientQuery) -> Result<CoreResponse, CoreError> {
        println!("Info: Query: {:?}", query);
        Ok(match query {
            ClientQuery::ClientGetState => CoreResponse::ClientGetState(self.state.clone()),
            _ => todo!()
        })
    }

    pub async fn exec_command(&mut self, command: ClientCommand) -> Result<CoreResponse, CoreError> {
        println!("Info: Command: {:?}", command);
        Ok(match command {
            ClientCommand::AddOne { value } => CoreResponse::Sum(value + 1),
            ClientCommand::Add { x, y } => {
                CoreResponse::Sum(x + y)
            },
            ClientCommand::NewReminder { mut reminder } => {
                self.add_reminder(&mut reminder);
                CoreResponse::Success(())
            }
            // _ => todo!()
        })
    }

    fn add_reminder(&mut self, reminder: &mut ReminderState) {
        reminder.id = uuid::Uuid::new_v4().to_string();
        self.state.reminders.push(reminder.to_owned())
        // todo: refresh tokio sync job to 
    }

}

// a wrapper around external input with a returning sender channel for core to respond
#[derive(Debug)]
pub struct ReturnableMessage<D, R = Result<CoreResponse, CoreError>> {
  data: D,
  tx_return: oneshot::Sender<R>,
}

// core controller is passed to the client to communicate with the core which runs in a dedicated thread
pub struct CoreController {
  query_tx: UnboundedSender<ReturnableMessage<ClientQuery>>,
  command_tx: UnboundedSender<ReturnableMessage<ClientCommand>>,
}

impl CoreController {
    pub async fn query(&self, query: ClientQuery) -> Result<CoreResponse, CoreError> {
        
        let (tx, rx) = oneshot::channel();
        let payload = ReturnableMessage {
            data: query,
            tx_return: tx
        };

        self.query_tx
            .send(payload)
            .unwrap_or(());
         rx.await.unwrap()
    }

    pub async fn command(&self, cmd: ClientCommand) -> Result<CoreResponse, CoreError> {
        
        let (tx, rx) = oneshot::channel();
        let payload = ReturnableMessage {
            data: cmd,
            tx_return: tx
        };

        self.command_tx
            .send(payload)
            .unwrap_or(());
         rx.await.unwrap()
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
    NewReminder{ reminder: ReminderState }
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