
use tokio::task::JoinHandle;
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
use tokio::time::interval;

use crate::state::ReminderState;

pub struct Cache {
    reminders: Vec<ActiveReminderCache>
}

pub struct ActiveReminderCache {
    pub id: String,
    pub name: String,
    pub wait_sec: u64,
    pub last_execution: Arc<Mutex<DateTime<Utc>>>, // https://tokio.rs/tokio/tutorial/shared-state
    pub next_execution: Arc<Mutex<DateTime<Utc>>>,
    schedule: Option<JoinHandle<()>>
}

impl Cache {

    pub fn new() -> Cache {
        Cache { reminders: Vec::<ActiveReminderCache>::new() }
    }

    pub fn add(&mut self, reminder: &ReminderState) {
        self.reminders.push(ActiveReminderCache::new(reminder))
    } 

    pub fn start(&mut self) {
        for r in &mut self.reminders {
            r.start()
        }
    }
}

impl ActiveReminderCache {

    pub fn new(reminder: &ReminderState) -> ActiveReminderCache {
        let now = Utc::now();
        let span = chrono::Duration::seconds(i64::try_from(reminder.wait_sec).unwrap());
        ActiveReminderCache {
            id: reminder.id.clone(),
            name: reminder.name.clone(),
            wait_sec: reminder.wait_sec,
            last_execution: Arc::new(Mutex::new(now)),
            next_execution: Arc::new(Mutex::new(now + span)),
            schedule: None
        }
    }

    pub fn re_sync(&mut self, name: String, wait_sec: u64) {
        
        self.name = name;
        if self.wait_sec != wait_sec {
            self.wait_sec = wait_sec;
            let now = Utc::now();
            let span = chrono::Duration::seconds(i64::try_from(self.wait_sec).unwrap());
            self.last_execution = Arc::new(Mutex::new(now));
            self.next_execution = Arc::new(Mutex::new(now + span));
            self.start()
        }
    }

    pub fn start(&mut self) {
        let dur = self.wait_sec.clone();
        let id = self.id.clone();
        if let Some(s) = &self.schedule {
            s.abort();
        }
        let task = tokio::spawn(async move {
            let period = std::time::Duration::from_secs(dur);
            let mut interval = interval(period);
            print!("Starting {}", id);
            loop {
                interval.tick().await;
                println!("Tick {}", id);
            }
        });
        self.schedule = Some(task)
    }

}