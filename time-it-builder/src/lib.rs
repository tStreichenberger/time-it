use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

lazy_static::lazy_static! {
    static ref TIMER_CONFIG: Arc<RwLock<TimeItConfig>> = Arc::new(RwLock::new(TimeItConfig::default()));
}

pub struct TimeItConfig {
    pub action: Option<Box<dyn Fn(Duration) -> () + Send + Sync>>,
}

impl Default for TimeItConfig {
    fn default() -> Self {
        TimeItConfig { action: None }
    }
}

pub struct TimeItBuilder {}

impl TimeItBuilder {
    pub fn new() -> Self {
        TimeItBuilder {}
    }

    /// This function is called when the timer is done.
    /// The first argument is the name of the timer, the second is the duration.
    pub fn time_it(self, action: Box<dyn Fn(Duration) -> () + Send + Sync>) -> Self {
        let mut config = TIMER_CONFIG.write().unwrap();
        config.action = Some(action);
        self
    }
}

pub fn get_config() -> Arc<RwLock<TimeItConfig>> {
    TIMER_CONFIG.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        TimeItBuilder::new().time_it(Box::new(|duration| {
            let millis = duration.as_millis();
            println!("[Custom Message] Time Elapsed: {}ms", millis)
        }));
    }
}
