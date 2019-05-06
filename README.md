# daktylos - a simple memory usage counting allocator

This crate gets the excellent [custom wrapper example](https://doc.rust-lang.org/std/alloc/struct.System.html) that keeps track of memory usage.

## Usage

```
use daktylos::Allocator;

#[global_allocator]
static A: Allocator = Allocator::new();

fn main() {
    println!("{}", A.current_usage());
}
```
