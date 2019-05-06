use daktylos::CountingAllocator;

#[global_allocator]
static A: CountingAllocator = CountingAllocator::new();

fn main() {
    println!("allocated bytes before main: {}", A.current_usage());
}
