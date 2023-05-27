# time-it

## Examples

Examples found in time-it main lib

### Simple

```
cargo run --example simple
```

```
fn main() {
    time_it::time_it! {
        let x = String::from("Defined in macro");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    };
    println!("I still have ownership of {x}!!!");
}
```
```
Operation took 1003ms
I still have ownership of Defined in macro!!!
```

### Functions

```
cargo run --example functions
```

```
sleep took 1000ms
make_post_request took 155ms
parse_response took 3ms
Database contains 42 files
main took 1159ms
```


## TODO:
* Add custom message to print in `time_it!`