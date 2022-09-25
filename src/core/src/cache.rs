use tokio::{task::JoinHandle, sync::mpsc::unbounded_channel, net};
use chrono::{DateTime, Utc, Duration};
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
    pub wait_sec_dur: Duration,
    pub one_off_wait_sec_dur: Option<Duration>,
    pub reminder_dur: Duration,
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

    pub fn delay_reminder(&mut self, id: &String, delay: u64) {
        if let Some(r) = self.get_reminder(id) {
            r.delay(delay);
        }
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
                Some(r) => c.resync(&r.name, &r.wait_sec, &r.duration_sec, None) 
            }
        });
    }

    fn get_reminder(&mut self, id: &String) -> Option<&mut ActiveReminderCache> {
        self.reminders.iter_mut().find(|x| { x.id == *id})
    }

}

impl ActiveReminderCache {

    pub fn new(reminder: &ReminderState, ctx: &CoreContext) -> ActiveReminderCache {
        let now = Utc::now();
        let wait_span = chrono::Duration::seconds(i64::try_from(reminder.wait_sec).unwrap());
        let dur_span = chrono::Duration::seconds(i64::try_from(reminder.duration_sec).unwrap());
        ActiveReminderCache {
            id: reminder.id.clone(),
            name: reminder.name.clone(),
            wait_sec_dur: wait_span,
            one_off_wait_sec_dur: None,
            reminder_dur : dur_span,
            next_execution: Arc::new(Mutex::new(now + wait_span)),
            schedule: None,
            ctx: ctx.clone()
        }
    }

    pub fn delay(&mut self, delay: u64) {
        let one_off_dur = chrono::Duration::seconds(i64::try_from(delay).unwrap());
        self.one_off_wait_sec_dur = Some(one_off_dur);
        self.start()
    }

    pub fn resync(&mut self, name: &String, wait_sec: &u64, dur_sec: &u64, one_off_wait_sec: Option<u64>) {
        let wait_span = chrono::Duration::seconds(i64::try_from(*wait_sec).unwrap());
        let dur_span = chrono::Duration::seconds(i64::try_from(*dur_sec).unwrap());
        let mut restart = false;
        
        if self.name != *name {
            self.name = name.clone()
        }
        if self.reminder_dur != dur_span {
            self.reminder_dur = dur_span
        }
        if let Some(one_off_sec) = one_off_wait_sec {
            let one_off_span = chrono::Duration::seconds(i64::try_from(one_off_sec).unwrap());
            self.one_off_wait_sec_dur = Some(one_off_span);
            restart = true;
        }
        if self.wait_sec_dur != wait_span {
            self.wait_sec_dur = wait_span;
            restart = true;
        }
        if restart {
            self.stop();
            self.start()
        }
    }

    fn start(&mut self) {
        
        // setup wait duration
        let wait_dur = self.wait_sec_dur.clone();
        let wait_dur_std = wait_dur.to_std().unwrap();

        // setup reminder duration
        let reminder_dur = self.reminder_dur.clone();
        let reminder_dur_std = reminder_dur.to_std().unwrap();

        // setup one off duration
        let one_off_dur = self.one_off_wait_sec_dur.clone();

        // setup properties
        let id = self.id.clone();
        let event_emitter = self.ctx.intenal_sender.clone();
        let next_lock = self.next_execution.clone();

        self.stop();

        // spawn a background thread for this reminder
        let task = tokio::spawn(async move {

            // if one_off exists, start the execution with that delay
            // otherwise, wait the default wait delay
            let mut init_interval = match one_off_dur {
                None => interval(wait_dur_std),
                Some(one_off) => interval(one_off.to_std().unwrap())
            };
            let init_duration = match Duration::from_std(init_interval.period()) {
                Ok(d) => d,
                Err(e) => panic!("Error: {:?}", e)
            };
            
            {
                *next_lock.lock().unwrap() = Utc::now() + init_duration;
            }
            init_interval.tick().await;
            init_interval.tick().await;

            let default_dur = wait_dur + reminder_dur;
            {
                *next_lock.lock().unwrap() = Utc::now() + default_dur;
            }
            let mut default_interval = interval(wait_dur_std + reminder_dur_std);
            default_interval.tick().await;
            loop {
                {
                    *next_lock.lock().unwrap() = Utc::now() + default_dur;
                }
                event_emitter.send(InternalEvent::ReminderStart{id: id.clone()}).unwrap_or(());
                default_interval.tick().await;

            }
        });
        self.schedule = Some(task)
    }

    fn update_next_execution(next_execution: &mut Arc<Mutex<DateTime<Utc>>>, duration: &Duration) {
        let mut next_lock = next_execution.lock().unwrap();
        *next_lock = Utc::now() + *duration;
    }

    pub fn stop(&mut self) {
        if let Some(s) = &self.schedule {
            s.abort();
        }
    }

}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::{DateTime, Utc};
    use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

    use crate::{state::ReminderState, InternalEvent};
    use super::{Cache, ActiveReminderCache};

    fn create_test_cache() -> Cache {
        Cache { 
            context: crate::CoreContext { intenal_sender: unbounded_channel().0 },
            reminders: Vec::<ActiveReminderCache>::new()
        }
    }

    fn create_test_cache_with_sender(sender: UnboundedSender<InternalEvent>) -> Cache {
        Cache { 
            context: crate::CoreContext { intenal_sender: sender },
            reminders: Vec::<ActiveReminderCache>::new()
        }
    }

    #[tokio::test]
    async fn two_reminders_single_loop() {

        // assign
        let (tx, mut rx) = unbounded_channel::<InternalEvent>();
        let mut cache = create_test_cache_with_sender(tx);
        let reminder1 = ReminderState::new("test_reminder1".to_string(), 1, 1);
        let reminder2 = ReminderState::new("test_reminder2".to_string(), 2, 1);
        cache.add(&reminder1);
        cache.add(&reminder2);

        // act
        cache.start();
        let event_one = rx.recv().await;
        let event_two = rx.recv().await;
        let event_three = rx.recv().await;
        let event_four = rx.recv().await;

        // assert
        assert_eq!(event_one.unwrap(), InternalEvent::ReminderStart { id: reminder1.id.clone() });
        assert_eq!(event_two.unwrap(), InternalEvent::ReminderStart { id: reminder2.id.clone() });
        assert_eq!(event_three.unwrap(), InternalEvent::ReminderStart { id: reminder1.id.clone() });
        assert_eq!(event_four.unwrap(), InternalEvent::ReminderStart { id: reminder2.id.clone() });
    }


    #[tokio::test]
    async fn reminder_default_single_loop() {

        // assign
        let (tx, mut rx) = unbounded_channel::<InternalEvent>();
        let mut cache = create_test_cache_with_sender(tx);
        let reminder = ReminderState::new("test_reminder".to_string(), 2, 1);
        cache.add(&reminder);

        // act
        let now = Utc::now();
        cache.start();
        let next_one = cache.get_reminder(&reminder.id).unwrap().next_execution.lock().unwrap().clone();
        let event_one = rx.recv().await;
        let one_ts = Utc::now();

        let next_two = cache.get_reminder(&reminder.id).unwrap().next_execution.lock().unwrap().clone();
        let event_two = rx.recv().await;
        let two_ts = Utc::now();

        // assert
        let dur1 = (one_ts - now).num_milliseconds();
        let dur2 = (two_ts - one_ts).num_milliseconds();
        let diff1 = (next_one - one_ts).num_milliseconds();
        let diff2 = (next_two - two_ts).num_milliseconds();
        assert_eq!(event_one.unwrap(), InternalEvent::ReminderStart { id: reminder.id.clone() });
        assert!(dur1 >= 2000 && dur1 < 2040, "dur1 is {} but should be around 2000 ms", dur1);
        assert!(diff1 > -40 && diff1 < 40, "diff1 is {} but should be around 0 ms", diff1);
        assert_eq!(event_two.unwrap(), InternalEvent::ReminderStart { id: reminder.id.clone() });
        assert!(dur2 >= 2960 && dur2 < 3040, "dur2 is {} but should be around 3000 ms", dur2);
        assert!(diff2 > -40 && diff2 < 40, "diff2 is {} but should be around 0 ms", diff2);

    }

    #[tokio::test]
    async fn reminder_oneoff_single_loop() {

        // assign
        let (tx, mut rx) = unbounded_channel::<InternalEvent>();
        let mut cache = create_test_cache_with_sender(tx);
        let reminder = ReminderState::new("test_reminder".to_string(), 1, 1);
        cache.add(&reminder);

        // act
        let now = Utc::now();
        cache.start();

        let next_one = cache.get_reminder(&reminder.id).unwrap().next_execution.lock().unwrap().clone();
        let event_one = rx.recv().await;
        let one_ts = Utc::now();

        cache.delay_reminder(&reminder.id, 3);
        tokio::time::sleep(Duration::from_secs(1)).await;

        let next_two = cache.get_reminder(&reminder.id).unwrap().next_execution.lock().unwrap().clone();
        let event_two = rx.recv().await;
        let two_ts = Utc::now();

        let next_three = cache.get_reminder(&reminder.id).unwrap().next_execution.lock().unwrap().clone();
        let event_three = rx.recv().await;
        let three_ts = Utc::now();

        // assert
        let dur1 = (one_ts - now).num_milliseconds();
        let dur2 = (two_ts - one_ts).num_milliseconds();
        let dur3 = (three_ts - two_ts).num_milliseconds();
        let diff1 = (next_one - one_ts).num_milliseconds();
        let diff2 = (next_two - two_ts).num_milliseconds();
        let diff3 = (next_three - three_ts).num_milliseconds();
        assert_eq!(event_one.unwrap(), InternalEvent::ReminderStart { id: reminder.id.clone() });
        assert!(dur1 >= 1000 && dur1 < 1040, "dur1 is {} but should be around 1000 ms", dur1);
        assert!(diff1 > -40 && diff1 < 40, "diff1 is {} but should be around 0 ms", diff1);
        assert_eq!(event_two.unwrap(), InternalEvent::ReminderStart { id: reminder.id.clone() });
        assert!(dur2 >= 2960 && dur2 < 3040, "dur2 is {} but should be around 3000 ms", dur2);
        assert!(diff2 > -40 && diff2 < 40, "diff2 is {} but should be around 0 ms", diff2);
        assert_eq!(event_three.unwrap(), InternalEvent::ReminderStart { id: reminder.id.clone() });
        assert!(dur3 >= 1960 && dur3 < 2040, "dur3 is {} but should be around 2000 ms", dur3);
        assert!(diff3 > -40 && diff3 < 40, "diff2 is {} but should be around 0 ms", diff3);


    }

    #[test]
    fn cache_resync_removes_deleted() {
        let mut persisted = vec![
            ReminderState::new("r1".to_string(), 1, 1),
            ReminderState::new("r2".to_string(), 1, 1),
            ReminderState::new("r3".to_string(), 1, 1),
        ];
        let deleted = ReminderState::new("r4".to_string(), 1, 1);
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
            ReminderState::new("r1".to_string(), 1, 1),
            ReminderState::new("r2".to_string(), 1, 1),
            ReminderState::new("r3".to_string(), 1, 1),
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
        let r1 = ReminderState::new("r1".to_string(), 1, 1);
        let mut r2 = ReminderState::new("r2".to_string(), 1, 1);
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
            ReminderState::new("r1".to_string(), 1, 1),
            ReminderState::new("r2".to_string(), 1, 1),
        ];
        persisted.iter_mut().for_each(|r| r.is_active = true);
        
        let mut cache = create_test_cache();
        cache.add(&persisted[0]);

        cache.resync(&persisted);

        assert_eq!(cache.reminders.len(), 2);

    }
}