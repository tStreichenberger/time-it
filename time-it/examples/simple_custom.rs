use std::sync::Arc;

fn main() {
    time_it::TimeItBuilder::new().time_it(|duration| {
        let millis = duration.as_millis();
        println!("[Custom Message] Time Elapsed: {}ms", millis)
    });

    for _ in 0..6 {
        time_it::time_it! {
            let x = String::from("Defined in macro");
            std::thread::sleep(std::time::Duration::from_millis(500));
        };
        println!("I still have ownership of {x}!!!");
    }
}
