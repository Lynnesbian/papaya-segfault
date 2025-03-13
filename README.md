papaya-segfault
===
Reproduction for Papaya 0.2.0 [memory corruption bug](https://github.com/ibraheemdev/papaya/issues/63).

## Example Output
```text
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/papaya-segfault`
[src/main.rs:11:17] s = "Hello 34!"
[src/main.rs:11:17] s = "Hello 50!"
[src/main.rs:11:17] s = "Hello 47!"
[src/main.rs:11:17] s = "Segmentation fault (core dumped)
```
