fn main() {
    time_it::time_it_2! {
        #[time_it::msg("sending request over http")]
        let x = String::from("Defined in macro");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    };
    println!("I still have ownership of {x}!!!");
}
