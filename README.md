# daktylos - a simple memory usage counting allocator

This crate gets the excellent [custom wrapper example](https://doc.rust-lang.org/std/alloc/struct.System.html) that keeps track of memory usage, and packages it into a easy-to-use crate.

## Usage

```rust
use daktylos::CountingAllocator;

#[global_allocator]
static A: CountingAllocator = CountingAllocator::new();

fn main() {
    println!("{}", A.current_usage());
}
```
