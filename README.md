# Rust TCP

- Multi threaded (without thread pool) TCP Server
- Multiple Producer, Single Consumer chat server

# Usage

1. Clone and cd into this project.

2. In a terminal instance run:

```rust
cargo run
```

3. Open two more terminal instance and run below on both:

```rust
telnet 0.0.0.0 6969
```

4. Start sending messages from the terminals from where you ran the `telnet` command.

![](https://raw.githubusercontent.com/shivajichalise/rust-tcp/refs/heads/main/Rust%20TCP%20demo.gif)
