use lazy_init::Lazy;
use parking_lot::RwLock;
use std::{
    sync::Arc,
    time::Duration,
};

lazy_static::lazy_static! {
    static ref TIMER_CONFIG: RwLock<TimeItConfig> = RwLock::new(TimeItConfig::default());
    static ref LAZY_TIMER_CONFIG: Lazy<Arc<TimeItConfig>> = Lazy::new();
}

type TimeItCallBack = fn(&str, Duration) -> ();

#[derive(Clone)]
pub struct TimeItConfig {
    pub action: TimeItCallBack,
}

impl Default for TimeItConfig {
    fn default() -> Self {
        TimeItConfig {
            action: |tag, duration| {
                let time_elapsed = duration.as_millis();
                println!("[{tag}] Time elapsed: {}ms", time_elapsed);
            },
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
        let mut config = TIMER_CONFIG.write();
        config.action = action;
        self
    }
}

pub fn get_config() -> Arc<TimeItConfig> {
    Arc::clone(LAZY_TIMER_CONFIG.get_or_create(|| Arc::new(TIMER_CONFIG.read().clone())))
}

pub fn action(tag: &str, duration: Duration) {
    let config = LAZY_TIMER_CONFIG.get_or_create(|| Arc::new(TIMER_CONFIG.read().clone()));
    (config.action)(tag, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        TimeItBuilder::new().time_it(|tag, duration| {
            let millis = duration.as_millis();
            println!("Time Elapsed during {tag}: {}ms", millis)
        });

        let config = self::get_config();
        let action = config.action.clone();
        action("Custom Message", Duration::from_millis(100));
    }
}
