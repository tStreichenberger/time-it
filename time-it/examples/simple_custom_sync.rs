fn main() {
    // Sync example
    // The Outer Timer will include the duration of the callback
    time_it::TimeItBuilder::new()
        .time_it(|tag, duration| {
            let millis = duration.as_millis();
            std::thread::sleep(std::time::Duration::from_millis(1000));
            println!("Time Elapsed during {tag}: {}ms", millis)
        })
        .mode(time_it::TimeItMode::Sync);

    time_it::time_it!("Outer Timer", {
        for i in 0..3 {
            time_it::time_it!("Inner Timer", {
                println!("Hello World {}", i);
            });
        }
    });
}
