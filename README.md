# time-it

## Example

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


## TODO:
* Add custom message to print