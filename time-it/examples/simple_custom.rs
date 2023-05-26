use std::time::Duration;

fn main() {
    time_it::TimeItBuilder::new().time_it(Box::new(|duration| {
        let millis = duration.as_millis();
        println!("[Custom Message] Time Elapsed: {}ms", millis)
    }));

    time_it::time_it! {
        let x = String::from("Defined in macro");
        std::thread::sleep(std::time::Duration::from_millis(500));
    };

    println!("I still have ownership of {x}!!!");
}
