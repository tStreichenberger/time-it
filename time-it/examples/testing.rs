

fn main() {
    time_it::time_it2! (
        "Sleeping",
        {
            let x = String::from("Defined in macro");
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    );
    println!("I still have ownership of {x}!!!");
}