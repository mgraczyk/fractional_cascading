# Fractional Cascading in Rust

This is an implementation of a basic form of [fractional
cascading](https://en.wikipedia.org/wiki/Fractional_cascading) in Rust.

Suppose we have `M` sorted lists, each of length `N`.
We want to search for a particular value `k` in all of these lists, and we want to do this
search many times.

The obvious solution is to perform a binary search in each of the `M` lists
(implemented in naive.rs).
This solution requires no additional space and takes time `O(M log N)`

Surprisingly, we can do better!
Fractional cascading allows us to do just one binary search, then do a constant
amount of work to perform the search in each list. This amounts to time `O(log N + M)`
and requires space proportional in size to the input lists `O(M N)`.


The implementation here gives the expected asymptotic speedup:

```
$ cargo test --release -- --nocapture bench_large_naive
...
Took 0.377125236 seconds
...

$ cargo test --release -- --nocapture bench_large_fc
...
Took 0.072147736 seconds
...
```
