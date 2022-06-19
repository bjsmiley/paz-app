use tokio::{task::JoinHandle, sync::mpsc::unbounded_channel};
use chrono::{DateTime, Utc};
use std::{sync::{Arc, Mutex}, collections::{HashMap, HashSet}};
use tokio::time::interval;

use crate::{state::{ReminderState, ClientState}, CoreContext, InternalEvent};

pub struct Cache {
    context : CoreContext,
    reminders: Vec<ActiveReminderCache>
}

pub struct ActiveReminderCache {
    pub id: String,
    pub name: String,
    pub wait_sec: u64,
    pub last_execution: Arc<Mutex<DateTime<Utc>>>, // https://tokio.rs/tokio/tutorial/shared-state
    pub next_execution: Arc<Mutex<DateTime<Utc>>>,
    ctx: CoreContext,
    schedule: Option<JoinHandle<()>>
}

impl Cache {

    pub fn new(state: &ClientState, ctx: CoreContext) -> Cache {
        let mut cache = Cache { 
            context: ctx, 
            reminders: Vec::<ActiveReminderCache>::new() 
        };

        state.reminders
            .iter()
            .filter(|x| x.is_active)
            .for_each(|x| cache.add(x));

        cache
    }

    pub fn add(&mut self, reminder: &ReminderState) {
        self.reminders.push(ActiveReminderCache::new(reminder, &self.context))
    } 

    pub fn start(&mut self) {
        self.reminders.iter_mut().for_each(|r| {
            r.start()
        })
    } 

    pub fn resync(&mut self, reminders: &Vec<ReminderState>) {


        // fill reminder map with persisted reminders
        let mut rem_map = HashMap::<String,&ReminderState>::new();
        reminders.iter().for_each(|r| {
            rem_map.insert(r.id.clone(), r);
        });

        // record the cached id
        let mut cache_ids = HashSet::<String>::new();
        self.reminders.iter().for_each(|c| {
            cache_ids.insert(c.id.clone());
        });

        // add new reminders to cache
        reminders.iter().for_each(|r| {
            if !cache_ids.contains(&r.id) {
                let mut new = ActiveReminderCache::new(r, &self.context);
                new.start();
                self.reminders.push(new)
            }
        });

        // stop all deleted reminders from cache
        // seperate for_each since retain_mut() is a nightly feature
        self.reminders.iter_mut().for_each(|r| {
            if !rem_map.contains_key(&r.id) {
                r.stop();
            }
            else if let Some(rr) = rem_map.get(&r.id) {
                if !rr.is_active {
                    r.stop();
                }
            }
        });

        // remove all deleted reminders from cache
        self.reminders.retain(|r| {
            if !rem_map.contains_key(&r.id) {
                return false
            }
            else if let Some(rr) = rem_map.get(&r.id) {
                if !rr.is_active {
                    return false
                }
            }
            return true
        });

        // resync all cached reminders
        self.reminders.iter_mut().for_each(|c| {
            match rem_map.get(&c.id) {
                None => panic!("impossible"),
                Some(r) => c.resync(&r.name, &r.wait_sec) 
            }
        });
    }

}

impl ActiveReminderCache {

    pub fn new(reminder: &ReminderState, ctx: &CoreContext) -> ActiveReminderCache {
        let now = Utc::now();
        let span = chrono::Duration::seconds(i64::try_from(reminder.wait_sec).unwrap());
        ActiveReminderCache {
            id: reminder.id.clone(),
            name: reminder.name.clone(),
            wait_sec: reminder.wait_sec,
            last_execution: Arc::new(Mutex::new(now)),
            next_execution: Arc::new(Mutex::new(now + span)),
            schedule: None,
            ctx: ctx.clone()
        }
    }

    pub fn resync(&mut self, name: &String, wait_sec: &u64) {
        
        if self.name != *name {
            self.name = name.clone()
        }
        if self.wait_sec != *wait_sec {
            self.wait_sec = *wait_sec;
            let now = Utc::now();
            let span = chrono::Duration::seconds(i64::try_from(self.wait_sec).unwrap());
            self.last_execution = Arc::new(Mutex::new(now));
            self.next_execution = Arc::new(Mutex::new(now + span));
            self.start()
        }
    }

    fn start(&mut self) {
        let dur = self.wait_sec.clone();
        let id = self.id.clone();
        let event_emitter = self.ctx.intenal_sender.clone();
        self.stop();
        let task = tokio::spawn(async move {
            let period = std::time::Duration::from_secs(dur);
            let mut interval = interval(period);
            //println!("Info: Reminder: Starting {}", id);
            interval.tick().await;
            loop {
                interval.tick().await;
                //println!("Info: Reminder: Tick {}", id);
                event_emitter.send(InternalEvent::ReminderStart{id: id.clone()}).unwrap_or(());
            }
        });
        self.schedule = Some(task)
    }

    pub fn stop(&mut self) {
        if let Some(s) = &self.schedule {
            s.abort();
        }
    }

}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc::unbounded_channel;

    use crate::state::ReminderState;
    use super::{Cache, ActiveReminderCache};

    fn create_test_cache() -> Cache {
        Cache { 
            context: crate::CoreContext { intenal_sender: unbounded_channel().0 },
            reminders: Vec::<ActiveReminderCache>::new()
        }
    }

    #[test]
    fn cache_resync_removes_deleted() {
        let mut persisted = vec![
            ReminderState::new("r1".to_string(), 1),
            ReminderState::new("r2".to_string(), 1),
            ReminderState::new("r3".to_string(), 1),
        ];
        let deleted = ReminderState::new("r4".to_string(), 1);
        persisted.iter_mut().for_each(|r| r.is_active = true);


        let mut cache = create_test_cache();
        cache.add(&persisted[0]);
        cache.add(&persisted[1]);
        cache.add(&persisted[2]);
        cache.add(&deleted);

        cache.resync(&persisted);

        assert_eq!(cache.reminders.len(), 3)

    }

    #[tokio::test]
    async fn cache_resync_adds_new() {
        let mut persisted = vec![
            ReminderState::new("r1".to_string(), 1),
            ReminderState::new("r2".to_string(), 1),
            ReminderState::new("r3".to_string(), 1),
        ];
        persisted.iter_mut().for_each(|r| r.is_active = true);

        let mut cache = create_test_cache();
        cache.add(&persisted[0]);
        cache.add(&persisted[1]);

        cache.resync(&persisted);

        assert_eq!(cache.reminders.len(), 3)
    }

    #[tokio::test]
    async fn cache_resync_unactive_removed() {
        let r1 = ReminderState::new("r1".to_string(), 1);
        let mut r2 = ReminderState::new("r2".to_string(), 1);
        r2.is_active = true;

        let persisted = vec![
            r1,
            r2
        ];

        let mut cache = create_test_cache();
        cache.add(&persisted[0]);
        cache.add(&persisted[1]);

        cache.resync(&persisted);

        assert_eq!(cache.reminders.len(), 1);
    }

    #[tokio::test]
    async fn cache_resync_active_added() {
        let mut persisted = vec![
            ReminderState::new("r1".to_string(), 1),
            ReminderState::new("r2".to_string(), 1),
        ];
        persisted.iter_mut().for_each(|r| r.is_active = true);
        
        let mut cache = create_test_cache();
        cache.add(&persisted[0]);

        cache.resync(&persisted);

        assert_eq!(cache.reminders.len(), 2);

    }
}