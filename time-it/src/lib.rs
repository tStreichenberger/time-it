extern crate time_it_builder;
extern crate time_it_macros;

pub use time_it_builder::*;
pub use time_it_macros::*;

/// TODO: probably just make this an Option String if we want to make this part of public api
pub struct Timer(pub Option<&'static str>, pub std::time::Instant);

impl Drop for Timer {
    fn drop(&mut self) {
        // TODO: replace with run
        action(self.0, self.1.elapsed());
    }
}
