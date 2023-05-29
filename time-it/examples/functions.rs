use std::str::FromStr;

#[time_it::time_fn]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    sleep();
    let response = make_post_request()?;
    let num_files: u32 = parse_response(&response)?;
    println!("Database contains {num_files} files");
    Ok(())
}

#[time_it::time_fn]
fn sleep() {
    std::thread::sleep(std::time::Duration::from_secs(1))
}

#[time_it::time_fn]
fn make_post_request() -> Result<String, String> {
    std::thread::sleep(std::time::Duration::from_millis(150));
    Ok(String::from("42"))
}

#[time_it::time_fn]
fn parse_response<'a, T, E>(response: &'a str) -> Result<T, E>
where
    T: FromStr<Err = E>,
{
    std::thread::sleep(std::time::Duration::from_millis(3));
    response.parse()
}
