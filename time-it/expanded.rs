#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate time_it_macros;
pub use time_it_macros::*;
fn main() {
    let start_time = std::time::Instant::now();
    let x = 4;
    std::thread::sleep(std::time::Duration::from_millis(100));
    {
        ::std::io::_print(
            format_args!(
                "{0}: Time elapsed {1}ms\n", "some message", start_time.elapsed()
                .as_millis()
            ),
        );
    };
    {
        ::std::io::_print(format_args!("x = {0}\n", x));
    }
}
