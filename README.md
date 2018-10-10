## Working scenario
```
✗ cargo +1.26.2 build --features feature                   
   Compiling cargo_check v0.1.0 (file:///Users/skletsun/projects/cargo_check)
    Finished dev [unoptimized + debuginfo] target(s) in 0.76 secs
✗ cargo +1.26.2 test --all -- --nocapture 
   Compiling cargo_check v0.1.0 (file:///Users/skletsun/projects/cargo_check)
   Compiling app v0.1.0 (file:///Users/skletsun/projects/cargo_check/app)
    Finished dev [unoptimized + debuginfo] target(s) in 0.75 secs
     Running app-7831ba7dae82efbc

running 1 test
--- App with no feature, library: no feature ---
test test_app ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```
## Doesn't work
```
✗ cargo +1.28.0 build --features feature
   Compiling cargo_check v0.1.0 (file:///Users/skletsun/projects/cargo_check)
    Finished dev [unoptimized + debuginfo] target(s) in 1.16s
✗ cargo +1.28.0 test --all -- --nocapture 
   Compiling cargo_check v0.1.0 (file:///Users/skletsun/projects/cargo_check)
   Compiling app v0.1.0 (file:///Users/skletsun/projects/cargo_check/app)
    Finished dev [unoptimized + debuginfo] target(s) in 1.39s
     Running target/debug/deps/app-a0d72a045bf384d9

running 1 test
--- App with no feature, library: feature ---
thread 'test_app' panicked at 'assertion failed: `(left == right)`
  left: `"no feature"`,
 right: `"feature"`', app/src/main.rs:20:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
test test_app ... FAILED
```
