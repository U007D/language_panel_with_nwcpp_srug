# language_panel_with_nwcpp_srug
## Purpose
* Explain the purpose of locks (concurrency synchronization primitive)
* Demonstrate memory management (managing of allocations)
* Properly handle errors
* Use idiomatic Rust

- `race_global_mut_counter` bypasses (contains UB!) Rust's data race safety guarantees to illustrate the problem.
- `race_allocated_counter` bypasses (contains UB!) Rust's data race safety guarantees to illustrate the problem.
- `lock_allocated_counter` uses locks to eliminate the data race data loss introduced in v2.
- `atomic_allocated_counter` uses `Atomic`s to achieve the same effect with less code to maintain.

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.
