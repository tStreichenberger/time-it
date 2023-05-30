fn main() {
    time_it::TimeItBuilder::new().time_it(|tag, duration| {
        let millis = duration.as_millis();
        println!("Took {}ms for {}", millis, tag)
    });

    for _ in 0..6 {
        time_it::time_it! {
            let x = String::from("Defined in macro");
            std::thread::sleep(std::time::Duration::from_millis(500));
        };
        println!("I still have ownership of {x}!!!");
    }
}
