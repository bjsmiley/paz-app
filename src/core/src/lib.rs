use cache::Cache;
use serde::{Deserialize, Serialize};
use state::{ClientState, ReminderState};
use tokio::sync::{oneshot, mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel}};
use ts_rs::TS;
use std::{path::PathBuf, fs};
use thiserror::Error;

pub mod state;
pub mod cache;


pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}


pub struct Core {
    
    state: ClientState,
    cache: Cache,
    query_channel: (
        UnboundedSender<ReturnableMessage<ClientQuery>>, 
        UnboundedReceiver<ReturnableMessage<ClientQuery>>
    ),
    command_channel: (
        UnboundedSender<ReturnableMessage<ClientCommand>>, 
        UnboundedReceiver<ReturnableMessage<ClientCommand>>
    ),

    // a channel for child threads to send events back to the core
    internal_channel: (
        UnboundedSender<InternalEvent>,
        UnboundedReceiver<InternalEvent>
    )
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

        // build channels
        let internal_channel = unbounded_channel::<InternalEvent>();

        // build cache
        let cache = Cache::new(&state, CoreContext { 
            intenal_sender: internal_channel.0.clone() 
        });

        let core = Core {
            state,
            cache,
            query_channel: unbounded_channel(),
            command_channel: unbounded_channel(),
            internal_channel
        };



        return core;
    }

    pub fn initialize(&mut self) {
        println!("Info: Core: initializing...");

        // setup reminder cache instance
        self.cache = Cache::new(&self.state, self.get_context());
        self.cache.start()
    }

    pub fn get_controller(&self) -> CoreController {
        CoreController {
          query_tx: self.query_channel.0.clone(),
          command_tx: self.command_channel.0.clone(),
        }
    }

    pub fn get_context(&self) -> CoreContext {
        CoreContext { 
            intenal_sender:  self.internal_channel.0.clone()
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
                Some(e) = self.internal_channel.1.recv() => self.exec_event(e).await
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

    // handle commands
    pub async fn exec_command(&mut self, command: ClientCommand) -> Result<CoreResponse, CoreError> {
        println!("Info: Command: {:?}", command);
        Ok(match command {
            ClientCommand::AddOne { value } => CoreResponse::Sum(value + 1),
            ClientCommand::Add { x, y } => {
                CoreResponse::Sum(x + y)
            },
            ClientCommand::SaveReminders { reminders } => {
                let res = self.save_reminders(reminders);
                res
            },
            ClientCommand::DelayReminder { id, delay } => {
                self.cache.delay_reminder(&id, delay);
                CoreResponse::Success(())
            }
            // _ => todo!()
        })
    }

    // handle events
    pub async fn exec_event(&mut self, event: InternalEvent) {
        println!("Info: Event {:?}", event);
        match event {
            InternalEvent::ReminderStart { id } => self.start_reminder(id)
        }
    }

    fn save_reminders(&mut self, reminders: Vec<ReminderState>) -> CoreResponse {
        
        // persist
        self.state.reminders = reminders;
        self.state.save();

        // resync cache
        self.cache.resync(&self.state.reminders);
        CoreResponse::Success(())
    }

    fn start_reminder(&self, id: String) {
        todo!();
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

#[derive(Clone)]
pub struct CoreContext {
    pub intenal_sender: UnboundedSender<InternalEvent>
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
    SaveReminders{ reminders: Vec<ReminderState> },
    DelayReminder{ id: String, delay: u64}
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

#[derive(Debug, PartialEq)]
pub enum InternalEvent {
    ReminderStart{ id: String } 
}