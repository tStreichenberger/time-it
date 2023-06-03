#[time_it::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _time = time_it::TimeItBuilder::new().time_it(|tag, duration| {
        let millis = duration.as_millis();
        std::thread::sleep(std::time::Duration::from_millis(300));
        println!("Took {}ms for {}", millis, tag)
    });

    // TimeIt uses async mode by default
    // In async mode, the Outer time will not include the duration of the callback
    time_it::time_it!("Outer Timer", {
        for i in 0..3 {
            time_it::time_it!("Inner Timer", {
                println!("Hello World {}", i);
            });
        }
    });

    Ok(())
}
