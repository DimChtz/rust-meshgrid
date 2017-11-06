# rust-meshgrid (meshgrid) - v0.1.0
Meshgrid functionality for Rust.

# Examples
#### 2D loop.
```rust
for &(i, j) in meshgrid::new(1..3, 1..5).iter() {

        println!("[{}, {}]", i, j);
        
        // Do-stuff with i, j
}
```

# Installation

Add this line to your Cargo.toml:

```toml
[dependencies]
meshgrid = "0.1.0"
```

and then add this line to your main.rs:

```rust
extern crate meshgrid;
```
