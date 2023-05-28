use lazy_init::Lazy;
use parking_lot::RwLock;
use std::{sync::Arc, time::Duration};

lazy_static::lazy_static! {
    static ref TIMER_CONFIG: RwLock<TimeItConfig> = RwLock::new(TimeItConfig::default());
    static ref LAZY_TIMER_CONFIG: Lazy<Arc<TimeItConfig>> = Lazy::new();
}

#[derive(Clone)]
pub struct TimeItConfig {
    pub action: Arc<Box<dyn Fn(Duration) -> () + Send + Sync>>,
}

impl Default for TimeItConfig {
    fn default() -> Self {
        TimeItConfig {
            action: Arc::new(Box::new(|duration| {
                let time_elapsed = duration.as_millis();
                println!("[TimeIt] Time elapsed: {}ms", time_elapsed);
            })),
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
    pub fn time_it(self, action: Box<dyn Fn(Duration) -> () + Send + Sync>) -> Self {
        let mut config = TIMER_CONFIG.write();
        config.action = Arc::new(action);
        self
    }
}

pub fn get_config() -> Arc<TimeItConfig> {
    Arc::clone(LAZY_TIMER_CONFIG.get_or_create(|| Arc::new(TIMER_CONFIG.read().clone())))
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

        let config = self::get_config();
        let action = config.action.clone();
        action(Duration::from_millis(100));
    }
}
