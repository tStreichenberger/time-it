use crossbeam_channel::{unbounded, Sender};
use lazy_init::Lazy;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

lazy_static::lazy_static! {
    static ref TIMEIT_CONFIG: RwLock<TimeItConfig> = RwLock::new(TimeItConfig::default());
    static ref LAZY_TIMEIT_CONFIG: Lazy<TimeItConfig> = Lazy::new();
}

type TimeItCallBack = fn(&str, Duration) -> ();

#[derive(Clone)]
pub enum TimeItMode {
    Sync,
    Async,
}

#[derive(Debug)]
pub struct TimeItEvent {
    pub tag: Option<String>,
    pub duration: Duration,
}

pub struct TimeItConfig {
    pub action: TimeItCallBack,
    pub default_tag: String,
    pub mode: TimeItMode,
    pub sender: Option<Sender<TimeItEvent>>,
    pub receiver_handle: Option<thread::JoinHandle<()>>,
    pub shutdown: Arc<AtomicBool>,
}

impl Default for TimeItConfig {
    fn default() -> Self {
        TimeItConfig {
            action: |tag, duration| {
                println!("[{tag}] Time elapsed: {:?}", duration);
            },
            receiver_handle: None,
            default_tag: String::from("TimeIt"),
            mode: TimeItMode::Sync,
            sender: None,
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl TimeItConfig {
    pub fn wait_for_finish(&mut self) {
        if let Some(handle) = self.receiver_handle.take() {
            handle.join().unwrap();
        }
    }
}

pub struct TimeItBuilder {}

impl TimeItBuilder {
    pub fn new() -> Self {
        TimeItBuilder {}
    }

    /// This function is called when the timer is done.
    /// The first argument is the name of the timer, the second is the duration.
    pub fn time_it(&self, action: TimeItCallBack) -> &Self {
        let mut config = TIMEIT_CONFIG.write();
        config.action = action;
        self
    }

    pub fn default_tag(&self, tag: impl ToString) -> &Self {
        let mut config = TIMEIT_CONFIG.write();
        config.default_tag = tag.to_string();
        self
    }

    pub fn mode(&self, mode: TimeItMode) -> &Self {
        let mut config = TIMEIT_CONFIG.write();
        config.mode = mode;
        self
    }
}

pub fn get_config() -> &'static TimeItConfig {
    LAZY_TIMEIT_CONFIG.get_or_create(|| {
        let mut config = TIMEIT_CONFIG.write();
        let mut lazy_config = std::mem::replace(&mut *config, TimeItConfig::default());
        let default_tag = lazy_config.default_tag.clone();
        let shutdown = config.shutdown.clone();
        match lazy_config.mode {
            TimeItMode::Async => {
                let (sender, receiver) = unbounded::<TimeItEvent>();
                let handle = thread::spawn(move || {
                    while !(shutdown.load(Ordering::Relaxed) && receiver.is_empty()) {
                        match receiver.recv() {
                            Ok(event) => {
                                (lazy_config.action)(
                                    event.tag.as_ref().unwrap_or(&default_tag),
                                    event.duration,
                                );
                            }
                            Err(_) => {
                                break;
                            }
                        }
                    }
                });
                lazy_config.sender = Some(sender);
                // Keep the config in the original struct because we want to get mutable reference to it later for cleaning up
                config.receiver_handle = Some(handle);
                lazy_config
            }
            TimeItMode::Sync => lazy_config,
        }
    })
}

pub fn run(tag: Option<&str>, duration: Duration) {
    let config = get_config();
    match config.mode {
        TimeItMode::Async => {
            emit(tag, duration);
        }
        TimeItMode::Sync => {
            action(tag, duration);
        }
    }
}

pub fn wait_for_finish() {
    let mut config = TIMEIT_CONFIG.write();
    config.shutdown.store(true, Ordering::Relaxed);
    config.wait_for_finish()
}

fn action(tag: Option<&str>, duration: Duration) {
    let config = get_config();
    (config.action)(tag.unwrap_or(&config.default_tag), duration);
}

fn emit(tag: Option<&str>, duration: Duration) {
    let config = get_config();
    let sender = config.sender.as_ref().unwrap();
    sender
        .send(TimeItEvent {
            tag: tag.map(|s| s.to_string()),
            duration,
        })
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        TimeItBuilder::new();
    }

    #[test]
    fn it_works_async() {
        TimeItBuilder::new()
            .time_it(|tag, duration| {
                let millis = duration.as_millis();
                println!("Time Elapsed during {tag}: {}ms", millis)
            })
            .mode(TimeItMode::Async);

        let config = self::get_config();
        let action = config.action.clone();
        action("Custom Message", Duration::from_millis(100));
    }

    #[test]
    fn it_works_sync() {
        TimeItBuilder::new()
            .time_it(|tag, duration| {
                let millis = duration.as_millis();
                println!("Time Elapsed during {tag}: {}ms", millis)
            })
            .mode(TimeItMode::Sync);

        let config = self::get_config();
        let action = config.action.clone();
        action("Custom Message", Duration::from_millis(100));
    }
}
