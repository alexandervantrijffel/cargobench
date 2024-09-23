# Rust benchmark experiment

This is an experiment with criterion.rs to measure the impact on performance when passing a pretty large object as a boxed argument, an unboxed argument, and an unboxed argument with cloning in a deep call stack of 10 nested calls.

Run with command `cargo bench`.

Results on my Macbook Pro M3 Max 16 Core 2023 in a VMware VM running Arch Linux:
