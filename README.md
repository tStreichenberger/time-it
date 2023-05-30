# time-it

## Examples

Examples found in [time-it main lib](https://github.com/tStreichenberger/time-it/tree/main/time-it)

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
[TimeIt] Time elapsed: 1003ms
I still have ownership of Defined in macro!!!
```

### Functions

```
cargo run --example functions
```

```
[sleep] Time elapsed: 1000ms
[make_post_request] Time elapsed: 151ms
[parse_response] Time elapsed: 3ms
Database contains 42 files
[main] Time elapsed: 1155ms
```


## TODO:
* Add task spawning to handle task action
* optionally use tokio for this task
* add optional tag to `#[time_fn]` macro to be called like `#[time_fn("tag")]`