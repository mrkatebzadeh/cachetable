# CacheTable

`cachetable` is a high-performance lossy hash table designed for low-latency workloads.

## Usage

To include `cachetable` in your project, add it to your `Cargo.toml` from the GitHub repository:

``` toml
[dependencies]
cachetable = { git = "https://github.com/mrkatebzadeh/cachetable.git" }
```

## Example

Here’s a simple example demonstrating how to use the `cachetable` crate for a key-value storage:

``` rust
use cachetable::CacheTable;

fn main() {
    let key = 10;
    let value = vec![10];
    let ctable = CacheTable::<u32, Vec<u32>, 4 /* Log size */, 32 /* Bucket size*/>::new();
    ctable.insert(key, value);
    let value = ctable.get(&key);
}
```

## TODO
- [x] Implement retrieval of a key-value pair from the cache table.
- [ ] Implement lock-free concurent access.
- [ ] Implement different policies for eviction.
