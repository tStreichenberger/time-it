
fn main() {
    // can optionally define tag
    time_it::time_it!("Sending Request to Backend", {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let data = "Data base has processed request successfully";
    });

    println!("{data}");

    // or don't... will use default set in config
    time_it::time_it! {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    };
}
