/// # Expected Output:
/// ```text
/// Finshed Sleep 1
/// [TimeIt] Time elapsed: 1005ms
/// Finshed Sleep 2
/// [TimeIt] Time elapsed: 1004ms
/// Finshed Sleep 3
/// [TimeIt] Time elapsed: 1000ms
/// [TimeIt] Time elapsed: 3010ms
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
