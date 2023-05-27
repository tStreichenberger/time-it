/// # Expected Output: 
/// ```text
/// Finshed Sleep 1
/// Operation took 1002ms
/// Finshed Sleep 2
/// Operation took 1000ms
/// Finshed Sleep 3
/// Operation took 1000ms
/// Operation took 3002ms
/// ```
fn main() {
    time_it::time_it! {
        time_it::time_it!{
            std::thread::sleep(std::time::Duration::from_millis(1000));
            println!("Finshed Sleep 1");
        }
        time_it::time_it!{
            std::thread::sleep(std::time::Duration::from_millis(1000));
            println!("Finshed Sleep 2");
        }
        time_it::time_it!{
            std::thread::sleep(std::time::Duration::from_millis(1000));
            println!("Finshed Sleep 3");
        }
    };
}
